//! 渲染入口：GlobalModel → zrender Storage

use rust_zrender::{ChildRef, Storage, ZRenderer};

use crate::chart::render_components;
use crate::interaction::InteractionState;
use crate::model::GlobalModel;
use crate::option::OptionModel;

pub const CHART_ROOT_NAME: &str = "__ec_chart_root";

pub fn render_chart(
    zr: &mut ZRenderer,
    zr_id: u32,
    option: &OptionModel,
    width: u32,
    height: u32,
    interaction: &InteractionState,
) {
    zr.storage = Storage::new();
    let _ = wasm_zrender::rematerialize_mounted_roots(zr, zr_id);
    if option.is_empty() {
        return;
    }
    let effective = OptionModel::with_root(option.effective_root(
        interaction.timeline_index,
        width as f64,
        height as f64,
    ));
    let model = GlobalModel::from_option_with_zoom(&effective, width, height, interaction.data_zoom);
    let group = zr.storage.create_group();
    zr.storage.group_mut(group).base.name = CHART_ROOT_NAME.to_string();
    render_components(zr, group, &model, &effective, interaction);
    zr.storage.add_root(ChildRef::Group(group));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::InteractionState;
    use crate::option::OptionModel;

    #[test]
    fn empty_render_resets_storage() {
        let option = OptionModel::new();
        let interaction = InteractionState::default();
        let mut zr = ZRenderer::new(120, 80).unwrap();
        let stray = zr.storage.create_group();
        zr.storage.add_root(ChildRef::Group(stray));
        render_chart(&mut zr, 1, &option, 120, 80, &interaction);
        assert!(zr.storage.roots().is_empty());
        render_chart(&mut zr, 1, &option, 120, 80, &interaction);
        assert!(zr.storage.roots().is_empty());
    }
}
