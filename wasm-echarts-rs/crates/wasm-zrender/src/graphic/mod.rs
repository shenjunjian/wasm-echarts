//! 图元 wasm 导出

mod group;
pub(crate) mod gradient;
mod displayable;
mod geometry_util;
mod bounding_rect;
mod image;
mod oriented_bounding_rect;
mod path;
mod point;
mod shapes;
mod stub;
mod text;
mod tspan;

pub use group::Group;
pub use gradient::{LinearGradient, Pattern, RadialGradient};
pub use displayable::Displayable;
pub use bounding_rect::BoundingRect;
pub use image::Image;
pub use oriented_bounding_rect::OrientedBoundingRect;
pub use path::Path;
pub use point::Point;
pub use shapes::{
    Arc, BezierCurve, Circle, CompoundPath, Droplet, Ellipse, Heart, Isogon, Line, Polygon,
    Polyline, Rect, Ring, Rose, Sector, Star, Trochoid,
};
pub use stub::*;
pub use text::Text;
pub use tspan::TSpan;
