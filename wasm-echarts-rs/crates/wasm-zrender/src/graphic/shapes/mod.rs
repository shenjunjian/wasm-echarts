//! 基础 shape 定义与 buildPath

pub mod circle;
pub mod line;
pub mod polygon;
pub mod polyline;
pub mod rect;

use crate::graphic::path_proxy::PathProxy;

pub use circle::{CircleShape, build_circle_path};
pub use line::{LineShape, build_line_path};
pub use polygon::{PolygonShape, build_polygon_path};
pub use polyline::{PolylineShape, build_polyline_path};
pub use rect::{RectShape, build_rect_path};

#[derive(Debug, Clone)]
pub enum Shape {
    Rect(RectShape),
    Circle(CircleShape),
    Line(LineShape),
    Polygon(PolygonShape),
    Polyline(PolylineShape),
}

impl Shape {
    pub fn build_path(&self, ctx: &mut PathProxy) {
        match self {
            Shape::Rect(s) => build_rect_path(ctx, s),
            Shape::Circle(s) => build_circle_path(ctx, s),
            Shape::Line(s) => build_line_path(ctx, s),
            Shape::Polygon(s) => build_polygon_path(ctx, s, true),
            Shape::Polyline(s) => build_polyline_path(ctx, s),
        }
    }
}
