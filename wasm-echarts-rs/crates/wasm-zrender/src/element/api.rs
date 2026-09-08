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

pub fn element_animate(id: u32, path: JsValue, _looping: JsValue) -> Animator {
    new_animator(id, path)
}

pub fn apply_animator_end_state(
    id: u32,
    path: Option<&str>,
    props: &JsValue,
) -> Result<(), JsValue> {
    match path {
        Some("shape") => element_set_shape(id, props),
        Some("style") => element_set_style(id, props),
        _ => element_attr(id, props.clone(), JsValue::UNDEFINED),
    }
}

pub fn element_on(id: u32, event: &str, handler: JsValue) {
    crate::handler::element_on(id, event, handler);
}

pub fn element_off(id: u32, event: JsValue, handler: JsValue) {
    crate::handler::element_off(
        id,
        event.as_string().as_deref(),
        if handler.is_function() {
            Some(&handler)
        } else {
            None
        },
    );
}

pub fn element_trigger(id: u32, event: &str, packet: JsValue) {
    crate::handler::element_trigger(id, event, packet);
}

pub fn element_hide(id: u32) -> Result<(), JsValue> {
    apply_ignore(id, &JsValue::TRUE)
}

pub fn element_show(id: u32) -> Result<(), JsValue> {
    apply_ignore(id, &JsValue::FALSE)
}

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
        "draggable" => {
            crate::handler::element_set_draggable(id, value);
            Ok(())
        }
        "x" | "y" | "scaleX" | "scaleY" | "rotation" | "originX" | "originY" => {
            apply_transform_field(id, key, value)
        }
        "name" => apply_name(id, value),
        "ignore" => apply_ignore(id, value),
        "silent" => apply_silent(id, value),
        "z" | "z2" | "zlevel" => apply_z_field(id, key, value),
        "invisible" => apply_invisible(id, value),
        "clipPath" => element_set_clip_path(id, value.clone()),
        _ => Ok(()),
    }
}

pub fn element_set_shape(id: u32, patch: &JsValue) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        let type_ok = reg.kind(id) == Some(ElementKind::Path);
        if !type_ok {
            return Ok::<(), JsValue>(());
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
    })?;
    crate::handler::paint_element(id);
    Ok(())
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
        if let Some(pending) = reg.pending_mut(id) {
            pending.set_clip_element_id(Some(clip_id));
        }
        apply_clip_to_storage(&mut reg, id, Some(clip_id))
    })
}

pub fn element_remove_clip_path(id: u32) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(pending) = reg.pending_mut(id) {
            pending.set_clip_element_id(None);
        }
        apply_clip_to_storage(&mut reg, id, None)
    })
}

fn apply_clip_to_storage(
    reg: &mut crate::registry::ElementRegistry,
    host_id: u32,
    clip_id: Option<u32>,
) -> Result<(), JsValue> {
    let zr_id = reg.zr_id(host_id);
    let host_idx = reg.storage_index(host_id);
    let kind = reg.kind(host_id);
    if let (Some(zr_id), Some(host_idx), Some(kind)) = (zr_id, host_idx, kind) {
        with_zr(zr_id, |zr| {
            let clip_idx = if let Some(clip_id) = clip_id {
                reg.materialize_element(zr, zr_id, clip_id)?;
                if reg.kind(clip_id) == Some(ElementKind::Path) {
                    reg.storage_index(clip_id)
                } else {
                    None
                }
            } else {
                None
            };
            match kind {
                ElementKind::Path => zr.storage.path_mut(host_idx).clip_path = clip_idx,
                ElementKind::Text => zr.storage.text_mut(host_idx).clip_path = clip_idx,
                ElementKind::Image => zr.storage.image_mut(host_idx).clip_path = clip_idx,
                ElementKind::Group => zr.storage.group_mut(host_idx).clip_path = clip_idx,
            }
            zr.storage.mark_display_dirty();
            Ok(())
        })?;
    }
    Ok(())
}

pub fn element_use_states(id: u32, states: JsValue) -> Result<(), JsValue> {
    let arr = js_sys::Array::from(&states);
    let mut names = Vec::new();
    for i in 0..arr.length() {
        if let Some(s) = arr.get(i).as_string() {
            names.push(s);
        }
    }
    crate::registry::path_use_states(id, &names)
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
        Ok::<(), JsValue>(())
    })?;
    sync_transform_to_storage(id)?;
    crate::handler::paint_element(id);
    Ok(())
}

fn apply_transform_field(id: u32, key: &str, value: &JsValue) -> Result<(), JsValue> {
    let Some(n) = value.as_f64() else {
        return Ok(());
    };
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(pending) = reg.pending_mut(id) {
            pending.transform_mut().set_field(key, n);
        }
        Ok::<(), JsValue>(())
    })?;
    sync_transform_to_storage(id)?;
    crate::handler::paint_element(id);
    Ok(())
}

fn sync_transform_to_storage(id: u32) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let reg = reg.borrow();
        let transform = match reg.pending(id) {
            Some(pending) => pending.transform().clone(),
            None => return Ok(()),
        };
        let kind = reg.kind(id);
        let zr_id = reg.zr_id(id);
        let idx = reg.storage_index(id);
        if let (Some(zr_id), Some(idx), Some(kind)) = (zr_id, idx, kind) {
            with_zr(zr_id, |zr| {
                match kind {
                    ElementKind::Path => transform.apply_to_base(&mut zr.storage.path_mut(idx).base),
                    ElementKind::Text => transform.apply_to_base(&mut zr.storage.text_mut(idx).base),
                    ElementKind::Image => {
                        transform.apply_to_base(&mut zr.storage.image_mut(idx).base)
                    }
                    ElementKind::Group => {
                        transform.apply_to_base(&mut zr.storage.group_mut(idx).base)
                    }
                }
                zr.storage.mark_display_dirty();
                Ok(())
            })?;
        }
        Ok::<(), JsValue>(())
    })
}

fn apply_name(id: u32, value: &JsValue) -> Result<(), JsValue> {
    let name = value.as_string().unwrap_or_default();
    apply_base_meta(id, |pending| pending.set_name(name.clone()), |base| {
        base.name = name.clone();
        base.mark_redraw();
    })
}

fn apply_ignore(id: u32, value: &JsValue) -> Result<(), JsValue> {
    let ignore = value.as_bool().unwrap_or(false);
    apply_base_meta(id, |pending| pending.set_ignore(ignore), |base| {
        base.ignore = ignore;
        base.mark_redraw();
    })
}

fn apply_silent(id: u32, value: &JsValue) -> Result<(), JsValue> {
    let silent = value.as_bool().unwrap_or(false);
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(pending) = reg.pending_mut(id) {
            pending.set_silent(silent);
        }
        let kind = reg.kind(id);
        let zr_id = reg.zr_id(id);
        let idx = reg.storage_index(id);
        if let (Some(zr_id), Some(idx), Some(kind)) = (zr_id, idx, kind) {
            with_zr(zr_id, |zr| {
                match kind {
                    ElementKind::Path => zr.storage.path_mut(idx).silent = silent,
                    ElementKind::Text => zr.storage.text_mut(idx).silent = silent,
                    ElementKind::Image => zr.storage.image_mut(idx).silent = silent,
                    ElementKind::Group => {}
                }
                zr.storage.mark_display_dirty();
                Ok(())
            })?;
        }
        Ok::<(), JsValue>(())
    })?;
    crate::handler::paint_element(id);
    Ok(())
}

fn apply_z_field(id: u32, key: &str, value: &JsValue) -> Result<(), JsValue> {
    let Some(n) = value.as_f64() else {
        return Ok(());
    };
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(displayable) = reg.pending_mut(id).and_then(|p| p.displayable_mut()) {
            match key {
                "z" => displayable.z = n,
                "z2" => displayable.z2 = n,
                "zlevel" => displayable.zlevel = n,
                _ => {}
            }
        }
        let kind = reg.kind(id);
        let zr_id = reg.zr_id(id);
        let idx = reg.storage_index(id);
        if let (Some(zr_id), Some(idx), Some(kind)) = (zr_id, idx, kind) {
            with_zr(zr_id, |zr| {
                let displayable = match kind {
                    ElementKind::Path => Some(&mut zr.storage.path_mut(idx).displayable),
                    ElementKind::Text => Some(&mut zr.storage.text_mut(idx).displayable),
                    ElementKind::Image => Some(&mut zr.storage.image_mut(idx).displayable),
                    ElementKind::Group => None,
                };
                if let Some(displayable) = displayable {
                    match key {
                        "z" => displayable.z = n,
                        "z2" => displayable.z2 = n,
                        "zlevel" => displayable.zlevel = n,
                        _ => {}
                    }
                }
                zr.storage.mark_display_dirty();
                Ok(())
            })?;
        }
        Ok::<(), JsValue>(())
    })?;
    crate::handler::paint_element(id);
    Ok(())
}

fn apply_invisible(id: u32, value: &JsValue) -> Result<(), JsValue> {
    let invisible = value.as_bool().unwrap_or(false);
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(displayable) = reg.pending_mut(id).and_then(|p| p.displayable_mut()) {
            displayable.invisible = invisible;
        }
        let kind = reg.kind(id);
        let zr_id = reg.zr_id(id);
        let idx = reg.storage_index(id);
        if let (Some(zr_id), Some(idx), Some(kind)) = (zr_id, idx, kind) {
            with_zr(zr_id, |zr| {
                match kind {
                    ElementKind::Path => zr.storage.path_mut(idx).displayable.invisible = invisible,
                    ElementKind::Text => zr.storage.text_mut(idx).displayable.invisible = invisible,
                    ElementKind::Image => {
                        zr.storage.image_mut(idx).displayable.invisible = invisible
                    }
                    ElementKind::Group => {}
                }
                zr.storage.mark_display_dirty();
                Ok(())
            })?;
        }
        Ok::<(), JsValue>(())
    })?;
    crate::handler::paint_element(id);
    Ok(())
}

fn apply_base_meta(
    id: u32,
    update_pending: impl FnOnce(&mut PendingData),
    update_base: impl Fn(&mut rust_zrender::element::ElementBase),
) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(pending) = reg.pending_mut(id) {
            update_pending(pending);
        }
        let kind = reg.kind(id);
        let zr_id = reg.zr_id(id);
        let idx = reg.storage_index(id);
        if let (Some(zr_id), Some(idx), Some(kind)) = (zr_id, idx, kind) {
            with_zr(zr_id, |zr| {
                match kind {
                    ElementKind::Path => update_base(&mut zr.storage.path_mut(idx).base),
                    ElementKind::Text => update_base(&mut zr.storage.text_mut(idx).base),
                    ElementKind::Image => update_base(&mut zr.storage.image_mut(idx).base),
                    ElementKind::Group => update_base(&mut zr.storage.group_mut(idx).base),
                }
                zr.storage.mark_display_dirty();
                Ok(())
            })?;
        }
        Ok::<(), JsValue>(())
    })?;
    crate::handler::paint_element(id);
    Ok(())
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
