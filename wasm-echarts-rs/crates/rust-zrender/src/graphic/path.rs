//! Path 图元

use crate::core::bbox::BoundingRect;
use crate::element::{EcData, ElementBase, ElementStates, SHAPE_CHANGED_BIT};
use crate::graphic::displayable::DisplayableProps;
use crate::graphic::path_proxy::PathProxy;
use crate::graphic::shapes::Shape;
use crate::graphic::style::PathStyle;

#[derive(Debug, Clone)]
pub struct Path {
    pub base: ElementBase,
    pub displayable: DisplayableProps,
    pub shape: Shape,
    pub style: PathStyle,
    pub ec_data: EcData,
    pub silent: bool,
    /// clipPath 引用（paths 数组中的索引）
    pub clip_path: Option<usize>,
    pub states: ElementStates,
    path_proxy: PathProxy,
    bbox: Option<BoundingRect>,
}

impl Path {
    pub fn new(shape: Shape, style: PathStyle) -> Self {
        Self {
            base: ElementBase::default(),
            displayable: DisplayableProps::default(),
            shape,
            style,
            ec_data: EcData::default(),
            silent: false,
            clip_path: None,
            states: ElementStates::default(),
            path_proxy: PathProxy::new(),
            bbox: None,
        }
    }

    pub fn with_displayable(mut self, displayable: DisplayableProps) -> Self {
        self.displayable = displayable;
        self
    }

    pub fn with_ec_data(mut self, ec_data: EcData) -> Self {
        self.ec_data = ec_data;
        self
    }

    pub fn with_clip_path(mut self, clip_index: usize) -> Self {
        self.clip_path = Some(clip_index);
        self
    }

    pub fn with_transform(mut self, x: f64, y: f64) -> Self {
        self.base.transform_state.x = x;
        self.base.transform_state.y = y;
        self.base.mark_redraw();
        self
    }

    pub fn use_state(&mut self, state_name: &str) {
        self.states.use_state(&mut self.style, state_name);
        self.base.mark_redraw();
    }

    pub fn use_states(&mut self, state_names: &[&str]) {
        self.states.use_states(&mut self.style, state_names);
        self.base.mark_redraw();
    }

    pub fn path_proxy(&self) -> &PathProxy {
        &self.path_proxy
    }

    pub fn rebuild_path(&mut self) {
        self.path_proxy.begin_path();
        self.shape.build_path(&mut self.path_proxy);
        self.base.dirty &= !SHAPE_CHANGED_BIT;
        self.bbox = Some(estimate_bbox(&self.shape));
    }

    pub fn ensure_path(&mut self) {
        if self.path_proxy.is_empty() || self.base.dirty & SHAPE_CHANGED_BIT != 0 {
            self.rebuild_path();
        }
    }

    pub fn bounding_rect(&mut self) -> Option<&BoundingRect> {
        self.ensure_path();
        self.bbox.as_ref()
    }

    pub fn should_be_painted(&mut self, view_width: f64, view_height: f64) -> bool {
        let bbox = self.bounding_rect().cloned();
        let transform = *self.base.transform();
        self.displayable.should_be_painted(
            self.base.ignore,
            self.style.opacity,
            &transform,
            view_width,
            view_height,
            bbox.as_ref(),
        )
    }

    /// 填充命中检测（全局坐标）
    pub fn contains(&mut self, x: f64, y: f64) -> bool {
        self.ensure_path();
        crate::contain::contain_with_transform(
            self.path_proxy(),
            self.base.transform(),
            x,
            y,
        )
    }

    /// 描边命中检测（全局坐标）
    pub fn contains_stroke(&mut self, x: f64, y: f64) -> bool {
        self.ensure_path();
        crate::contain::contain_stroke_with_transform(
            self.path_proxy(),
            self.base.transform(),
            self.style.line_width,
            x,
            y,
        )
    }

    /// 几何命中：填充或描边
    pub fn hit_test(&mut self, x: f64, y: f64) -> bool {
        if self.style.has_fill() && self.contains(x, y) {
            return true;
        }
        if self.style.has_stroke() && self.contains_stroke(x, y) {
            return true;
        }
        false
    }
}

fn estimate_bbox(shape: &Shape) -> BoundingRect {
    match shape {
        Shape::Rect(s) => BoundingRect::new(s.x, s.y, s.width, s.height),
        Shape::Circle(s) => BoundingRect::new(s.cx - s.r, s.cy - s.r, s.r * 2.0, s.r * 2.0),
        Shape::Line(s) => {
            let min_x = s.x1.min(s.x2);
            let min_y = s.y1.min(s.y2);
            let max_x = s.x1.max(s.x2);
            let max_y = s.y1.max(s.y2);
            BoundingRect::new(min_x, min_y, max_x - min_x, max_y - min_y)
        }
        Shape::Polygon(s) => bbox_from_points(&s.points),
        Shape::Polyline(s) => bbox_from_points(&s.points),
        Shape::Sector(s) => BoundingRect::new(s.cx - s.r, s.cy - s.r, s.r * 2.0, s.r * 2.0),
        Shape::Arc(s) => BoundingRect::new(s.cx - s.r, s.cy - s.r, s.r * 2.0, s.r * 2.0),
        Shape::Ellipse(s) => BoundingRect::new(s.cx - s.rx, s.cy - s.ry, s.rx * 2.0, s.ry * 2.0),
        Shape::Ring(s) => {
            let outer = s.r.max(s.r0);
            BoundingRect::new(s.cx - outer, s.cy - outer, outer * 2.0, outer * 2.0)
        }
        Shape::BezierCurve(s) => bbox_bezier_curve(s),
        Shape::Isogon(s) => BoundingRect::new(s.x - s.r, s.y - s.r, s.r * 2.0, s.r * 2.0),
        Shape::Star(s) => {
            let outer = s.r.max(s.r0.unwrap_or_else(|| s.r / 3.0));
            BoundingRect::new(s.cx - outer, s.cy - outer, outer * 2.0, outer * 2.0)
        }
        Shape::Heart(s) => BoundingRect::new(
            s.cx - s.width * 2.0,
            s.cy - s.height * 2.0 / 3.0,
            s.width * 4.0,
            s.height + s.height * 2.0 / 3.0,
        ),
        Shape::Droplet(s) => BoundingRect::new(
            s.cx - s.width * 3.0 / 2.0,
            s.cy - s.height,
            s.width * 3.0,
            s.height + s.width,
        ),
        Shape::Rose(s) => {
            let max_r = s.r.iter().copied().fold(0.0_f64, f64::max);
            BoundingRect::new(s.cx - max_r, s.cy - max_r, max_r * 2.0, max_r * 2.0)
        }
        Shape::Trochoid(s) => {
            let outer = s.r + s.r0 + s.d;
            BoundingRect::new(s.cx - outer, s.cy - outer, outer * 2.0, outer * 2.0)
        }
    }
}

fn bbox_bezier_curve(s: &crate::graphic::shapes::BezierCurveShape) -> BoundingRect {
    let mut xs = vec![s.x1, s.x2, s.cpx1];
    let mut ys = vec![s.y1, s.y2, s.cpy1];
    if let Some(cpx2) = s.cpx2 {
        xs.push(cpx2);
    }
    if let Some(cpy2) = s.cpy2 {
        ys.push(cpy2);
    }
    let min_x = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let max_x = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_y = ys.iter().copied().fold(f64::INFINITY, f64::min);
    let max_y = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    BoundingRect::new(min_x, min_y, max_x - min_x, max_y - min_y)
}

fn bbox_from_points(points: &[(f64, f64)]) -> BoundingRect {
    if points.is_empty() {
        return BoundingRect::default();
    }
    let mut min_x = points[0].0;
    let mut min_y = points[0].1;
    let mut max_x = points[0].0;
    let mut max_y = points[0].1;
    for p in points {
        min_x = min_x.min(p.0);
        min_y = min_y.min(p.1);
        max_x = max_x.max(p.0);
        max_y = max_y.max(p.1);
    }
    BoundingRect::new(min_x, min_y, max_x - min_x, max_y - min_y)
}
