use crate::data::numeric_or_time;
use crate::option::OptionValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisType {
    Category,
    Value,
    Time,
    Log,
}

impl AxisType {
    pub fn from_str(s: &str, fallback: AxisType) -> Self {
        match s {
            "category" => AxisType::Category,
            "value" => AxisType::Value,
            "time" => AxisType::Time,
            "log" => AxisType::Log,
            _ => fallback,
        }
    }

    pub fn is_category(self) -> bool {
        self == AxisType::Category
    }

    pub fn is_continuous(self) -> bool {
        matches!(self, AxisType::Value | AxisType::Time | AxisType::Log)
    }
}

/// `axis.breaks[]`：未展开的区间在比例尺上折叠为 `gap`。
#[derive(Debug, Clone)]
pub struct AxisBreak {
    pub start: f64,
    pub end: f64,
    pub gap_abs: Option<f64>,
    pub gap_percent: Option<f64>,
    pub is_expanded: bool,
}

impl AxisBreak {
    pub fn span(&self) -> f64 {
        (self.end - self.start).abs()
    }

    pub fn contains(&self, value: f64) -> bool {
        value > self.start && value < self.end
    }

    pub fn gap_in_span(&self) -> f64 {
        let span = self.span();
        if let Some(pct) = self.gap_percent {
            return (pct.clamp(0.0, 1.0) * span).min(span);
        }
        if let Some(abs) = self.gap_abs {
            return abs.clamp(0.0, span);
        }
        0.0
    }
}

#[derive(Debug, Clone)]
pub struct AxisModel {
    pub axis_type: AxisType,
    pub category_data: Vec<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub grid_index: usize,
    pub log_base: f64,
    pub breaks: Vec<AxisBreak>,
    pub jitter: f64,
    pub jitter_overlap: bool,
    pub jitter_margin: f64,
}

impl Default for AxisModel {
    fn default() -> Self {
        Self {
            axis_type: AxisType::Value,
            category_data: Vec::new(),
            min: None,
            max: None,
            grid_index: 0,
            log_base: 10.0,
            breaks: Vec::new(),
            jitter: 0.0,
            jitter_overlap: false,
            jitter_margin: 0.0,
        }
    }
}

impl AxisModel {
    pub fn with_data_range(mut self, (min, max): (f64, f64)) -> Self {
        if self.min.is_none() {
            self.min = Some(min);
        }
        if self.max.is_none() {
            self.max = Some(max);
        }
        self
    }

    pub fn value_min(&self) -> f64 {
        self.min.unwrap_or(0.0)
    }

    pub fn value_max(&self) -> f64 {
        self.max.unwrap_or(1.0)
    }

    pub fn active_breaks(&self) -> Vec<&AxisBreak> {
        let mut list: Vec<&AxisBreak> = self
            .breaks
            .iter()
            .filter(|b| !b.is_expanded && b.end > b.start)
            .collect();
        list.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap_or(std::cmp::Ordering::Equal));
        list
    }

    pub fn has_breaks(&self) -> bool {
        !self.active_breaks().is_empty()
    }

    pub fn is_inside_break(&self, value: f64) -> bool {
        self.active_breaks().iter().any(|b| b.contains(value))
    }

    /// 把原始值映射到折叠后的线性空间（log 轴先取对数）。
    pub fn elapse(&self, value: f64) -> f64 {
        let v = self.to_inner(value);
        let mut offset = 0.0;
        for brk in self.active_breaks() {
            let a = self.to_inner(brk.start);
            let b = self.to_inner(brk.end);
            let gap = self.gap_inner(brk, a, b);
            if v <= a {
                return v - offset;
            }
            if v < b {
                let t = (v - a) / (b - a).abs().max(f64::EPSILON);
                return a - offset + t * gap;
            }
            offset += (b - a) - gap;
        }
        v - offset
    }

    pub fn unelapse(&self, elapsed: f64) -> f64 {
        let mut offset = 0.0;
        for brk in self.active_breaks() {
            let a = self.to_inner(brk.start);
            let b = self.to_inner(brk.end);
            let gap = self.gap_inner(brk, a, b);
            let start_e = a - offset;
            let end_e = start_e + gap;
            if elapsed <= start_e {
                return self.from_inner(elapsed + offset);
            }
            if elapsed < end_e {
                let t = (elapsed - start_e) / gap.max(f64::EPSILON);
                return self.from_inner(a + t * (b - a));
            }
            offset += (b - a) - gap;
        }
        self.from_inner(elapsed + offset)
    }

    pub fn elapsed_min(&self) -> f64 {
        self.elapse(self.value_min())
    }

    pub fn elapsed_max(&self) -> f64 {
        self.elapse(self.value_max())
    }

    fn to_inner(&self, value: f64) -> f64 {
        match self.axis_type {
            AxisType::Log => {
                let base = self.log_base.max(1.000_000_1);
                value.max(f64::MIN_POSITIVE).log(base)
            }
            _ => value,
        }
    }

    fn from_inner(&self, value: f64) -> f64 {
        match self.axis_type {
            AxisType::Log => {
                let base = self.log_base.max(1.000_000_1);
                base.powf(value)
            }
            _ => value,
        }
    }

    fn gap_inner(&self, brk: &AxisBreak, a: f64, b: f64) -> f64 {
        let span = (b - a).abs();
        if let Some(pct) = brk.gap_percent {
            return (pct.clamp(0.0, 1.0) * span).min(span);
        }
        if let Some(abs) = brk.gap_abs {
            match self.axis_type {
                AxisType::Log => {
                    let g = self.to_inner(brk.start + abs) - a;
                    g.clamp(0.0, span)
                }
                _ => abs.clamp(0.0, span),
            }
        } else {
            0.0
        }
    }
}

pub fn parse_axis_breaks(comp: &OptionValue) -> Vec<AxisBreak> {
    let Some(arr) = comp.get("breaks").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in arr {
        let start = item.get("start").and_then(numeric_or_time);
        let end = item.get("end").and_then(numeric_or_time);
        let (Some(start), Some(end)) = (start, end) else {
            continue;
        };
        let (start, end) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        let (gap_abs, gap_percent) = parse_break_gap(item.get("gap"));
        out.push(AxisBreak {
            start,
            end,
            gap_abs,
            gap_percent,
            is_expanded: item.get("isExpanded").and_then(|v| v.as_bool()).unwrap_or(false),
        });
    }
    out
}

fn parse_break_gap(value: Option<&OptionValue>) -> (Option<f64>, Option<f64>) {
    let Some(value) = value else {
        return (None, None);
    };
    if let Some(n) = value.as_f64() {
        return (Some(n.max(0.0)), None);
    }
    if let Some(s) = value.as_str() {
        let t = s.trim();
        if let Some(num) = t.strip_suffix('%') {
            if let Ok(p) = num.trim().parse::<f64>() {
                return (None, Some((p / 100.0).clamp(0.0, 1.0)));
            }
        }
        if let Ok(n) = t.parse::<f64>() {
            return (Some(n.max(0.0)), None);
        }
    }
    (None, None)
}

pub fn parse_axis_jitter(comp: &OptionValue) -> (f64, bool, f64) {
    let jitter = comp
        .get("jitter")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
        .max(0.0);
    let overlap = comp
        .get("jitterOverlap")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let margin = comp
        .get("jitterMargin")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
        .max(0.0);
    (jitter, overlap, margin)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn axis_with_break(start: f64, end: f64) -> AxisModel {
        AxisModel {
            axis_type: AxisType::Value,
            min: Some(0.0),
            max: Some(100.0),
            breaks: vec![AxisBreak {
                start,
                end,
                gap_abs: Some(0.0),
                gap_percent: None,
                is_expanded: false,
            }],
            ..Default::default()
        }
    }

    #[test]
    fn elapse_collapses_break_span() {
        let axis = axis_with_break(20.0, 80.0);
        assert!((axis.elapse(10.0) - 10.0).abs() < 1e-9);
        assert!((axis.elapse(20.0) - 20.0).abs() < 1e-9);
        assert!((axis.elapse(90.0) - 30.0).abs() < 1e-9);
        assert!((axis.unelapse(30.0) - 90.0).abs() < 1e-9);
        assert!((axis.unelapse(10.0) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn expanded_break_is_ignored() {
        let mut axis = axis_with_break(20.0, 80.0);
        axis.breaks[0].is_expanded = true;
        assert!((axis.elapse(90.0) - 90.0).abs() < 1e-9);
    }
}
