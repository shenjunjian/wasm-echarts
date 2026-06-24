//! 基础 shape 定义与 buildPath

pub mod arc;
pub mod bezier_curve;
pub mod circle;
pub mod compound_path;
pub mod droplet;
pub mod ellipse;
pub mod heart;
pub mod isogon;
pub mod line;
pub mod path_data;
pub mod polygon;
pub mod polyline;
pub mod rect;
pub mod ring;
pub mod rose;
pub mod sector;
pub mod star;
pub mod trochoid;

use crate::graphic::path_proxy::PathProxy;

pub use arc::{ArcShape, build_arc_path};
pub use bezier_curve::{BezierCurveShape, build_bezier_curve_path};
pub use circle::{CircleShape, build_circle_path};
pub use compound_path::{CompoundPathShape, build_compound_path};
pub use droplet::{DropletShape, build_droplet_path};
pub use ellipse::{EllipseShape, build_ellipse_path};
pub use heart::{HeartShape, build_heart_path};
pub use isogon::{IsogonShape, build_isogon_path};
pub use line::{LineShape, build_line_path};
pub use path_data::{PathDataShape, build_path_data_path};
pub use polygon::{PolygonShape, build_polygon_path};
pub use polyline::{PolylineShape, build_polyline_path};
pub use rect::{RectShape, build_rect_path};
pub use ring::{RingShape, build_ring_path};
pub use rose::{RoseShape, build_rose_path};
pub use sector::{SectorShape, build_sector_path};
pub use star::{StarShape, build_star_path};
pub use trochoid::{TrochoidShape, build_trochoid_path};

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
    Isogon(IsogonShape),
    Star(StarShape),
    Heart(HeartShape),
    Droplet(DropletShape),
    Rose(RoseShape),
    Trochoid(TrochoidShape),
    PathData(PathDataShape),
    Compound(CompoundPathShape),
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
            Shape::Isogon(s) => build_isogon_path(ctx, s),
            Shape::Star(s) => build_star_path(ctx, s),
            Shape::Heart(s) => build_heart_path(ctx, s),
            Shape::Droplet(s) => build_droplet_path(ctx, s),
            Shape::Rose(s) => build_rose_path(ctx, s),
            Shape::Trochoid(s) => build_trochoid_path(ctx, s),
            Shape::PathData(s) => build_path_data_path(ctx, s),
            Shape::Compound(s) => build_compound_path(ctx, s),
        }
    }
}
