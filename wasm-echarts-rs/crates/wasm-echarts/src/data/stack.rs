//! stack / stackStrategy：line 与 bar 共用偏移

use crate::model::{SeriesModel, SeriesType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackStrategy {
    SameSign,
    All,
    Positive,
    Negative,
}

impl StackStrategy {
    pub fn from_option(s: Option<&str>) -> Self {
        match s {
            Some("all") => StackStrategy::All,
            Some("positive") => StackStrategy::Positive,
            Some("negative") => StackStrategy::Negative,
            _ => StackStrategy::SameSign,
        }
    }

    fn allows(self, sum: f64, val: f64) -> bool {
        match self {
            StackStrategy::All => true,
            StackStrategy::Positive => val > 0.0,
            StackStrategy::Negative => val < 0.0,
            StackStrategy::SameSign => (sum >= 0.0 && val > 0.0) || (sum <= 0.0 && val < 0.0),
        }
    }
}

pub fn apply_stack(series: &mut [SeriesModel]) {
    for s in series.iter_mut() {
        for p in &mut s.data {
            p.stack_base = 0.0;
            p.stacked_value = p.value;
        }
    }
    let mut groups: Vec<String> = Vec::new();
    for s in series.iter() {
        if let Some(ref stack) = s.stack {
            if !stack.is_empty() && !groups.iter().any(|g| g == stack) {
                groups.push(stack.clone());
            }
        }
    }
    for stack_id in groups {
        let indices: Vec<usize> = series
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                s.stack.as_deref() == Some(stack_id.as_str())
                    && matches!(s.series_type, SeriesType::Line | SeriesType::Bar)
            })
            .map(|(i, _)| i)
            .collect();
        if indices.len() < 2 {
            continue;
        }
        calculate_stack(series, &indices);
    }
}

fn calculate_stack(series: &mut [SeriesModel], indices: &[usize]) {
    for pos in 0..indices.len() {
        let si = indices[pos];
        let strategy = series[si].stack_strategy;
        let len = series[si].data.len();
        for data_index in 0..len {
            let own = series[si].data[data_index].value;
            if !own.is_finite() {
                series[si].data[data_index].stack_base = f64::NAN;
                series[si].data[data_index].stacked_value = f64::NAN;
                continue;
            }
            let mut sum = own;
            let mut stacked_over = 0.0;
            for prev in (0..pos).rev() {
                let pj = indices[prev];
                let Some(prev_pt) = series[pj].data.get(data_index) else {
                    continue;
                };
                let prev_result = prev_pt.stacked_value;
                if !prev_result.is_finite() {
                    continue;
                }
                if strategy.allows(sum, prev_result) || strategy.allows(own, prev_result) {
                    if strategy.allows(own, prev_result) {
                        sum = own + prev_result;
                        stacked_over = prev_result;
                        break;
                    }
                }
            }
            series[si].data[data_index].stack_base = stacked_over;
            series[si].data[data_index].stacked_value = sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Sampling;
    use crate::model::DataPoint;
    use crate::option::OptionValue;

    fn pt(v: f64) -> DataPoint {
        DataPoint {
            value: v,
            x_value: None,
            name: None,
            raw_index: 0,
            raw: OptionValue::Number(v),
            stack_base: 0.0,
            stacked_value: v,
        }
    }

    fn series(stack: &str, values: &[f64]) -> SeriesModel {
        SeriesModel {
            index: 0,
            name: stack.into(),
            series_type: SeriesType::Bar,
            data: values.iter().copied().map(pt).collect(),
            x_axis_index: 0,
            y_axis_index: 0,
            coord_sys: crate::model::CoordSysKind::Cartesian,
            polar_index: 0,
            radar_index: 0,
            geo_index: 0,
            calendar_index: 0,
            single_axis_index: 0,
            parallel_index: 0,
            matrix_index: 0,
            stack: Some(stack.into()),
            stack_strategy: StackStrategy::SameSign,
            sampling: Sampling::None,
        }
    }

    #[test]
    fn samesign_positive_stack() {
        let mut list = vec![series("a", &[10.0]), series("a", &[20.0])];
        list[1].index = 1;
        apply_stack(&mut list);
        assert!((list[0].data[0].stacked_value - 10.0).abs() < 1e-9);
        assert!((list[1].data[0].stack_base - 10.0).abs() < 1e-9);
        assert!((list[1].data[0].stacked_value - 30.0).abs() < 1e-9);
    }

    #[test]
    fn samesign_skips_opposite() {
        let mut list = vec![series("a", &[10.0]), series("a", &[-5.0])];
        list[1].index = 1;
        apply_stack(&mut list);
        assert!((list[1].data[0].stacked_value + 5.0).abs() < 1e-9);
        assert!((list[1].data[0].stack_base).abs() < 1e-9);
    }
}
