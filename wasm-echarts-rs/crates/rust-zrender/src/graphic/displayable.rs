//! Displayable 公共属性

use crate::core::bbox::BoundingRect;

#[derive(Debug, Clone)]
pub struct DisplayableProps {
    pub z: f64,
    pub z2: f64,
    pub zlevel: f64,
    pub invisible: bool,
    pub culling: bool,
}

impl Default for DisplayableProps {
    fn default() -> Self {
        Self {
            z: 0.0,
            z2: 0.0,
            zlevel: 0.0,
            invisible: false,
            culling: false,
        }
    }
}

impl DisplayableProps {
    pub fn should_be_painted(
        &self,
        ignore: bool,
        opacity: f32,
        transform: &[f32; 6],
        view_width: f64,
        view_height: f64,
        bbox: Option<&BoundingRect>,
    ) -> bool {
        if ignore || self.invisible || opacity <= 0.0 {
            return false;
        }
        if transform[0] == 0.0 && transform[3] == 0.0 {
            return false;
        }
        if self.culling {
            if let Some(rect) = bbox {
                if !rect.intersects_viewport(view_width, view_height) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn normalize_z(props: &mut DisplayableProps) {
    if props.z.is_nan() {
        props.z = 0.0;
    }
    if props.z2.is_nan() {
        props.z2 = 0.0;
    }
    if props.zlevel.is_nan() {
        props.zlevel = 0.0;
    }
}

pub fn displayable_compare(a: &DisplayableProps, b: &DisplayableProps) -> std::cmp::Ordering {
    a.zlevel
        .partial_cmp(&b.zlevel)
        .unwrap_or(std::cmp::Ordering::Equal)
        .then_with(|| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal))
        .then_with(|| a.z2.partial_cmp(&b.z2).unwrap_or(std::cmp::Ordering::Equal))
}
