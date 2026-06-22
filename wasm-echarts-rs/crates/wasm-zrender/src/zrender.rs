//! ZRender 顶层 API：Storage + Painter + Handler

use crate::canvas::backend::{BackendError, VlConvertBackend};
use crate::canvas::painter::Painter;
use crate::core::types::RgbaBuffer;
use crate::element::{EcData, PathStylePatch};
use crate::handler::{Handler, HitResult};
use crate::storage::Storage;

pub struct ZRenderer {
    pub storage: Storage,
    painter: Painter<VlConvertBackend>,
    width: u32,
    height: u32,
    dpr: f64,
}

impl ZRenderer {
    pub fn new(width: u32, height: u32) -> Result<Self, BackendError> {
        Self::new_with_dpr(width, height, 1.0)
    }

    pub fn new_with_dpr(width: u32, height: u32, dpr: f64) -> Result<Self, BackendError> {
        let backend = VlConvertBackend::new(width, height)?;
        Ok(Self {
            storage: Storage::new(),
            painter: Painter::new(backend, width, height),
            width,
            height,
            dpr,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dpr(&self) -> f64 {
        self.dpr
    }

    pub fn refresh(&mut self) -> Result<RgbaBuffer, BackendError> {
        self.painter.refresh(&mut self.storage)
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), BackendError> {
        self.resize_with_dpr(width, height, self.dpr)
    }

    pub fn resize_with_dpr(&mut self, width: u32, height: u32, dpr: f64) -> Result<(), BackendError> {
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        let backend = VlConvertBackend::new(width, height)?;
        self.painter = Painter::new(backend, width, height);
        self.storage.mark_all_dirty();
        Ok(())
    }

    /// 命中检测（反向遍历 displayList）
    pub fn find_hover(&mut self, x: f64, y: f64) -> Option<HitResult> {
        Handler::find_hover(&mut self.storage, x, y)
    }

    /// 切换图元状态（emphasis / select / normal）
    pub fn set_path_state(&mut self, path_index: usize, state: &str) {
        let path = self.storage.path_mut(path_index);
        path.use_state(state);
    }

    /// 注册状态样式补丁
    pub fn set_path_state_style(
        &mut self,
        path_index: usize,
        state: &str,
        patch: PathStylePatch,
    ) {
        let path = self.storage.path_mut(path_index);
        path.states.set_state_patch(state, patch);
        if path.states.current.iter().any(|s| s == state) {
            path.use_state(state);
        } else {
            path.base.mark_redraw();
        }
    }

    /// 设置 ECData 元数据
    pub fn set_path_ec_data(&mut self, path_index: usize, ec_data: EcData) {
        self.storage.path_mut(path_index).ec_data = ec_data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{EcData, PathStylePatch, STATE_EMPHASIS, STATE_NORMAL};
    use crate::graphic::displayable::DisplayableProps;
    use crate::graphic::group::ChildRef;
    use crate::graphic::path::Path;
    use crate::graphic::shapes::{CircleShape, LineShape, PolygonShape, RectShape, Shape};
    use crate::graphic::style::{FillStrokeStyle, PathStyle};

    fn build_test_scene(storage: &mut Storage) {
        let group = storage.create_group();

        let rect = storage.create_path(
            Path::new(
                Shape::Rect(RectShape {
                    x: 20.0,
                    y: 20.0,
                    width: 100.0,
                    height: 60.0,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color("#5470c6"),
                    ..Default::default()
                },
            )
            .with_ec_data(EcData::new(0, 0)),
        );

        let circle = storage.create_path(
            Path::new(
                Shape::Circle(CircleShape {
                    cx: 180.0,
                    cy: 80.0,
                    r: 40.0,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color("rgba(145, 204, 117, 0.8)"),
                    stroke: FillStrokeStyle::color("#ee6666"),
                    line_width: 3.0,
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                z: 1.0,
                ..Default::default()
            })
            .with_ec_data(EcData::new(0, 1)),
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
                fill: FillStrokeStyle::none(),
                stroke: FillStrokeStyle::color("#333"),
                line_width: 2.0,
                line_dash: Some(vec![6.0, 4.0]),
                ..Default::default()
            },
        ));

        let polygon = storage.create_path(Path::new(
            Shape::Polygon(PolygonShape {
                points: vec![(240.0, 30.0), (300.0, 60.0), (270.0, 100.0)],
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#fac858"),
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

    #[test]
    fn path_contains_hit_test() {
        let mut zr = ZRenderer::new(320, 160).unwrap();
        build_test_scene(&mut zr.storage);
        let rect_path = 0usize;
        let path = zr.storage.path_mut(rect_path);
        assert!(path.contains(70.0, 50.0));
        assert!(!path.contains(5.0, 5.0));
    }

    #[test]
    fn find_hover_returns_ec_data() {
        let mut zr = ZRenderer::new(320, 160).unwrap();
        build_test_scene(&mut zr.storage);
        let hit = zr.find_hover(180.0, 80.0).unwrap();
        assert_eq!(hit.ec_data.series_index, Some(0));
        assert_eq!(hit.ec_data.data_index, Some(1));
        assert!(!hit.silent);
    }

    #[test]
    fn emphasis_state_changes_style() {
        let mut zr = ZRenderer::new(200, 100).unwrap();
        let idx = zr.storage.create_path(Path::new(
            Shape::Rect(RectShape {
                x: 10.0,
                y: 10.0,
                width: 80.0,
                height: 50.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#5470c6"),
                ..Default::default()
            },
        ));
        zr.storage.add_root(ChildRef::Path(idx));

        zr.set_path_state_style(
            idx,
            STATE_EMPHASIS,
            PathStylePatch {
                fill: Some(FillStrokeStyle::color("#ee6666")),
                line_width: Some(4.0),
                ..Default::default()
            },
        );
        zr.set_path_state(idx, STATE_EMPHASIS);
        assert!(zr.storage.path(idx).states.is_emphasis());

        zr.set_path_state(idx, STATE_NORMAL);
        assert!(!zr.storage.path(idx).states.is_emphasis());
    }

    #[test]
    fn multi_zlevel_refresh() {
        let mut zr = ZRenderer::new(100, 100).unwrap();
        let bg = zr.storage.create_path(
            Path::new(
                Shape::Rect(RectShape {
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 100.0,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color("#5470c6"),
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                zlevel: 0.0,
                ..Default::default()
            }),
        );
        let fg = zr.storage.create_path(
            Path::new(
                Shape::Circle(CircleShape {
                    cx: 50.0,
                    cy: 50.0,
                    r: 20.0,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color("#ee6666"),
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                zlevel: 1.0,
                ..Default::default()
            }),
        );
        zr.storage.add_root(ChildRef::Path(bg));
        zr.storage.add_root(ChildRef::Path(fg));
        let rgba = zr.refresh().unwrap();
        assert!(rgba.chunks(4).any(|px| px[0] > 200));
    }
}
