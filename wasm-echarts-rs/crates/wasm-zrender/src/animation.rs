//! 动画 API stub：方法可链式调用且不抛错，当前不播放关键帧。

use wasm_bindgen::prelude::*;

/// 对齐 zrender `Animator`：`when` / `during` / `done` / `start` 为空操作。
#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Animator {}

#[wasm_bindgen]
impl Animator {
    pub fn when(&self, _time: f64, _props: JsValue) -> Animator {
        self.clone()
    }

    pub fn during(&self, _cb: JsValue) -> Animator {
        self.clone()
    }

    pub fn done(&self, _cb: JsValue) -> Animator {
        self.clone()
    }

    pub fn delay(&self, _time: f64) -> Animator {
        self.clone()
    }

    pub fn start(&self, _easing: JsValue) -> Animator {
        self.clone()
    }

    pub fn stop(&self) {}
}

/// 对齐 `zr.animation`：`on('frame')` 为空操作（无帧循环）。
#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Animation {}

#[wasm_bindgen]
impl Animation {
    pub fn on(&self, _event: &str, _handler: JsValue) {}

    pub fn off(&self, _event: JsValue, _handler: JsValue) {}
}

pub fn new_animator() -> Animator {
    Animator {}
}
