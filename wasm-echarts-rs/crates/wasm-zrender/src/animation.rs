//! 动画 API：链式 `when` / `during` / `done` / `start`。
//! 不播中间帧：`start()` 立刻写入最后一组 `when` 目标，再调用 during(percent=1) 与 done。

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct AnimatorInner {
    id: u32,
    path: Option<String>,
    last_props: JsValue,
    during: Vec<JsValue>,
    done: Vec<JsValue>,
}

/// 对齐 zrender `Animator`：终态语义。
#[wasm_bindgen]
#[derive(Clone)]
pub struct Animator {
    inner: Rc<RefCell<AnimatorInner>>,
}

#[wasm_bindgen]
impl Animator {
    pub fn when(&self, _time: f64, props: JsValue) -> Animator {
        if !props.is_undefined() && !props.is_null() {
            self.inner.borrow_mut().last_props = props;
        }
        self.clone()
    }

    pub fn during(&self, cb: JsValue) -> Animator {
        if cb.is_function() {
            self.inner.borrow_mut().during.push(cb);
        }
        self.clone()
    }

    pub fn done(&self, cb: JsValue) -> Animator {
        if cb.is_function() {
            self.inner.borrow_mut().done.push(cb);
        }
        self.clone()
    }

    pub fn delay(&self, _time: f64) -> Animator {
        self.clone()
    }

    pub fn start(&self, _easing: JsValue) -> Animator {
        let inner = self.inner.borrow();
        let id = inner.id;
        let path = inner.path.clone();
        let props = inner.last_props.clone();
        let during = inner.during.clone();
        let done = inner.done.clone();
        drop(inner);

        if !props.is_undefined() && !props.is_null() {
            let _ = crate::element::api::apply_animator_end_state(id, path.as_deref(), &props);
        }

        let percent = JsValue::from(1.0);
        for cb in &during {
            if let Some(func) = cb.dyn_ref::<Function>() {
                let _ = func.call2(&JsValue::NULL, &JsValue::UNDEFINED, &percent);
            }
        }
        for cb in &done {
            if let Some(func) = cb.dyn_ref::<Function>() {
                let _ = func.call0(&JsValue::NULL);
            }
        }
        self.clone()
    }

    pub fn stop(&self) {}
}

/// 对齐 `zr.animation`：无帧循环；`on('frame')` 为空操作。
#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Animation {}

#[wasm_bindgen]
impl Animation {
    pub fn on(&self, _event: &str, _handler: JsValue) {}

    pub fn off(&self, _event: JsValue, _handler: JsValue) {}
}

pub fn new_animator(id: u32, path: JsValue) -> Animator {
    let path = path.as_string().filter(|s| !s.is_empty());
    Animator {
        inner: Rc::new(RefCell::new(AnimatorInner {
            id,
            path,
            last_props: JsValue::UNDEFINED,
            during: Vec::new(),
            done: Vec::new(),
        })),
    }
}
