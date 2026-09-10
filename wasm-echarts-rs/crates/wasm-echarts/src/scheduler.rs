//! setOption 更新调度（简化版：单次全量 refresh）

use rust_zrender::ZRenderer;

use crate::interaction::InteractionState;
use crate::option::OptionModel;
use crate::render;

pub fn run_update(
    zr: &mut ZRenderer,
    zr_id: u32,
    option: &OptionModel,
    width: u32,
    height: u32,
    interaction: &InteractionState,
) {
    render::render_chart(zr, zr_id, option, width, height, interaction);
}
