//! ECharts WASM 实例

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use rust_zrender::{STATE_EMPHASIS, STATE_NORMAL, STATE_SELECT, ZRenderer};

use crate::interaction::{DataTarget, InteractionState};
use crate::model::GlobalModel;
use crate::option::{parse_option_value, OptionModel, OptionValue};
use crate::scheduler::run_update;
use crate::visual::VisualContext;

#[wasm_bindgen]
pub struct EChartsInstance {
    zr: ZRenderer,
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
        Ok(EChartsInstance {
            zr,
            option: OptionModel::new(),
            width,
            height,
            dpr,
            interaction: InteractionState::default(),
        })
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

    pub fn set_option(&mut self, option: JsValue) -> Result<(), JsValue> {
        self.option.set_option(&option)?;
        self.interaction = InteractionState::from_option(&self.option);
        self.render_and_apply_states();
        Ok(())
    }

    pub fn has_option(&self) -> bool {
        !self.option.is_empty()
    }

    pub fn option_has_functions(&self) -> bool {
        option_contains_function(self.option.root())
    }

    pub fn resize(&mut self, width: u32, height: u32, dpr: f64) -> Result<(), JsValue> {
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        self.zr
            .resize_with_dpr(width, height, dpr)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        if !self.option.is_empty() {
            self.render_and_apply_states();
        }
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<Vec<u8>, JsValue> {
        self.zr
            .refresh()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn find_hover(&mut self, x: f64, y: f64) -> JsValue {
        match self.zr.find_hover(x, y) {
            Some(hit) => hit_to_js(&hit),
            None => JsValue::NULL,
        }
    }

    /// 阶段 6：pointer move 统一处理 hover 高亮、axisPointer、tooltip
    pub fn handle_pointer_move(&mut self, x: f64, y: f64) -> JsValue {
        self.interaction.set_pointer(Some(x), Some(y));
        let hit = self.zr.find_hover(x, y);
        let hover_target = hit.as_ref().and_then(|h| {
            let si = h.ec_data.series_index?;
            let di = h.ec_data.data_index?;
            Some(DataTarget {
                series_index: si,
                data_index: di,
            })
        });
        self.interaction.set_hover(hover_target);
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
        } else {
            let _ = Reflect::set(&obj, &JsValue::from_str("tooltip"), &JsValue::NULL);
        }

        let model = GlobalModel::from_option_with_zoom(
            &self.option,
            self.width,
            self.height,
            self.interaction.data_zoom,
        );
        if let Some((cat_idx, label, snap_x)) =
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
        self.render_and_apply_states();
        Ok(())
    }

    /// 滚轮 dataZoom（option 含 dataZoom 时生效）
    pub fn apply_data_zoom_wheel(&mut self, x: f64, delta_y: f64) -> Result<(), JsValue> {
        if !self.interaction.data_zoom_enabled {
            return Ok(());
        }
        let model = GlobalModel::from_option(&self.option, self.width, self.height);
        let grid = model.grid;
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
        let model = GlobalModel::from_option_with_zoom(
            &self.option,
            self.width,
            self.height,
            self.interaction.data_zoom,
        );
        let si = series_index as usize;
        let di = data_index as usize;
        if si >= model.series.len() || di >= model.series[si].data.len() {
            return JsValue::NULL;
        }
        let visual = VisualContext::new(&self.option, &model);
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
    }

    /// 阶段 7：基准测试 setOption 管线 + refresh 平均耗时（毫秒）
    pub fn benchmark_render(&mut self, iterations: u32) -> f64 {
        if self.option.is_empty() || iterations == 0 {
            return 0.0;
        }
        let start = js_sys::Date::now();
        for _ in 0..iterations {
            self.render_and_apply_states();
            let _ = self.zr.refresh();
        }
        (js_sys::Date::now() - start) / iterations as f64
    }
}

impl EChartsInstance {
    fn render_and_apply_states(&mut self) {
        if self.option.is_empty() {
            return;
        }
        run_update(
            &mut self.zr,
            &self.option,
            self.width,
            self.height,
            &self.interaction,
        );
        self.apply_interaction_states();
    }

    fn apply_interaction_states(&mut self) {
        for i in 0..self.zr.storage.paths().len() {
            self.zr.set_path_state(i, STATE_NORMAL);
        }

        let selected: Vec<DataTarget> = self.interaction.selected.iter().copied().collect();
        for target in selected {
            self.apply_state_to_target(target, STATE_SELECT);
        }

        if let Some(target) = self.interaction.hover {
            self.apply_state_to_target(target, STATE_EMPHASIS);
        }
    }

    fn apply_state_to_target(&mut self, target: DataTarget, state: &str) {
        for i in 0..self.zr.storage.paths().len() {
            let ec = &self.zr.storage.path(i).ec_data;
            if ec.series_index == Some(target.series_index)
                && ec.data_index == Some(target.data_index)
            {
                self.zr.set_path_state(i, state);
            }
        }
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
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("pathIndex"),
        &JsValue::from(hit.path_index as u32),
    );
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
