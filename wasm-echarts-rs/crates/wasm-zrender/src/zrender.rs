//! ZRender 实例 + init / dispose 顶层 API

use wasm_bindgen::prelude::*;

use crate::bridge::hit::hit_to_hover_result;
use crate::element::js::element_from_js;
use crate::bridge::opts::{parse_init_opts, InitOpts};
use crate::registry::{
    mount_element_to_zr, unmount_element_from_zr, with_zr, ZR_REGISTRY, ELEMENT_REGISTRY,
};
use rust_zrender::ZRenderer;

#[wasm_bindgen]
pub struct ZRender {
    id: u32,
}

impl ZRender {
    fn from_id(id: u32) -> Self {
        Self { id }
    }
}

#[wasm_bindgen]
impl ZRender {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn width(&self) -> u32 {
        with_zr(self.id, |zr| Ok(zr.width())).unwrap_or(0)
    }

    pub fn height(&self) -> u32 {
        with_zr(self.id, |zr| Ok(zr.height())).unwrap_or(0)
    }

    pub fn dpr(&self) -> f64 {
        with_zr(self.id, |zr| Ok(zr.dpr())).unwrap_or(1.0)
    }

    pub fn add(&self, el: JsValue) -> Result<(), JsValue> {
        let element = element_from_js(&el)?;
        mount_element_to_zr(self.id, &element)
    }

    pub fn remove(&self, el: JsValue) -> Result<(), JsValue> {
        let element = element_from_js(&el)?;
        unmount_element_from_zr(self.id, &element)
    }

    pub fn refresh(&mut self) -> Result<Vec<u8>, JsValue> {
        with_zr(self.id, |zr| {
            zr.refresh()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        })
    }

    pub fn flush(&mut self) -> Result<Vec<u8>, JsValue> {
        self.refresh()
    }

    pub fn resize(&mut self, opts: JsValue) -> Result<(), JsValue> {
        let InitOpts { width, height, dpr } = parse_init_opts(&JsValue::NULL, &opts)?;
        with_zr(self.id, |zr| {
            zr.resize_with_dpr(width, height, dpr)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        })
    }

    #[wasm_bindgen(js_name = findHover)]
    pub fn find_hover(&mut self, x: f64, y: f64) -> Option<crate::bridge::hit::HoverResult> {
        with_zr(self.id, |zr| Ok(zr.find_hover(x, y)))
            .ok()
            .flatten()
            .and_then(|hit| hit_to_hover_result(&hit))
    }
}

/// 创建 ZRender 实例（dom 参数忽略，尺寸来自 opts）
#[wasm_bindgen]
pub fn init(dom: JsValue, opts: JsValue) -> Result<ZRender, JsValue> {
    crate::utils::set_panic_hook();
    let InitOpts { width, height, dpr } = parse_init_opts(&dom, &opts)?;
    let zr = ZRenderer::new_with_dpr(width, height, dpr)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let id = ZR_REGISTRY.with(|reg| reg.borrow_mut().insert(zr));
    Ok(ZRender::from_id(id))
}

#[wasm_bindgen]
pub fn dispose(zr: ZRender) {
    let id = zr.id;
    ZR_REGISTRY.with(|reg| {
        reg.borrow_mut().remove(id);
    });
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut().remove_by_zr(id);
    });
}

#[wasm_bindgen(js_name = disposeAll)]
pub fn dispose_all() {
    ZR_REGISTRY.with(|reg| {
        reg.borrow_mut().clear();
    });
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut().clear();
    });
}

#[wasm_bindgen(js_name = getInstance)]
pub fn get_instance(id: u32) -> Option<ZRender> {
    ZR_REGISTRY.with(|reg| {
        if reg.borrow().contains(id) {
            Some(ZRender::from_id(id))
        } else {
            None
        }
    })
}
