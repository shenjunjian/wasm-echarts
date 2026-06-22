//! 图元：Path、Group、样式

pub mod displayable;
pub mod group;
pub mod path;
pub mod path_proxy;
pub mod shapes;
pub mod style;

pub use displayable::{DisplayableProps, displayable_compare, normalize_z};
pub use group::{ChildRef, Group};
pub use path::Path;
pub use path_proxy::PathProxy;
pub use shapes::*;
pub use style::PathStyle;
