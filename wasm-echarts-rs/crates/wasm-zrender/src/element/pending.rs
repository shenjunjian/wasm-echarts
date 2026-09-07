//! 未挂载图元的 pending 数据（挂载 ZRender 时再写入 Storage）

use std::collections::HashMap;

use rust_zrender::{
    DisplayableProps, EcData, ImageStyle, PathStyle, PathStylePatch, Shape, TextStyle,
};

#[derive(Debug, Clone, Default)]
pub struct PendingGroup {
    pub x: f64,
    pub y: f64,
}

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
    pub x: f64,
    pub y: f64,
    pub clip_element_id: Option<u32>,
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
    pub tx: f64,
    pub ty: f64,
}

#[derive(Debug, Clone)]
pub struct PendingImage {
    pub style: ImageStyle,
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: String,
    pub ec_data: EcData,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub enum PendingData {
    Group(PendingGroup),
    Path(PendingPath),
    Text(PendingText),
    Image(PendingImage),
}

impl PendingData {
    pub fn group() -> Self {
        Self::Group(PendingGroup::default())
    }

    pub fn position(&self) -> (f64, f64) {
        match self {
            Self::Group(g) => (g.x, g.y),
            Self::Path(p) => (p.x, p.y),
            Self::Text(t) => (t.tx, t.ty),
            Self::Image(p) => (p.x, p.y),
        }
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        match self {
            Self::Group(g) => {
                g.x = x;
                g.y = y;
            }
            Self::Path(p) => {
                p.x = x;
                p.y = y;
            }
            Self::Text(t) => {
                t.tx = x;
                t.ty = y;
            }
            Self::Image(p) => {
                p.x = x;
                p.y = y;
            }
        }
    }
}
