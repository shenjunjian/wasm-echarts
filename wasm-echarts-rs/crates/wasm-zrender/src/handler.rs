//! Handler：事件分发 + draggable + DOM proxy（对齐 zrender Handler / HandlerProxy）
//!
//! - 有 canvas：绑定 pointer 事件，自动 putImageData
//! - 无 DOM：通过 `zr.handler.dispatch(name, { zrX, zrY })` 注入，供测试 / Node 使用

use std::cell::RefCell;
use std::collections::HashMap;

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, PointerEvent};

use crate::bridge::hit::hit_to_hover_result;
use crate::bridge::opts::{get_f64, DraggableKind};
use crate::element::api::element_set_position;
use crate::element::Element;
use crate::registry::{with_zr, ELEMENT_REGISTRY};

struct HandlerState {
    canvas: Option<HtmlCanvasElement>,
    zr_listeners: HashMap<String, Vec<JsValue>>,
    dragging_id: Option<u32>,
    drag_x: f64,
    drag_y: f64,
    closures: Vec<Closure<dyn FnMut(PointerEvent)>>,
}

impl Default for HandlerState {
    fn default() -> Self {
        Self {
            canvas: None,
            zr_listeners: HashMap::new(),
            dragging_id: None,
            drag_x: 0.0,
            drag_y: 0.0,
            closures: Vec::new(),
        }
    }
}

thread_local! {
    pub static HANDLER_REGISTRY: RefCell<HashMap<u32, HandlerState>> =
        RefCell::new(HashMap::new());
}

/// 对齐官方 `zr.handler`：无 DOM 时用 `dispatch` 注入指针事件。
#[wasm_bindgen]
pub struct Handler {
    zr_id: u32,
}

impl Handler {
    pub(crate) fn from_zr(zr_id: u32) -> Self {
        Self { zr_id }
    }
}

#[wasm_bindgen]
impl Handler {
    pub fn dispatch(&self, event_name: &str, event: JsValue) -> Result<(), JsValue> {
        let x = get_f64(&event, "zrX")
            .or_else(|| get_f64(&event, "offsetX"))
            .unwrap_or(0.0);
        let y = get_f64(&event, "zrY")
            .or_else(|| get_f64(&event, "offsetY"))
            .unwrap_or(0.0);
        dispatch_pointer(self.zr_id, event_name, x, y, event)
    }
}

pub fn attach(zr_id: u32, dom: &JsValue) -> Result<(), JsValue> {
    HANDLER_REGISTRY.with(|reg| {
        reg.borrow_mut().insert(zr_id, HandlerState::default());
    });
    let Some(canvas) = as_canvas(dom) else {
        return Ok(());
    };
    bind_canvas(zr_id, canvas)
}

pub fn detach(zr_id: u32) {
    HANDLER_REGISTRY.with(|reg| {
        if let Some(mut state) = reg.borrow_mut().remove(&zr_id) {
            if let Some(canvas) = state.canvas.take() {
                let target: web_sys::EventTarget = canvas.into();
                for (name, closure) in [
                    "pointerdown",
                    "pointermove",
                    "pointerup",
                    "pointercancel",
                ]
                .iter()
                .zip(state.closures.drain(..))
                {
                    let _ = target.remove_event_listener_with_callback(
                        name,
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }
        }
    });
}

pub fn detach_all() {
    let ids: Vec<u32> = HANDLER_REGISTRY.with(|reg| reg.borrow().keys().copied().collect());
    for id in ids {
        detach(id);
    }
}

pub fn add_zr_listener(zr_id: u32, event: &str, handler: JsValue) {
    if !handler.is_function() {
        return;
    }
    HANDLER_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        let state = reg.entry(zr_id).or_default();
        state
            .zr_listeners
            .entry(event.to_string())
            .or_default()
            .push(handler);
    });
}

pub fn remove_zr_listeners(zr_id: u32, event: Option<&str>) {
    HANDLER_REGISTRY.with(|reg| {
        if let Some(state) = reg.borrow_mut().get_mut(&zr_id) {
            if let Some(name) = event {
                state.zr_listeners.remove(name);
            } else {
                state.zr_listeners.clear();
            }
        }
    });
}

pub fn element_on(id: u32, event: &str, handler: JsValue) {
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut().add_listener(id, event, handler);
    });
}

pub fn element_draggable_js(id: u32) -> JsValue {
    ELEMENT_REGISTRY.with(|reg| reg.borrow().draggable(id).to_js())
}

pub fn element_set_draggable(id: u32, value: &JsValue) {
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .set_draggable(id, DraggableKind::from_js(value));
    });
}

pub fn paint_if_bound(zr_id: u32) {
    let canvas = HANDLER_REGISTRY.with(|reg| {
        reg.borrow()
            .get(&zr_id)
            .and_then(|s| s.canvas.clone())
    });
    let Some(canvas) = canvas else {
        return;
    };
    let Ok(rgba) = with_zr(zr_id, |zr| {
        zr.refresh()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }) else {
        return;
    };
    let width = with_zr(zr_id, |zr| Ok(zr.width())).unwrap_or(0);
    let height = with_zr(zr_id, |zr| Ok(zr.height())).unwrap_or(0);
    let _ = blit_canvas(&canvas, &rgba, width, height);
}

pub fn paint_element(id: u32) {
    let zr_id = ELEMENT_REGISTRY.with(|reg| reg.borrow().zr_id(id));
    if let Some(zr_id) = zr_id {
        paint_if_bound(zr_id);
    }
}

fn as_canvas(dom: &JsValue) -> Option<HtmlCanvasElement> {
    if dom.is_null() || dom.is_undefined() {
        return None;
    }
    if let Ok(canvas) = dom.clone().dyn_into::<HtmlCanvasElement>() {
        return Some(canvas);
    }
    let el = dom.dyn_ref::<web_sys::Element>()?;
    el.query_selector("canvas")
        .ok()
        .flatten()
        .and_then(|node| node.dyn_into::<HtmlCanvasElement>().ok())
}

fn bind_canvas(zr_id: u32, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    let _ = canvas.style().set_property("touch-action", "none");
    let target: web_sys::EventTarget = canvas.clone().into();

    let down = make_listener(zr_id, "mousedown");
    let move_ = make_listener(zr_id, "mousemove");
    let up = make_listener(zr_id, "mouseup");
    let cancel = make_listener(zr_id, "mouseup");

    target.add_event_listener_with_callback("pointerdown", down.as_ref().unchecked_ref())?;
    target.add_event_listener_with_callback("pointermove", move_.as_ref().unchecked_ref())?;
    target.add_event_listener_with_callback("pointerup", up.as_ref().unchecked_ref())?;
    target.add_event_listener_with_callback("pointercancel", cancel.as_ref().unchecked_ref())?;

    HANDLER_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        let state = reg.entry(zr_id).or_default();
        state.canvas = Some(canvas);
        state.closures = vec![down, move_, up, cancel];
    });
    Ok(())
}

fn make_listener(zr_id: u32, name: &'static str) -> Closure<dyn FnMut(PointerEvent)> {
    Closure::wrap(Box::new(move |event: PointerEvent| {
        let canvas = HANDLER_REGISTRY.with(|reg| {
            reg.borrow()
                .get(&zr_id)
                .and_then(|s| s.canvas.clone())
        });
        let Some(canvas) = canvas else {
            return;
        };
        if name == "mousedown" {
            let _ = canvas.set_pointer_capture(event.pointer_id());
        } else if name == "mouseup" {
            let _ = canvas.release_pointer_capture(event.pointer_id());
        }
        let (zr_w, zr_h) = with_zr(zr_id, |zr| Ok((zr.width() as f64, zr.height() as f64)))
            .unwrap_or((1.0, 1.0));
        let (x, y) = event_xy(&canvas, &event, zr_w, zr_h);
        let _ = dispatch_pointer(zr_id, name, x, y, event.into());
    }) as Box<dyn FnMut(PointerEvent)>)
}

fn event_xy(
    canvas: &HtmlCanvasElement,
    event: &PointerEvent,
    zr_w: f64,
    zr_h: f64,
) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let w = rect.width().max(1.0);
    let h = rect.height().max(1.0);
    (
        (event.client_x() as f64 - rect.left()) * (zr_w / w),
        (event.client_y() as f64 - rect.top()) * (zr_h / h),
    )
}

fn dispatch_pointer(
    zr_id: u32,
    name: &str,
    x: f64,
    y: f64,
    native: JsValue,
) -> Result<(), JsValue> {
    let hover = with_zr(zr_id, |zr| Ok(zr.find_hover(x, y)))
        .ok()
        .flatten();
    let hover_el = hover.as_ref().and_then(hit_to_hover_result);
    let target_id = hover_el.as_ref().map(|h| h.target().raw_id());

    if name == "mousedown" {
        let drag_id = target_id.and_then(|id| {
            ELEMENT_REGISTRY.with(|reg| reg.borrow().find_draggable_ancestor(id))
        });
        HANDLER_REGISTRY.with(|reg| {
            if let Some(state) = reg.borrow_mut().get_mut(&zr_id) {
                state.dragging_id = drag_id;
                state.drag_x = x;
                state.drag_y = y;
            }
        });
    }

    if name == "mousemove" {
        let drag = HANDLER_REGISTRY.with(|reg| {
            reg.borrow()
                .get(&zr_id)
                .and_then(|s| s.dragging_id.map(|id| (id, s.drag_x, s.drag_y)))
        });
        if let Some((id, last_x, last_y)) = drag {
            let kind = ELEMENT_REGISTRY.with(|reg| reg.borrow().draggable(id));
            let (dx, dy) = kind.clamp_delta(x - last_x, y - last_y);
            HANDLER_REGISTRY.with(|reg| {
                if let Some(state) = reg.borrow_mut().get_mut(&zr_id) {
                    state.drag_x = x;
                    state.drag_y = y;
                }
            });
            if dx != 0.0 || dy != 0.0 {
                let (px, py) = ELEMENT_REGISTRY.with(|reg| {
                    reg.borrow()
                        .pending(id)
                        .map(|p| p.position())
                        .unwrap_or((0.0, 0.0))
                });
                let pos = js_sys::Array::new();
                pos.push(&JsValue::from(px + dx));
                pos.push(&JsValue::from(py + dy));
                let _ = element_set_position(id, &pos.into());
            }
        }
        set_cursor(zr_id, target_id);
    }

    if name == "mouseup" {
        HANDLER_REGISTRY.with(|reg| {
            if let Some(state) = reg.borrow_mut().get_mut(&zr_id) {
                state.dragging_id = None;
            }
        });
        set_cursor(zr_id, target_id);
    }

    fire(zr_id, name, x, y, target_id, &native);
    paint_if_bound(zr_id);
    Ok(())
}

fn fire(zr_id: u32, name: &str, x: f64, y: f64, target_id: Option<u32>, native: &JsValue) {
    let packet = make_packet(name, x, y, target_id, native);
    if let Some(mut id) = target_id {
        loop {
            let handlers = ELEMENT_REGISTRY.with(|reg| reg.borrow().listeners(id, name));
            call_handlers(&handlers, &packet);
            match ELEMENT_REGISTRY.with(|reg| reg.borrow().parent_id(id)) {
                Some(parent) => id = parent,
                None => break,
            }
        }
    }
    let zr_handlers = HANDLER_REGISTRY.with(|reg| {
        reg.borrow()
            .get(&zr_id)
            .map(|s| s.zr_listeners.get(name).cloned().unwrap_or_default())
            .unwrap_or_default()
    });
    call_handlers(&zr_handlers, &packet);
}

fn call_handlers(handlers: &[JsValue], packet: &JsValue) {
    for handler in handlers {
        if let Some(func) = handler.dyn_ref::<Function>() {
            let _ = func.call1(&JsValue::NULL, packet);
        }
    }
}

fn make_packet(name: &str, x: f64, y: f64, target_id: Option<u32>, native: &JsValue) -> JsValue {
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"type".into(), &JsValue::from_str(name));
    let _ = Reflect::set(&obj, &"offsetX".into(), &JsValue::from(x));
    let _ = Reflect::set(&obj, &"offsetY".into(), &JsValue::from(y));
    let _ = Reflect::set(&obj, &"zrX".into(), &JsValue::from(x));
    let _ = Reflect::set(&obj, &"zrY".into(), &JsValue::from(y));
    if let Some(id) = target_id {
        let _ = Reflect::set(
            &obj,
            &"target".into(),
            &JsValue::from(Element::from_id(id)),
        );
    }
    if !native.is_undefined() && !native.is_null() {
        let _ = Reflect::set(&obj, &"event".into(), native);
    }
    obj.into()
}

fn set_cursor(zr_id: u32, target_id: Option<u32>) {
    let canvas = HANDLER_REGISTRY.with(|reg| {
        reg.borrow()
            .get(&zr_id)
            .and_then(|s| s.canvas.clone())
    });
    let Some(canvas) = canvas else {
        return;
    };
    let dragging = HANDLER_REGISTRY.with(|reg| {
        reg.borrow()
            .get(&zr_id)
            .and_then(|s| s.dragging_id)
            .is_some()
    });
    let cursor = if dragging {
        "grabbing"
    } else if target_id.is_some_and(|id| {
        ELEMENT_REGISTRY.with(|reg| !reg.borrow().draggable(id).is_none())
    }) {
        "move"
    } else {
        "default"
    };
    let _ = canvas.style().set_property("cursor", cursor);
}

fn blit_canvas(
    canvas: &HtmlCanvasElement,
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<(), JsValue> {
    if width == 0 || height == 0 {
        return Ok(());
    }
    let ctx = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("2d context missing"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(rgba), width, height)?;
    ctx.put_image_data(&image, 0.0, 0.0)?;
    Ok(())
}
