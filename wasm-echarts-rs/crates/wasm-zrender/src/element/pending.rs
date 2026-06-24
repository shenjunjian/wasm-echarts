//! 未挂载图元的 pending 数据（挂载 ZRender 时再写入 Storage）

use std::collections::HashMap;

use rust_zrender::{
    DisplayableProps, EcData, ImageStyle, PathStyle, PathStylePatch, Shape, TextStyle,
};

#[derive(Debug, Clone)]
pub struct PendingPath {
    pub shape: Shape,
    pub style: PathStyle,
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: String,
    pub ec_data: EcData,
    pub state_patches: HashMap<String, PathStylePatch>,
    pub active_states: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PendingText {
    pub content: String,
    pub x: f64,
    pub y: f64,
    pub style: TextStyle,
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: String,
    pub ec_data: EcData,
}

#[derive(Debug, Clone)]
pub struct PendingImage {
    pub style: ImageStyle,
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: String,
    pub ec_data: EcData,
}

#[derive(Debug, Clone)]
pub enum PendingData {
    Group,
    Path(PendingPath),
    Text(PendingText),
    Image(PendingImage),
}

impl PendingData {
    pub fn group() -> Self {
        Self::Group
    }
}
