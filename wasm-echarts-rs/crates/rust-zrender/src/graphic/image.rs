//! Image 图元

use std::sync::Arc;

use crate::core::bbox::BoundingRect;
use crate::element::{EcData, ElementBase};
use crate::graphic::displayable::DisplayableProps;

#[derive(Debug, Clone)]
pub struct ImageStyle {
    pub x: f64,
    pub y: f64,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub sx: f64,
    pub sy: f64,
    pub s_width: Option<f64>,
    pub s_height: Option<f64>,
    pub opacity: f32,
    pub data: Arc<[u8]>,
    pub source_width: u32,
    pub source_height: u32,
}

impl Default for ImageStyle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: None,
            height: None,
            sx: 0.0,
            sy: 0.0,
            s_width: None,
            s_height: None,
            opacity: 1.0,
            data: Arc::from([]),
            source_width: 0,
            source_height: 0,
        }
    }
}

impl ImageStyle {
    pub fn has_pixels(&self) -> bool {
        self.source_width > 0 && self.source_height > 0 && !self.data.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub base: ElementBase,
    pub displayable: DisplayableProps,
    pub style: ImageStyle,
    pub ec_data: EcData,
    pub silent: bool,
}

impl Image {
    pub fn new(style: ImageStyle) -> Self {
        Self {
            base: ElementBase::default(),
            displayable: DisplayableProps::default(),
            style,
            ec_data: EcData::default(),
            silent: false,
        }
    }

    pub fn with_displayable(mut self, displayable: DisplayableProps) -> Self {
        self.displayable = displayable;
        self
    }

    pub fn with_ec_data(mut self, ec_data: EcData) -> Self {
        self.ec_data = ec_data;
        self
    }

    pub fn draw_width(&self) -> f64 {
        if let Some(w) = self.style.width {
            return w;
        }
        if let Some(h) = self.style.height {
            if self.style.source_height > 0 {
                return h * self.style.source_width as f64 / self.style.source_height as f64;
            }
        }
        self.style.source_width as f64
    }

    pub fn draw_height(&self) -> f64 {
        if let Some(h) = self.style.height {
            return h;
        }
        if let Some(w) = self.style.width {
            if self.style.source_width > 0 {
                return w * self.style.source_height as f64 / self.style.source_width as f64;
            }
        }
        self.style.source_height as f64
    }

    pub fn bounding_rect(&self) -> BoundingRect {
        BoundingRect::new(self.style.x, self.style.y, self.draw_width(), self.draw_height())
    }

    pub fn should_be_painted(&self, view_width: f64, view_height: f64) -> bool {
        if !self.style.has_pixels() {
            return false;
        }
        let bbox = self.bounding_rect();
        self.displayable.should_be_painted(
            self.base.ignore,
            self.style.opacity,
            self.base.transform(),
            view_width,
            view_height,
            Some(&bbox),
        )
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        if let Some((lx, ly)) = invert_transform_point(self.base.transform(), x, y) {
            let rect = self.bounding_rect();
            lx >= rect.x
                && ly >= rect.y
                && lx <= rect.x + rect.width
                && ly <= rect.y + rect.height
        } else {
            false
        }
    }

    pub fn hit_test(&self, x: f64, y: f64) -> bool {
        self.contains(x, y)
    }
}

fn invert_transform_point(m: &[f32; 6], x: f64, y: f64) -> Option<(f64, f64)> {
    let det = m[0] as f64 * m[3] as f64 - m[2] as f64 * m[1] as f64;
    if det.abs() < 1e-12 {
        return None;
    }
    let inv_det = 1.0 / det;
    let tx = x - m[4] as f64;
    let ty = y - m[5] as f64;
    Some((
        (m[3] as f64 * tx - m[2] as f64 * ty) * inv_det,
        (-m[1] as f64 * tx + m[0] as f64 * ty) * inv_det,
    ))
}
