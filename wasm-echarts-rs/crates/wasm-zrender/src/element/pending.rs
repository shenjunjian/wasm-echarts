//! 未挂载图元的 pending 数据（挂载 ZRender 时再写入 Storage）

use std::collections::HashMap;

use crate::bridge::opts::DraggableKind;
use rust_zrender::{
    DisplayableProps, EcData, ImageStyle, PathStyle, PathStylePatch, Shape, TextStyle,
};
use rust_zrender::element::ElementBase;

#[derive(Debug, Clone)]
pub struct PendingTransform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
    pub origin_x: f64,
    pub origin_y: f64,
}

impl Default for PendingTransform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }
}

impl PendingTransform {
    pub fn set_field(&mut self, key: &str, value: f64) {
        match key {
            "x" => self.x = value,
            "y" => self.y = value,
            "scaleX" => self.scale_x = value,
            "scaleY" => self.scale_y = value,
            "rotation" => self.rotation = value,
            "originX" => self.origin_x = value,
            "originY" => self.origin_y = value,
            _ => {}
        }
    }

    pub fn apply_to_base(&self, base: &mut ElementBase) {
        base.apply_transform_props(
            self.x,
            self.y,
            self.scale_x,
            self.scale_y,
            self.rotation,
            self.origin_x,
            self.origin_y,
        );
    }
}

#[derive(Debug, Clone, Default)]
pub struct PendingGroup {
    pub transform: PendingTransform,
    pub name: String,
    pub ignore: bool,
    pub silent: bool,
    pub draggable: DraggableKind,
}

#[derive(Debug, Clone)]
pub struct PendingPath {
    pub shape: Shape,
    pub style: PathStyle,
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: String,
    pub ignore: bool,
    pub ec_data: EcData,
    pub state_patches: HashMap<String, PathStylePatch>,
    pub active_states: Vec<String>,
    pub transform: PendingTransform,
    pub clip_element_id: Option<u32>,
    pub draggable: DraggableKind,
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
    pub ignore: bool,
    pub ec_data: EcData,
    pub transform: PendingTransform,
    pub draggable: DraggableKind,
}

#[derive(Debug, Clone)]
pub struct PendingImage {
    pub style: ImageStyle,
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: String,
    pub ignore: bool,
    pub ec_data: EcData,
    pub transform: PendingTransform,
    pub draggable: DraggableKind,
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

    pub fn draggable(&self) -> DraggableKind {
        match self {
            Self::Group(g) => g.draggable,
            Self::Path(p) => p.draggable,
            Self::Text(t) => t.draggable,
            Self::Image(p) => p.draggable,
        }
    }

    pub fn transform(&self) -> &PendingTransform {
        match self {
            Self::Group(g) => &g.transform,
            Self::Path(p) => &p.transform,
            Self::Text(t) => &t.transform,
            Self::Image(p) => &p.transform,
        }
    }

    pub fn transform_mut(&mut self) -> &mut PendingTransform {
        match self {
            Self::Group(g) => &mut g.transform,
            Self::Path(p) => &mut p.transform,
            Self::Text(t) => &mut t.transform,
            Self::Image(p) => &mut p.transform,
        }
    }

    pub fn position(&self) -> (f64, f64) {
        let t = self.transform();
        (t.x, t.y)
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        let t = self.transform_mut();
        t.x = x;
        t.y = y;
    }

    pub fn set_name(&mut self, name: String) {
        match self {
            Self::Group(g) => g.name = name,
            Self::Path(p) => p.name = name,
            Self::Text(t) => t.name = name,
            Self::Image(p) => p.name = name,
        }
    }

    pub fn set_ignore(&mut self, ignore: bool) {
        match self {
            Self::Group(g) => g.ignore = ignore,
            Self::Path(p) => p.ignore = ignore,
            Self::Text(t) => t.ignore = ignore,
            Self::Image(p) => p.ignore = ignore,
        }
    }

    pub fn set_silent(&mut self, silent: bool) {
        match self {
            Self::Group(g) => g.silent = silent,
            Self::Path(p) => p.silent = silent,
            Self::Text(t) => t.silent = silent,
            Self::Image(p) => p.silent = silent,
        }
    }

    pub fn displayable_mut(&mut self) -> Option<&mut DisplayableProps> {
        match self {
            Self::Group(_) => None,
            Self::Path(p) => Some(&mut p.displayable),
            Self::Text(t) => Some(&mut t.displayable),
            Self::Image(p) => Some(&mut p.displayable),
        }
    }
}
