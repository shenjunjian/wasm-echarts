//! 基础 shape 定义与 buildPath

pub mod arc;
pub mod bezier_curve;
pub mod circle;
pub mod ellipse;
pub mod line;
pub mod polygon;
pub mod polyline;
pub mod rect;
pub mod ring;
pub mod sector;

use crate::graphic::path_proxy::PathProxy;

pub use arc::{ArcShape, build_arc_path};
pub use bezier_curve::{BezierCurveShape, build_bezier_curve_path};
pub use circle::{CircleShape, build_circle_path};
pub use ellipse::{EllipseShape, build_ellipse_path};
pub use line::{LineShape, build_line_path};
pub use polygon::{PolygonShape, build_polygon_path};
pub use polyline::{PolylineShape, build_polyline_path};
pub use rect::{RectShape, build_rect_path};
pub use ring::{RingShape, build_ring_path};
pub use sector::{SectorShape, build_sector_path};

#[derive(Debug, Clone)]
pub enum Shape {
    Rect(RectShape),
    Circle(CircleShape),
    Line(LineShape),
    Polygon(PolygonShape),
    Polyline(PolylineShape),
    Sector(SectorShape),
    Arc(ArcShape),
    Ellipse(EllipseShape),
    Ring(RingShape),
    BezierCurve(BezierCurveShape),
}

impl Shape {
    pub fn build_path(&self, ctx: &mut PathProxy) {
        match self {
            Shape::Rect(s) => build_rect_path(ctx, s),
            Shape::Circle(s) => build_circle_path(ctx, s),
            Shape::Line(s) => build_line_path(ctx, s),
            Shape::Polygon(s) => build_polygon_path(ctx, s, true),
            Shape::Polyline(s) => build_polyline_path(ctx, s),
            Shape::Sector(s) => build_sector_path(ctx, s),
            Shape::Arc(s) => build_arc_path(ctx, s),
            Shape::Ellipse(s) => build_ellipse_path(ctx, s),
            Shape::Ring(s) => build_ring_path(ctx, s),
            Shape::BezierCurve(s) => build_bezier_curve_path(ctx, s),
        }
    }
}
