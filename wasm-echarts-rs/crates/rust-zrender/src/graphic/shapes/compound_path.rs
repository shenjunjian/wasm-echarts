//! 复合路径（多 subpath）

use super::Shape;
use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct CompoundPathShape {
    pub shapes: Vec<Shape>,
}

pub fn build_compound_path(ctx: &mut PathProxy, shape: &CompoundPathShape) {
    for sub in &shape.shapes {
        sub.build_path(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic::shapes::PathDataShape;

    #[test]
    fn compound_path_merges_subpaths() {
        let mut proxy = PathProxy::new();
        build_compound_path(
            &mut proxy,
            &CompoundPathShape {
                shapes: vec![
                    Shape::PathData(PathDataShape {
                        path_data: "M 0 0 L 50 0 L 50 50 Z".into(),
                    }),
                    Shape::PathData(PathDataShape {
                        path_data: "M 60 60 L 90 60 L 75 90 Z".into(),
                    }),
                ],
            },
        );
        assert!(!proxy.is_empty());
    }
}
