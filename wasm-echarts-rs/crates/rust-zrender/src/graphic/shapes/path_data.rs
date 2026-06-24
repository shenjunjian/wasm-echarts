//! 通用 SVG pathData

use kurbo::BezPath;

use crate::graphic::bezpath_util::append_bezpath;
use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct PathDataShape {
    pub path_data: String,
}

pub fn build_path_data_path(ctx: &mut PathProxy, shape: &PathDataShape) {
    if shape.path_data.is_empty() {
        return;
    }
    if let Ok(bez) = BezPath::from_svg(&shape.path_data) {
        append_bezpath(ctx, &bez);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_data_triangle_has_commands() {
        let mut proxy = PathProxy::new();
        build_path_data_path(
            &mut proxy,
            &PathDataShape {
                path_data: "M 10 10 L 100 10 L 55 80 Z".into(),
            },
        );
        assert!(!proxy.is_empty());
    }
}
