//! ZRender 实例 + init / dispose 顶层 API

use wasm_bindgen::prelude::*;

use crate::bridge::hit::hit_to_hover_result;
use crate::element::js::element_from_js;
use crate::bridge::opts::{parse_init_opts, InitOpts};
use crate::registry::{
    clear_zr, mount_element_to_zr, unmount_element_from_zr, with_zr, ZR_REGISTRY, ELEMENT_REGISTRY,
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
        mount_element_to_zr(self.id, &element)?;
        crate::handler::paint_if_bound(self.id);
        Ok(())
    }

    pub fn remove(&self, el: JsValue) -> Result<(), JsValue> {
        let element = element_from_js(&el)?;
        unmount_element_from_zr(self.id, &element)?;
        crate::handler::paint_if_bound(self.id);
        Ok(())
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

    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&self) -> u32 {
        self.width()
    }

    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> u32 {
        self.height()
    }

    pub fn on(&self, event: &str, handler: JsValue) -> ZRender {
        crate::handler::add_zr_listener(self.id, event, handler);
        ZRender::from_id(self.id)
    }

    pub fn off(&self, event: JsValue, handler: JsValue) -> ZRender {
        crate::handler::remove_zr_listeners(
            self.id,
            event.as_string().as_deref(),
            if handler.is_function() {
                Some(&handler)
            } else {
                None
            },
        );
        ZRender::from_id(self.id)
    }

    pub fn trigger(&self, event: &str, packet: JsValue) -> ZRender {
        crate::handler::trigger_zr(self.id, event, packet);
        ZRender::from_id(self.id)
    }

    pub fn clear(&self) -> Result<(), JsValue> {
        clear_zr(self.id)
    }

    #[wasm_bindgen(js_name = setBackgroundColor)]
    pub fn set_background_color(&self, color: JsValue) -> Result<(), JsValue> {
        let parsed = if color.is_null() || color.is_undefined() {
            None
        } else {
            color.as_string()
        };
        with_zr(self.id, |zr| {
            zr.set_background_color(parsed);
            Ok(())
        })?;
        crate::handler::paint_if_bound(self.id);
        Ok(())
    }

    #[wasm_bindgen(js_name = getBackgroundColor)]
    pub fn get_background_color(&self) -> JsValue {
        with_zr(self.id, |zr| {
            Ok(zr
                .background_color()
                .map(JsValue::from_str)
                .unwrap_or(JsValue::UNDEFINED))
        })
        .unwrap_or(JsValue::UNDEFINED)
    }

    pub fn dispose(&self) {
        dispose_zr_id(self.id);
    }

    #[wasm_bindgen(getter)]
    pub fn handler(&self) -> crate::handler::Handler {
        crate::handler::Handler::from_zr(self.id)
    }

    #[wasm_bindgen(getter)]
    pub fn animation(&self) -> crate::animation::Animation {
        crate::animation::Animation::default()
    }
}

fn dispose_zr_id(id: u32) {
    crate::handler::detach(id);
    ZR_REGISTRY.with(|reg| {
        reg.borrow_mut().remove(id);
    });
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut().remove_by_zr(id);
    });
}

/// 创建 ZRender 实例（dom 参数忽略，尺寸来自 opts）
#[wasm_bindgen]
pub fn init(dom: JsValue, opts: JsValue) -> Result<ZRender, JsValue> {
    crate::utils::set_panic_hook();
    let InitOpts { width, height, dpr } = parse_init_opts(&dom, &opts)?;
    let zr = ZRenderer::new_with_dpr(width, height, dpr)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let id = ZR_REGISTRY.with(|reg| reg.borrow_mut().insert(zr));
    crate::handler::attach(id, &dom)?;
    Ok(ZRender::from_id(id))
}

#[wasm_bindgen]
pub fn dispose(zr: &ZRender) {
    dispose_zr_id(zr.id());
}

#[wasm_bindgen(js_name = disposeAll)]
pub fn dispose_all() {
    crate::handler::detach_all();
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
