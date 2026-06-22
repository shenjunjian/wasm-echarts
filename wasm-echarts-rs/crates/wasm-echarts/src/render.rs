//! 渲染入口：GlobalModel → zrender Storage

use wasm_zrender::{Storage, ZRenderer};

use crate::chart::render_components;
use crate::model::GlobalModel;
use crate::option::OptionModel;

pub fn render_chart(zr: &mut ZRenderer, option: &OptionModel, width: u32, height: u32) {
    zr.storage = Storage::new();
    let model = GlobalModel::from_option(option, width, height);
    if model.series.is_empty() {
        return;
    }
    render_components(zr, &model, option);
}
