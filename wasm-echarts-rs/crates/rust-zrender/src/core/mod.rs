//! 核心工具：矩阵、包围盒、类型定义

pub mod bbox;
pub mod matrix;
pub mod obb;
pub mod point;
pub mod types;

pub use bbox::BoundingRect;
pub use obb::{IntersectOpt, OrientedBoundingRect};
pub use point::Point2;
