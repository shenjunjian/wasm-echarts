//! ECharts WASM 实例

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use rust_zrender::{STATE_EMPHASIS, STATE_NORMAL, STATE_SELECT, ZRenderer};

use crate::coord::{convert_from_pixel, convert_to_pixel, ConvertResult};
use crate::interaction::{DataTarget, InteractionState};
use crate::model::GlobalModel;
use crate::option::{parse_option_value, OptionModel, OptionValue, SetOptionFlags};
use crate::scheduler::run_update;
use crate::visual::VisualContext;

#[wasm_bindgen]
pub struct EChartsInstance {
    zr_id: u32,
    option: OptionModel,
    width: u32,
    height: u32,
    dpr: f64,
    interaction: InteractionState,
}

#[wasm_bindgen]
impl EChartsInstance {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, dpr: f64) -> Result<EChartsInstance, JsValue> {
        crate::utils::set_panic_hook();
        let zr = ZRenderer::new_with_dpr(width, height, dpr)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let zr_id = wasm_zrender::insert_renderer(zr);
        Ok(EChartsInstance {
            zr_id,
            option: OptionModel::new(),
            width,
            height,
            dpr,
            interaction: InteractionState::default(),
        })
    }

    #[wasm_bindgen(js_name = getZr)]
    pub fn get_zr(&self) -> wasm_zrender::ZRender {
        wasm_zrender::ZRender::from_id(self.zr_id)
    }

    #[wasm_bindgen(js_name = attachHost)]
    pub fn attach_host(&self, dom: JsValue) -> Result<(), JsValue> {
        wasm_zrender::attach_host(self.zr_id, &dom)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dpr(&self) -> f64 {
        self.dpr
    }

    /// `opts` 为官方第二参数：`boolean` 或 `{ notMerge, replaceMerge }`。可省略。
    pub fn set_option(&mut self, option: JsValue, opts: Option<JsValue>) -> Result<(), JsValue> {
        let flags = parse_set_option_flags(opts.as_ref())?;
        let not_merge = flags.not_merge;
        self.option.set_option(&option, flags)?;
        self.interaction.absorb_user_option(&self.option, not_merge);
        self.render_and_apply_states();
        Ok(())
    }

    pub fn get_option(&self) -> Result<JsValue, JsValue> {
        self.option.to_js()
    }

    pub fn has_option(&self) -> bool {
        !self.option.is_empty()
    }

    pub fn option_has_functions(&self) -> bool {
        option_contains_function(self.option.root())
    }

    /// `convertToPixel(finder, value)`：cartesian `xAxis` / `yAxis` / `grid` / `seriesIndex`，含 time / log。
    pub fn convert_to_pixel(&self, finder: JsValue, value: JsValue) -> JsValue {
        convert_pixel_js(&self.option, self.width, self.height, self.interaction.data_zoom, finder, value, true)
    }

    /// `convertFromPixel(finder, value)`：与 `convertToPixel` 同一 finder 最小集。
    pub fn convert_from_pixel(&self, finder: JsValue, value: JsValue) -> JsValue {
        convert_pixel_js(&self.option, self.width, self.height, self.interaction.data_zoom, finder, value, false)
    }

    pub fn resize(&mut self, width: u32, height: u32, dpr: f64) -> Result<(), JsValue> {
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        wasm_zrender::with_zr(self.zr_id, |zr| {
            zr.resize_with_dpr(width, height, dpr)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        })?;
        if !self.option.is_empty() {
            self.render_and_apply_states();
        }
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<Vec<u8>, JsValue> {
        wasm_zrender::with_zr(self.zr_id, |zr| {
            zr.refresh()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        })
    }

    /// 把全局 fontdb 同步到本实例（`registerFont` 之后由 facade 调用）。
    pub fn update_font_database(&mut self) -> Result<(), JsValue> {
        wasm_zrender::with_zr(self.zr_id, |zr| {
            rust_zrender::with_resolved_font_config(|resolved| {
                zr.update_font_database(resolved);
            })
            .map_err(|e| JsValue::from_str(&e.to_string()))
        })
    }

    pub fn find_hover(&mut self, x: f64, y: f64) -> JsValue {
        wasm_zrender::with_zr(self.zr_id, |zr| {
            Ok(match zr.find_hover(x, y) {
                Some(hit) => hit_to_js(&hit),
                None => JsValue::NULL,
            })
        })
        .unwrap_or(JsValue::NULL)
    }

    /// 阶段 6：pointer move 统一处理 hover 高亮、axisPointer、tooltip
    pub fn handle_pointer_move(&mut self, x: f64, y: f64) -> JsValue {
        self.interaction.set_pointer(Some(x), Some(y));
        let hit = wasm_zrender::with_zr(self.zr_id, |zr| Ok(zr.find_hover(x, y)))
            .ok()
            .flatten();
        let hover_target = hit.as_ref().and_then(|h| {
            if h.ec_data.data_type.is_some() && h.ec_data.series_index.is_none() {
                return None;
            }
            let si = h.ec_data.series_index?;
            let di = h.ec_data.data_index?;
            Some(DataTarget {
                series_index: si,
                data_index: di,
            })
        });
        if self.interaction.drag.is_some() {
            self.apply_pointer_drag(x, y);
        } else {
            self.interaction.set_hover(hover_target);
        }
        self.render_and_apply_states();

        let obj = Object::new();
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("hit"),
            &hit.as_ref().map(hit_to_js).unwrap_or(JsValue::NULL),
        );

        if let Some((si, di)) = hover_target.map(|t| (t.series_index, t.data_index)) {
            let tip = self.get_tooltip_content(si, di);
            let _ = Reflect::set(&obj, &JsValue::from_str("tooltip"), &tip);
        } else if self.interaction.tooltip_trigger_axis {
            let tip = self.get_axis_tooltip(x, y);
            let _ = Reflect::set(&obj, &JsValue::from_str("tooltip"), &tip);
        } else {
            let _ = Reflect::set(&obj, &JsValue::from_str("tooltip"), &JsValue::NULL);
        }

        let model = self.current_model();
        if let Some((cat_idx, label, snap_x, _)) =
            self.interaction.axis_pointer_label(&model, x, y)
        {
            let ap = Object::new();
            let _ = Reflect::set(
                &ap,
                &JsValue::from_str("categoryIndex"),
                &JsValue::from(cat_idx as u32),
            );
            let _ = Reflect::set(
                &ap,
                &JsValue::from_str("label"),
                &JsValue::from_str(&label),
            );
            let _ = Reflect::set(
                &ap,
                &JsValue::from_str("snapX"),
                &JsValue::from(snap_x),
            );
            let _ = Reflect::set(&obj, &JsValue::from_str("axisPointer"), &ap);
        } else {
            let _ = Reflect::set(&obj, &JsValue::from_str("axisPointer"), &JsValue::NULL);
        }

        obj.into()
    }

    pub fn handle_pointer_leave(&mut self) -> Result<(), JsValue> {
        self.interaction.set_hover(None);
        self.interaction.set_pointer(None, None);
        self.interaction.end_drag();
        self.render_and_apply_states();
        Ok(())
    }

    #[wasm_bindgen(js_name = handlePointerDown)]
    pub fn handle_pointer_down(&mut self, x: f64, y: f64) -> JsValue {
        self.interaction.set_pointer(Some(x), Some(y));
        let hit = wasm_zrender::with_zr(self.zr_id, |zr| Ok(zr.find_hover(x, y)))
            .ok()
            .flatten();
        let data_type = hit.as_ref().and_then(|h| h.ec_data.data_type.clone());
        let data_index = hit.as_ref().and_then(|h| h.ec_data.data_index).unwrap_or(-1);
        if let Some(dt) = data_type.as_deref() {
            if dt == crate::chart::HIT_DATA_ZOOM {
                if let Some(kind) = crate::chart::data_zoom::hit_slider_kind(data_index) {
                    self.interaction.begin_drag(kind, x, y);
                }
            } else if dt == crate::chart::HIT_THUMBNAIL {
                self.interaction
                    .begin_drag(crate::interaction::DragKind::Thumbnail, x, y);
            }
        } else {
            let model = self.current_model();
            let in_grid = model.grid().contains(x, y);
            if in_grid && (self.interaction.toolbox_zoom_select || crate::chart::brush::brush_enabled(&self.effective_option())) {
                let kind = if self.interaction.toolbox_zoom_select {
                    crate::interaction::DragKind::ToolboxZoom
                } else {
                    crate::interaction::DragKind::Brush
                };
                self.interaction.begin_drag(kind, x, y);
                self.interaction.brush_rect = Some((x, y, x, y));
            }
        }
        self.render_and_apply_states();
        hit.as_ref().map(hit_to_js).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(js_name = handlePointerUp)]
    pub fn handle_pointer_up(&mut self, x: f64, y: f64) -> Result<(), JsValue> {
        if let Some(drag) = self.interaction.drag {
            match drag.kind {
                crate::interaction::DragKind::Brush => {
                    if let Some(rect) = self.interaction.brush_rect {
                        let model = self.current_model();
                        let targets = crate::chart::brush::points_in_brush(&model, rect);
                        self.interaction.clear_select();
                        for t in targets {
                            self.interaction.select(t);
                        }
                    }
                }
                crate::interaction::DragKind::ToolboxZoom => {
                    if let Some((x0, _, x1, _)) = self.interaction.brush_rect {
                        let model = self.current_model();
                        let g = model.grid();
                        if g.width > 0.0 {
                            let s = ((x0.min(x1) - g.x) / g.width * 100.0).clamp(0.0, 100.0);
                            let e = ((x0.max(x1) - g.x) / g.width * 100.0).clamp(0.0, 100.0);
                            if e - s > 1.0 {
                                self.interaction.set_data_zoom_range(s, e);
                            }
                        }
                        self.interaction.toolbox_zoom_select = false;
                        self.interaction.brush_rect = None;
                    }
                }
                _ => {}
            }
        }
        self.interaction.end_drag();
        let _ = (x, y);
        self.render_and_apply_states();
        Ok(())
    }

    #[wasm_bindgen(js_name = handlePointerClick)]
    pub fn handle_pointer_click(&mut self, x: f64, y: f64) -> JsValue {
        let hit = wasm_zrender::with_zr(self.zr_id, |zr| Ok(zr.find_hover(x, y)))
            .ok()
            .flatten();
        let data_type = hit.as_ref().and_then(|h| h.ec_data.data_type.clone());
        let data_index = hit.as_ref().and_then(|h| h.ec_data.data_index).unwrap_or(-1);
        match data_type.as_deref() {
            Some(crate::chart::HIT_LEGEND) => {
                let effective = self.effective_option();
                if let Some(legend) = crate::chart::text_opt::option_component(effective.root(), "legend")
                {
                    let model = self.current_model();
                    if let Some(name) = crate::chart::legend::legend_item_name(legend, &model, data_index as usize)
                    {
                        let on = self.interaction.toggle_legend(&name);
                        self.option.set_legend_selected(&name, on);
                        self.render_and_apply_states();
                    }
                }
            }
            Some(crate::chart::HIT_TOOLBOX) => {
                self.handle_toolbox(data_index);
            }
            Some(crate::chart::HIT_TIMELINE) => {
                if data_index >= 0 {
                    self.interaction.timeline_index = data_index as usize;
                    self.option.set_timeline_index(self.interaction.timeline_index);
                    self.render_and_apply_states();
                }
            }
            Some(crate::chart::HIT_VISUAL_MAP) => {
                if let Some((vi, piece)) =
                    crate::chart::visual_map::piecewise_hit(&self.effective_option(), data_index)
                {
                    let on = self.interaction.toggle_visual_map_piece(vi * 100 + piece);
                    self.option.set_visual_map_selected(vi, piece, on);
                    self.render_and_apply_states();
                }
            }
            _ => {}
        }
        hit.as_ref().map(hit_to_js).unwrap_or(JsValue::NULL)
    }

    /// 滚轮 dataZoom（option 含 dataZoom 时生效）
    pub fn apply_data_zoom_wheel(&mut self, x: f64, delta_y: f64) -> Result<(), JsValue> {
        if !self.interaction.data_zoom_has_inside {
            return Ok(());
        }
        let model = self.current_model();
        let grid = match self.interaction.data_zoom_x_axis_index {
            Some(idx) => {
                let gi = model.x_axis_at(idx).grid_index;
                model.grid_at(gi)
            }
            None => model.grid(),
        };
        let anchor = if grid.width > 0.0 {
            ((x - grid.x) / grid.width).clamp(0.0, 1.0)
        } else {
            0.5
        };
        self.interaction.data_zoom.zoom_wheel(delta_y, anchor);
        self.render_and_apply_states();
        Ok(())
    }

    /// hover 时调用 tooltip.formatter，返回 string 或 null
    pub fn get_tooltip_content(&self, series_index: i32, data_index: i32) -> JsValue {
        if series_index < 0 || data_index < 0 {
            return JsValue::NULL;
        }
        let model = self.current_model();
        let si = series_index as usize;
        let di = data_index as usize;
        if si >= model.series.len() || di >= model.series[si].data.len() {
            return JsValue::NULL;
        }
        let effective = self.effective_option();
        let visual = VisualContext::new(&effective, &model);
        match visual.resolve_tooltip(si, di) {
            Some(text) => JsValue::from_str(&text),
            None => JsValue::NULL,
        }
    }

    pub fn dispatch_action(&mut self, action: JsValue) -> Result<(), JsValue> {
        let parsed = parse_option_value(&action)?;
        let action_type = parsed
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsValue::from_str("dispatchAction requires type"))?;

        match action_type {
            "highlight" => {
                if let Some(target) = parse_data_target(&parsed) {
                    self.interaction.set_hover(Some(target));
                    self.render_and_apply_states();
                }
            }
            "downplay" => {
                self.interaction.set_hover(None);
                self.render_and_apply_states();
            }
            "select" => {
                if let Some(target) = parse_data_target(&parsed) {
                    self.interaction.select(target);
                    self.render_and_apply_states();
                }
            }
            "unselect" => {
                if let Some(target) = parse_data_target(&parsed) {
                    self.interaction.unselect(target);
                    self.render_and_apply_states();
                }
            }
            "toggleSelect" => {
                if let Some(target) = parse_data_target(&parsed) {
                    self.interaction.toggle_select(target);
                    self.render_and_apply_states();
                }
            }
            "dataZoom" => {
                let start = parsed.get("start").and_then(|v| v.as_f64());
                let end = parsed.get("end").and_then(|v| v.as_f64());
                if let (Some(s), Some(e)) = (start, end) {
                    self.interaction.set_data_zoom_range(s, e);
                    self.render_and_apply_states();
                }
            }
            "showTip" => {
                if let Some(target) = parse_data_target(&parsed) {
                    self.interaction.set_hover(Some(target));
                    self.render_and_apply_states();
                } else if let (Some(x), Some(y)) = (
                    parsed.get("x").and_then(|v| v.as_f64()),
                    parsed.get("y").and_then(|v| v.as_f64()),
                ) {
                    let _ = self.handle_pointer_move(x, y);
                }
            }
            "hideTip" => {
                // tooltip DOM 由 JS facade 关闭；不改 hover（与官方 hideTip 一致）
            }
            "legendToggleSelect" => {
                if let Some(name) = parsed.get("name").and_then(|v| v.as_str()) {
                    let on = self.interaction.toggle_legend(name);
                    self.option.set_legend_selected(name, on);
                    self.render_and_apply_states();
                }
            }
            "legendSelect" => {
                if let Some(name) = parsed.get("name").and_then(|v| v.as_str()) {
                    self.interaction.set_legend(name, true);
                    self.option.set_legend_selected(name, true);
                    self.render_and_apply_states();
                }
            }
            "legendUnSelect" => {
                if let Some(name) = parsed.get("name").and_then(|v| v.as_str()) {
                    self.interaction.set_legend(name, false);
                    self.option.set_legend_selected(name, false);
                    self.render_and_apply_states();
                }
            }
            "restore" => {
                if let Some(snap) = self.interaction.restore_snapshot.clone() {
                    self.option.apply(
                        snap,
                        crate::option::SetOptionFlags {
                            not_merge: true,
                            replace_merge: vec![],
                        },
                    );
                    self.interaction.absorb_user_option(&self.option, true);
                    self.render_and_apply_states();
                }
            }
            "timelineChange" | "timelinePlayChange" => {
                if let Some(idx) = parsed
                    .get("currentIndex")
                    .and_then(|v| v.as_f64())
                    .map(|n| n.max(0.0) as usize)
                {
                    self.interaction.timeline_index = idx;
                    self.option.set_timeline_index(idx);
                    self.render_and_apply_states();
                }
            }
            "takeGlobalCursor" => {
                let key = parsed
                    .get("key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                self.interaction.toolbox_zoom_select = key == "dataZoomSelect";
            }
            "brush" | "brushEnd" => {
                // 选区由指针拖拽写入；此处接受 payload.areas 最小集
                if let Some(areas) = parsed.get("areas").and_then(|v| v.as_array()) {
                    if let Some(range) = areas.first().and_then(|a| a.get("range")).and_then(|v| v.as_array()) {
                        if range.len() >= 4 {
                            let r: Vec<f64> = range.iter().filter_map(|v| v.as_f64()).collect();
                            if r.len() >= 4 {
                                self.interaction.brush_rect = Some((r[0], r[1], r[2], r[3]));
                                let model = self.current_model();
                                let targets = crate::chart::brush::points_in_brush(&model, (r[0], r[1], r[2], r[3]));
                                self.interaction.clear_select();
                                for t in targets {
                                    self.interaction.select(t);
                                }
                                self.render_and_apply_states();
                            }
                        }
                    }
                }
            }
            other => {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "dispatchAction type '{other}' not implemented yet"
                )));
            }
        }
        Ok(())
    }

    pub fn dispose(&mut self) {
        self.option.clear();
        self.interaction = InteractionState::default();
        wasm_zrender::dispose_renderer(self.zr_id);
    }

    #[wasm_bindgen(js_name = appendData)]
    pub fn append_data(&mut self, params: JsValue) -> Result<(), JsValue> {
        let parsed = parse_option_value(&params)?;
        let series_index = parsed
            .get("seriesIndex")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as usize;
        let extra = parsed
            .get("data")
            .cloned()
            .ok_or_else(|| JsValue::from_str("appendData requires data"))?;
        self.option.append_series_data(series_index, extra)?;
        self.render_and_apply_states();
        Ok(())
    }

    #[wasm_bindgen(js_name = containPixel)]
    pub fn contain_pixel(&self, finder: JsValue, value: JsValue) -> bool {
        if self.option.is_empty() {
            return false;
        }
        let Ok(value) = parse_option_value(&value) else {
            return false;
        };
        let (x, y) = match value.as_array() {
            Some(arr) if arr.len() >= 2 => (
                arr[0].as_f64().unwrap_or(0.0),
                arr[1].as_f64().unwrap_or(0.0),
            ),
            _ => return false,
        };
        let Ok(finder) = parse_option_value(&finder) else {
            return false;
        };
        let model = self.current_model();
        crate::coord::contain_pixel(&model, &finder, x, y)
    }

    /// 阶段 7：基准测试 setOption 管线 + refresh 平均耗时（毫秒）
    pub fn benchmark_render(&mut self, iterations: u32) -> f64 {
        if self.option.is_empty() || iterations == 0 {
            return 0.0;
        }
        let start = js_sys::Date::now();
        for _ in 0..iterations {
            self.render_and_apply_states();
            let _ = wasm_zrender::with_zr(self.zr_id, |zr| {
                zr.refresh()
                    .map_err(|e| JsValue::from_str(&e.to_string()))
            });
        }
        (js_sys::Date::now() - start) / iterations as f64
    }
}

impl EChartsInstance {
    fn effective_option(&self) -> OptionModel {
        OptionModel::with_root(self.option.effective_root(self.interaction.timeline_index))
    }

    fn current_model(&self) -> GlobalModel {
        GlobalModel::from_option_with_zoom(
            &self.effective_option(),
            self.width,
            self.height,
            self.interaction.data_zoom,
        )
    }

    fn apply_pointer_drag(&mut self, x: f64, y: f64) {
        let Some(drag) = self.interaction.drag else {
            return;
        };
        match drag.kind {
            crate::interaction::DragKind::ZoomStart
            | crate::interaction::DragKind::ZoomEnd
            | crate::interaction::DragKind::ZoomFiller => {
                let model = self.current_model();
                if let Some(geom) = crate::chart::data_zoom::slider_geom(&model, &self.effective_option()) {
                    crate::chart::data_zoom::apply_slider_drag(&mut self.interaction, geom, x);
                }
            }
            crate::interaction::DragKind::Thumbnail => {
                let model = self.current_model();
                if let Some(geom) = crate::chart::thumbnail::thumb_geom(&model, &self.effective_option()) {
                    crate::chart::thumbnail::apply_thumbnail_drag(&mut self.interaction, geom, x);
                }
            }
            crate::interaction::DragKind::Brush | crate::interaction::DragKind::ToolboxZoom => {
                if let Some((x0, y0, _, _)) = self.interaction.brush_rect {
                    self.interaction.brush_rect = Some((x0, y0, x, y));
                }
            }
        }
    }

    fn handle_toolbox(&mut self, id: i32) {
        match id {
            crate::chart::TB_RESTORE => {
                if let Some(snap) = self.interaction.restore_snapshot.clone() {
                    self.option.apply(
                        snap,
                        crate::option::SetOptionFlags {
                            not_merge: true,
                            replace_merge: vec![],
                        },
                    );
                    self.interaction.absorb_user_option(&self.option, true);
                    self.render_and_apply_states();
                }
            }
            crate::chart::TB_MAGIC => {
                let next = crate::chart::next_magic_type(&self.effective_option());
                self.option.set_cartesian_series_type(&next);
                self.render_and_apply_states();
            }
            crate::chart::TB_DATA_ZOOM => {
                self.interaction.toolbox_zoom_select = !self.interaction.toolbox_zoom_select;
            }
            crate::chart::TB_DATA_VIEW => {
                web_sys::console::warn_1(&JsValue::from_str(
                    "toolbox dataView DOM overlay is not implemented",
                ));
            }
            crate::chart::TB_SAVE => {
                web_sys::console::warn_1(&JsValue::from_str(
                    "toolbox saveAsImage DOM download bar is not implemented; use getDataURL",
                ));
            }
            crate::chart::TB_BRUSH => {
                self.interaction.toolbox_zoom_select = false;
                if self.interaction.brush_rect.is_some() {
                    self.interaction.brush_rect = None;
                    self.interaction.clear_select();
                    self.render_and_apply_states();
                }
            }
            _ => {}
        }
    }

    fn get_axis_tooltip(&self, x: f64, y: f64) -> JsValue {
        let model = self.current_model();
        let Some((cat_idx, label, _, _)) = self.interaction.axis_pointer_label(&model, x, y) else {
            return JsValue::NULL;
        };
        let effective = self.effective_option();
        let visual = VisualContext::new(&effective, &model);
        let formatter = effective
            .root()
            .get("tooltip")
            .and_then(|t| t.get("formatter"))
            .cloned();
        let mut lines = vec![label.clone()];
        let mut first_params = None;
        let arr = js_sys::Array::new();
        for series in &model.series {
            if !self.interaction.is_name_selected(&series.name) {
                continue;
            }
            if cat_idx >= series.data.len() {
                continue;
            }
            let params = visual.data_params(series.index, cat_idx);
            if first_params.is_none() {
                first_params = Some(params.clone());
            }
            arr.push(&params);
            let p = &series.data[cat_idx];
            lines.push(format!("{}: {}", series.name, p.value));
        }
        if let Some(fmt) = formatter.as_ref() {
            if let Some(params) = first_params.as_ref() {
                if let Some(text) = crate::bridge::resolve_formatter(Some(fmt), params) {
                    return JsValue::from_str(&text);
                }
            }
            if let crate::option::OptionValue::Function(f) = fmt {
                if let Ok(ret) = f.call1(&JsValue::NULL, &arr) {
                    if let Some(s) = ret.as_string() {
                        return JsValue::from_str(&s);
                    }
                }
            }
        }
        if lines.len() <= 1 {
            return JsValue::NULL;
        }
        JsValue::from_str(&lines.join("\n"))
    }

    fn render_and_apply_states(&mut self) {
        if self.option.is_empty() {
            return;
        }
        let zr_id = self.zr_id;
        let width = self.width;
        let height = self.height;
        let _ = wasm_zrender::with_zr(zr_id, |zr| {
            run_update(zr, zr_id, &self.option, width, height, &self.interaction);
            Ok(())
        });
        self.apply_interaction_states();
    }

    fn apply_interaction_states(&mut self) {
        let selected: Vec<DataTarget> = self.interaction.selected.iter().copied().collect();
        let hover = self.interaction.hover;
        let _ = wasm_zrender::with_zr(self.zr_id, |zr| {
            for i in 0..zr.storage.paths().len() {
                zr.set_path_state(i, STATE_NORMAL);
            }
            for target in &selected {
                apply_state_to_zr(zr, *target, STATE_SELECT);
            }
            if let Some(target) = hover {
                apply_state_to_zr(zr, target, STATE_EMPHASIS);
            }
            Ok(())
        });
    }
}

fn apply_state_to_zr(zr: &mut ZRenderer, target: DataTarget, state: &str) {
    for i in 0..zr.storage.paths().len() {
        let ec = &zr.storage.path(i).ec_data;
        if ec.series_index == Some(target.series_index)
            && ec.data_index == Some(target.data_index)
        {
            zr.set_path_state(i, state);
        }
    }
}

fn parse_set_option_flags(opts: Option<&JsValue>) -> Result<SetOptionFlags, JsValue> {
    let Some(opts) = opts.filter(|v| !v.is_null() && !v.is_undefined()) else {
        return Ok(SetOptionFlags::default());
    };
    if let Some(not_merge) = opts.as_bool() {
        return Ok(SetOptionFlags {
            not_merge,
            ..Default::default()
        });
    }
    let parsed = parse_option_value(opts)?;
    Ok(SetOptionFlags {
        not_merge: parsed
            .get("notMerge")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        replace_merge: parse_replace_merge(parsed.get("replaceMerge")),
    })
}

fn parse_replace_merge(value: Option<&OptionValue>) -> Vec<String> {
    match value {
        Some(OptionValue::String(s)) => vec![s.clone()],
        Some(OptionValue::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

fn parse_data_target(parsed: &OptionValue) -> Option<DataTarget> {
    let series_index = parsed
        .get("seriesIndex")
        .and_then(|v| v.as_f64())
        .map(|n| n as i32)?;
    let data_index = parsed
        .get("dataIndex")
        .and_then(|v| v.as_f64())
        .map(|n| n as i32)?;
    Some(DataTarget {
        series_index,
        data_index,
    })
}

fn hit_to_js(hit: &rust_zrender::HitResult) -> JsValue {
    let obj = Object::new();
    let _ = Reflect::set(&obj, &JsValue::from_str("x"), &JsValue::from(hit.x));
    let _ = Reflect::set(&obj, &JsValue::from_str("y"), &JsValue::from(hit.y));
    if let rust_zrender::HitTarget::Path(path_index) = hit.target {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("pathIndex"),
            &JsValue::from(path_index as u32),
        );
    }
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("silent"),
        &JsValue::from(hit.silent),
    );
    if let Some(si) = hit.ec_data.series_index {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("seriesIndex"),
            &JsValue::from(si),
        );
    }
    if let Some(di) = hit.ec_data.data_index {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("dataIndex"),
            &JsValue::from(di),
        );
    }
    if let Some(ref dt) = hit.ec_data.data_type {
        let _ = Reflect::set(
            &obj,
            &JsValue::from_str("dataType"),
            &JsValue::from_str(dt),
        );
    }
    obj.into()
}

fn option_contains_function(value: &OptionValue) -> bool {
    match value {
        OptionValue::Function(_) => true,
        OptionValue::Array(arr) => arr.iter().any(option_contains_function),
        OptionValue::Object(map) => map.values().any(option_contains_function),
        _ => false,
    }
}

fn convert_result_to_js(result: ConvertResult) -> JsValue {
    match result {
        ConvertResult::Scalar(n) => JsValue::from(n),
        ConvertResult::Point(x, y) => {
            let arr = js_sys::Array::new_with_length(2);
            arr.set(0, JsValue::from(x));
            arr.set(1, JsValue::from(y));
            arr.into()
        }
    }
}

fn convert_pixel_js(
    option: &OptionModel,
    width: u32,
    height: u32,
    zoom: crate::interaction::DataZoomRange,
    finder: JsValue,
    value: JsValue,
    to_pixel: bool,
) -> JsValue {
    if option.is_empty() {
        return JsValue::NULL;
    }
    let Ok(finder) = parse_option_value(&finder) else {
        return JsValue::NULL;
    };
    let Ok(value) = parse_option_value(&value) else {
        return JsValue::NULL;
    };
    let model = GlobalModel::from_option_with_zoom(option, width, height, zoom);
    let result = if to_pixel {
        convert_to_pixel(&model, &finder, &value)
    } else {
        convert_from_pixel(&model, &finder, &value)
    };
    result.map(convert_result_to_js).unwrap_or(JsValue::NULL)
}
