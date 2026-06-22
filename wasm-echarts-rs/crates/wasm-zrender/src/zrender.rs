//! ZRender 顶层 API：Storage + Painter

use crate::canvas::backend::{BackendError, VlConvertBackend};
use crate::canvas::painter::Painter;
use crate::core::types::RgbaBuffer;
use crate::storage::Storage;

pub struct ZRenderer {
    pub storage: Storage,
    painter: Painter<VlConvertBackend>,
    width: u32,
    height: u32,
}

impl ZRenderer {
    pub fn new(width: u32, height: u32) -> Result<Self, BackendError> {
        let backend = VlConvertBackend::new(width, height)?;
        Ok(Self {
            storage: Storage::new(),
            painter: Painter::new(backend, width, height),
            width,
            height,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn refresh(&mut self) -> Result<RgbaBuffer, BackendError> {
        self.painter.refresh(&mut self.storage)
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), BackendError> {
        self.width = width;
        self.height = height;
        let backend = VlConvertBackend::new(width, height)?;
        self.painter = Painter::new(backend, width, height);
        self.storage.mark_all_dirty();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic::displayable::DisplayableProps;
    use crate::graphic::group::ChildRef;
    use crate::graphic::path::Path;
    use crate::graphic::shapes::{CircleShape, LineShape, PolygonShape, RectShape, Shape};
    use crate::graphic::style::PathStyle;

    fn build_test_scene(storage: &mut Storage) {
        let group = storage.create_group();

        let rect = storage.create_path(Path::new(
            Shape::Rect(RectShape {
                x: 20.0,
                y: 20.0,
                width: 100.0,
                height: 60.0,
            }),
            PathStyle {
                fill: Some("#5470c6".to_string()),
                ..Default::default()
            },
        ));

        let circle = storage.create_path(
            Path::new(
                Shape::Circle(CircleShape {
                    cx: 180.0,
                    cy: 80.0,
                    r: 40.0,
                }),
                PathStyle {
                    fill: Some("rgba(145, 204, 117, 0.8)".to_string()),
                    stroke: Some("#ee6666".to_string()),
                    line_width: 3.0,
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                z: 1.0,
                ..Default::default()
            }),
        );

        let line = storage.create_path(Path::new(
            Shape::Line(LineShape {
                x1: 20.0,
                y1: 120.0,
                x2: 280.0,
                y2: 120.0,
                percent: 1.0,
            }),
            PathStyle {
                fill: None,
                stroke: Some("#333".to_string()),
                line_width: 2.0,
                ..Default::default()
            },
        ));

        let polygon = storage.create_path(Path::new(
            Shape::Polygon(PolygonShape {
                points: vec![(240.0, 30.0), (300.0, 60.0), (270.0, 100.0)],
            }),
            PathStyle {
                fill: Some("#fac858".to_string()),
                ..Default::default()
            },
        ));

        storage.group_add_child(group, ChildRef::Path(rect));
        storage.group_add_child(group, ChildRef::Path(circle));
        storage.group_add_child(group, ChildRef::Path(line));
        storage.group_add_child(group, ChildRef::Path(polygon));
        storage.add_root(ChildRef::Group(group));
    }

    #[test]
    fn storage_painter_refresh_outputs_rgba() {
        let mut zr = ZRenderer::new(320, 160).unwrap();
        build_test_scene(&mut zr.storage);
        let rgba = zr.refresh().unwrap();
        assert_eq!(rgba.len(), 320 * 160 * 4);
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }
}
