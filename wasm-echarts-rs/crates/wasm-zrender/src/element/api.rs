//! 图元公共 API：attr / setShape / setStyle / position / setClipPath / getBoundingRect / animate / on

use js_sys::{Object, Reflect};
use rust_zrender::core::bbox::BoundingRect as InnerRect;
use rust_zrender::{Path as RustPath, PathStyle, Text as RustText};
use wasm_bindgen::prelude::*;

use crate::animation::{new_animator, Animator};
use crate::bridge::opts::{
    get_object, get_string, parse_path_style_patch, parse_text_style, parse_xy_pair, xy_pair_to_js,
};
use crate::bridge::shape::merge_shape;
use crate::element::js::element_id_from_js;
use crate::element::pending::PendingData;
use crate::registry::{with_zr, ElementKind, ELEMENT_REGISTRY};

pub fn element_animate(_id: u32, _path: JsValue, _looping: JsValue) -> Animator {
    new_animator()
}

pub fn element_on(_id: u32, _event: &str, _handler: JsValue) {}

pub fn element_position_js(id: u32) -> JsValue {
    ELEMENT_REGISTRY.with(|reg| {
        let reg = reg.borrow();
        if let Some(pending) = reg.pending(id) {
            let (x, y) = pending.position();
            xy_pair_to_js(x, y)
        } else {
            xy_pair_to_js(0.0, 0.0)
        }
    })
}

pub fn element_set_position(id: u32, value: &JsValue) -> Result<(), JsValue> {
    let Some((x, y)) = parse_xy_pair(value) else {
        return Ok(());
    };
    apply_position(id, x, y)
}

pub fn element_attr(id: u32, key_or_obj: JsValue, value: JsValue) -> Result<(), JsValue> {
    if let Some(key) = key_or_obj.as_string() {
        return apply_attr_key(id, &key, &value);
    }
    if key_or_obj.is_object() && !key_or_obj.is_null() {
        let obj = Object::from(key_or_obj);
        let keys = Object::keys(&obj);
        for i in 0..keys.length() {
            let key_js = keys.get(i);
            let key = key_js.as_string().unwrap_or_default();
            let val = Reflect::get(&obj, &key_js).unwrap_or(JsValue::UNDEFINED);
            apply_attr_key(id, &key, &val)?;
        }
    }
    Ok(())
}

fn apply_attr_key(id: u32, key: &str, value: &JsValue) -> Result<(), JsValue> {
    match key {
        "position" => element_set_position(id, value),
        "shape" => element_set_shape(id, value),
        "style" => element_set_style(id, value),
        _ => Ok(()),
    }
}

pub fn element_set_shape(id: u32, patch: &JsValue) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        let type_ok = reg.kind(id) == Some(ElementKind::Path);
        if !type_ok {
            return Ok(());
        }
        let new_shape = {
            let Some(PendingData::Path(pending)) = reg.pending_mut(id) else {
                return Ok(());
            };
            let merged = merge_shape(&pending.shape, patch)?;
            pending.shape = merged.clone();
            merged
        };
        let zr_id = reg.zr_id(id);
        let idx = reg.storage_index(id);
        if let (Some(zr_id), Some(idx)) = (zr_id, idx) {
            with_zr(zr_id, |zr| {
                let path = zr.storage.path_mut(idx);
                path.shape = new_shape;
                path.base.mark_shape_dirty();
                zr.storage.mark_display_dirty();
                Ok(())
            })?;
        }
        Ok(())
    })
}

pub fn element_set_style(id: u32, style: &JsValue) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        match reg.kind(id) {
            Some(ElementKind::Path) => {
                let patch = parse_path_style_patch(style);
                if let Some(PendingData::Path(pending)) = reg.pending_mut(id) {
                    pending.style = patch.apply_to(&pending.style);
                }
                let zr_id = reg.zr_id(id);
                let idx = reg.storage_index(id);
                if let (Some(zr_id), Some(idx)) = (zr_id, idx) {
                    with_zr(zr_id, |zr| {
                        let path = zr.storage.path_mut(idx);
                        path.style = patch.apply_to(&path.style);
                        path.base.mark_style_dirty();
                        zr.storage.mark_display_dirty();
                        Ok(())
                    })?;
                }
            }
            Some(ElementKind::Text) => {
                if let Some(PendingData::Text(pending)) = reg.pending_mut(id) {
                    if let Some(text) = get_string(style, "text") {
                        pending.content = text;
                    }
                    let parsed = parse_text_style(style);
                    if !get_object(style, "fill").is_undefined()
                        || get_string(style, "fill").is_some()
                        || get_string(style, "textFill").is_some()
                    {
                        pending.style.fill = parsed.fill;
                    }
                    if js_sys::Reflect::get(style, &"fontSize".into())
                        .ok()
                        .and_then(|v| v.as_f64())
                        .is_some()
                    {
                        pending.style.font_size = parsed.font_size;
                    }
                    pending.style.align = parsed.align;
                    pending.style.baseline = parsed.baseline;
                }
                let content = reg.pending(id).and_then(|p| {
                    if let PendingData::Text(t) = p {
                        Some((t.content.clone(), t.style.clone(), t.x, t.y))
                    } else {
                        None
                    }
                });
                let zr_id = reg.zr_id(id);
                let idx = reg.storage_index(id);
                if let (Some(zr_id), Some(idx), Some((content, style, x, y))) = (zr_id, idx, content)
                {
                    with_zr(zr_id, |zr| {
                        let text = zr.storage.text_mut(idx);
                        text.content = content;
                        text.style = style;
                        text.x = x;
                        text.y = y;
                        text.base.mark_style_dirty();
                        zr.storage.mark_display_dirty();
                        Ok(())
                    })?;
                }
            }
            _ => {}
        }
        Ok(())
    })
}

pub fn element_set_clip_path(id: u32, clip: JsValue) -> Result<(), JsValue> {
    let clip_id = match element_id_from_js(&clip) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(PendingData::Path(pending)) = reg.pending_mut(id) {
            pending.clip_element_id = Some(clip_id);
        }
        let zr_id = reg.zr_id(id);
        let host_idx = reg.storage_index(id);
        if let (Some(zr_id), Some(host_idx)) = (zr_id, host_idx) {
            with_zr(zr_id, |zr| {
                reg.materialize_element(zr, zr_id, clip_id)?;
                if let Some(clip_idx) = reg.storage_index(clip_id) {
                    zr.storage.path_mut(host_idx).clip_path = Some(clip_idx);
                    zr.storage.mark_display_dirty();
                }
                Ok(())
            })?;
        }
        Ok(())
    })
}

pub fn element_get_bounding_rect(id: u32) -> InnerRect {
    ELEMENT_REGISTRY.with(|reg| {
        let reg = reg.borrow();
        let mut acc: Option<InnerRect> = None;
        union_bbox(&reg, id, 0.0, 0.0, &mut acc);
        acc.unwrap_or_default()
    })
}

fn apply_position(id: u32, x: f64, y: f64) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(pending) = reg.pending_mut(id) {
            pending.set_position(x, y);
        }
        let kind = reg.kind(id);
        let zr_id = reg.zr_id(id);
        let idx = reg.storage_index(id);
        if let (Some(zr_id), Some(idx), Some(kind)) = (zr_id, idx, kind) {
            with_zr(zr_id, |zr| {
                match kind {
                    ElementKind::Path => {
                        let path = zr.storage.path_mut(idx);
                        path.base.transform_state.x = x;
                        path.base.transform_state.y = y;
                        path.base.mark_redraw();
                    }
                    ElementKind::Text => {
                        let text = zr.storage.text_mut(idx);
                        text.base.transform_state.x = x;
                        text.base.transform_state.y = y;
                        text.base.mark_redraw();
                    }
                    ElementKind::Image => {
                        let image = zr.storage.image_mut(idx);
                        image.base.transform_state.x = x;
                        image.base.transform_state.y = y;
                        image.base.mark_redraw();
                    }
                    ElementKind::Group => {
                        let group = zr.storage.group_mut(idx);
                        group.base.transform_state.x = x;
                        group.base.transform_state.y = y;
                        group.base.mark_redraw();
                    }
                }
                zr.storage.mark_display_dirty();
                Ok(())
            })?;
        }
        Ok(())
    })
}

fn union_bbox(
    reg: &crate::registry::ElementRegistry,
    id: u32,
    ox: f64,
    oy: f64,
    acc: &mut Option<InnerRect>,
) {
    let Some(pending) = reg.pending(id) else {
        return;
    };
    let (px, py) = pending.position();
    match pending {
        PendingData::Group(_) => {
            for child in reg.children_of(id) {
                union_bbox(reg, child, ox + px, oy + py, acc);
            }
        }
        PendingData::Path(p) => {
            let mut rect = shape_local_bbox(&p.shape);
            rect.x += ox + px;
            rect.y += oy + py;
            merge_acc(acc, rect);
        }
        PendingData::Text(t) => {
            let text = RustText::new(t.content.clone(), t.x, t.y).with_style(t.style.clone());
            let mut rect = text.bounding_rect();
            rect.x += ox + px;
            rect.y += oy + py;
            merge_acc(acc, rect);
        }
        PendingData::Image(img) => {
            let w = img.style.width.unwrap_or(img.style.source_width as f64);
            let h = img.style.height.unwrap_or(img.style.source_height as f64);
            let mut rect = InnerRect::new(img.style.x, img.style.y, w, h);
            rect.x += ox + px;
            rect.y += oy + py;
            merge_acc(acc, rect);
        }
    }
}

fn shape_local_bbox(shape: &rust_zrender::Shape) -> InnerRect {
    let mut path = RustPath::new(shape.clone(), PathStyle::default());
    path.bounding_rect().cloned().unwrap_or_default()
}

fn merge_acc(acc: &mut Option<InnerRect>, rect: InnerRect) {
    if rect.is_zero() {
        return;
    }
    match acc {
        Some(cur) => cur.union(&rect),
        None => *acc = Some(rect),
    }
}
