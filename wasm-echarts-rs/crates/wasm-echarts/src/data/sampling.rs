//! 大数据降采样：lttb / average

use crate::model::DataPoint;
use crate::option::OptionValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sampling {
    None,
    Lttb,
    Average,
}

impl Sampling {
    pub fn from_option(s: Option<&str>) -> Self {
        match s {
            Some("lttb") => Sampling::Lttb,
            Some("average") => Sampling::Average,
            _ => Sampling::None,
        }
    }
}

pub fn apply_sampling(data: &[DataPoint], sampling: Sampling, pixel_width: f64) -> Vec<DataPoint> {
    let count = data.len();
    if sampling == Sampling::None || count <= 10 || pixel_width <= 1.0 {
        return data.to_vec();
    }
    let rate = (count as f64 / pixel_width).round();
    if !rate.is_finite() || rate <= 1.0 {
        return data.to_vec();
    }
    let threshold = ((count as f64 / rate).round() as usize).clamp(3, count);
    match sampling {
        Sampling::Lttb => lttb_downsample(data, threshold),
        Sampling::Average => average_downsample(data, threshold),
        Sampling::None => data.to_vec(),
    }
}

fn point_x(p: &DataPoint, i: usize) -> f64 {
    p.x_value.unwrap_or(i as f64)
}

fn lttb_downsample(data: &[DataPoint], threshold: usize) -> Vec<DataPoint> {
    if threshold >= data.len() || data.len() < 3 {
        return data.to_vec();
    }
    let mut sampled = Vec::with_capacity(threshold);
    sampled.push(data[0].clone());
    let bucket_size = (data.len() - 2) as f64 / (threshold - 2) as f64;
    let mut a = 0usize;
    for i in 0..(threshold - 2) {
        let range_start = ((i as f64 + 1.0) * bucket_size).floor() as usize + 1;
        let range_end = (((i as f64 + 2.0) * bucket_size).floor() as usize + 1).min(data.len());
        let next_start = range_end;
        let next_end = ((((i as f64 + 3.0) * bucket_size).floor() as usize) + 1).min(data.len());
        let (avg_x, avg_y, avg_n) = avg_range(data, next_start, next_end.max(next_start + 1));
        let ax = point_x(&data[a], a);
        let ay = data[a].stacked_value;
        let mut max_area = -1.0;
        let mut next_a = range_start;
        for j in range_start..range_end.max(range_start + 1).min(data.len()) {
            let area = (ax - avg_x) * (data[j].stacked_value - ay)
                - (ax - point_x(&data[j], j)) * (avg_y - ay);
            let area = area.abs() / avg_n.max(1.0);
            if area > max_area {
                max_area = area;
                next_a = j;
            }
        }
        sampled.push(data[next_a].clone());
        a = next_a;
    }
    sampled.push(data[data.len() - 1].clone());
    sampled
}

fn avg_range(data: &[DataPoint], start: usize, end: usize) -> (f64, f64, f64) {
    let start = start.min(data.len().saturating_sub(1));
    let end = end.min(data.len()).max(start + 1);
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut n = 0.0;
    for (i, p) in data.iter().enumerate().take(end).skip(start) {
        if p.stacked_value.is_finite() {
            sx += point_x(p, i);
            sy += p.stacked_value;
            n += 1.0;
        }
    }
    if n == 0.0 {
        (0.0, 0.0, 1.0)
    } else {
        (sx / n, sy / n, n)
    }
}

fn average_downsample(data: &[DataPoint], threshold: usize) -> Vec<DataPoint> {
    if threshold >= data.len() {
        return data.to_vec();
    }
    let bucket = data.len() as f64 / threshold as f64;
    let mut out = Vec::with_capacity(threshold);
    for i in 0..threshold {
        let start = (i as f64 * bucket).floor() as usize;
        let end = ((i as f64 + 1.0) * bucket).floor() as usize;
        let end = end.max(start + 1).min(data.len());
        let slice = &data[start..end];
        let mut sum = 0.0;
        let mut n = 0.0;
        for p in slice {
            if p.stacked_value.is_finite() {
                sum += p.stacked_value;
                n += 1.0;
            }
        }
        let avg = if n == 0.0 { f64::NAN } else { sum / n };
        let mid = &slice[slice.len() / 2];
        out.push(DataPoint {
            value: avg,
            stacked_value: avg,
            stack_base: mid.stack_base,
            x_value: mid.x_value,
            name: mid.name.clone(),
            raw_index: mid.raw_index,
            raw: OptionValue::Number(avg),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pts(n: usize) -> Vec<DataPoint> {
        (0..n)
            .map(|i| DataPoint {
                value: i as f64,
                x_value: Some(i as f64),
                name: None,
                raw_index: i,
                raw: OptionValue::Number(i as f64),
                stack_base: 0.0,
                stacked_value: i as f64,
            })
            .collect()
    }

    #[test]
    fn lttb_reduces() {
        let data = pts(1000);
        let out = apply_sampling(&data, Sampling::Lttb, 50.0);
        assert!(out.len() < 200);
        assert!(out.len() >= 3);
        assert_eq!(out.first().unwrap().raw_index, 0);
        assert_eq!(out.last().unwrap().raw_index, 999);
    }

    #[test]
    fn average_reduces() {
        let data = pts(100);
        let out = apply_sampling(&data, Sampling::Average, 10.0);
        assert!(out.len() < 40);
    }
}
