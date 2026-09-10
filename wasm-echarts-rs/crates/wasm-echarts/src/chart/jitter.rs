//! ScatterJitter：类目轴 `jitter` 把重叠点沿轴终态错开。

use crate::coord::SeriesCoord;
use crate::model::{AxisModel, SeriesModel};

#[derive(Default)]
pub struct JitterState {
    items: Vec<(f64, f64, f64)>,
}

pub fn apply_scatter_jitter(
    state: &mut JitterState,
    coord: &SeriesCoord,
    series: &SeriesModel,
    data_index: usize,
    size: f64,
    mut x: f64,
    mut y: f64,
) -> (f64, f64) {
    let Some(cart) = coord.as_cartesian() else {
        return jitter_single(state, coord, series, data_index, size, x, y);
    };
    let x_axis = cart.x_axis();
    let y_axis = cart.y_axis();
    let has_category = x_axis.axis_type.is_category() || y_axis.axis_type.is_category();
    if !has_category {
        return (x, y);
    }
    let band = cart.category_band_width();
    let radius = size / 2.0;
    if x_axis.jitter > 0.0 {
        x = jitter_one(
            state,
            x_axis,
            y,
            x,
            radius,
            band,
            series.index,
            data_index,
            1,
        );
    }
    if y_axis.jitter > 0.0 {
        y = jitter_one(
            state,
            y_axis,
            x,
            y,
            radius,
            band,
            series.index,
            data_index,
            2,
        );
    }
    (x, y)
}

fn jitter_single(
    state: &mut JitterState,
    coord: &SeriesCoord,
    series: &SeriesModel,
    data_index: usize,
    size: f64,
    x: f64,
    y: f64,
) -> (f64, f64) {
    let Some(single) = coord.as_single() else {
        return (x, y);
    };
    let axis = single.axis();
    if axis.jitter <= 0.0 {
        return (x, y);
    }
    let radius = size / 2.0;
    let band = if single.spec.horizontal {
        single.spec.rect.width / axis.category_data.len().max(1) as f64
    } else {
        single.spec.rect.height / axis.category_data.len().max(1) as f64
    };
    if single.spec.horizontal {
        let nx = jitter_one(state, axis, y, x, radius, band, series.index, data_index, 3);
        (nx, y)
    } else {
        let ny = jitter_one(state, axis, x, y, radius, band, series.index, data_index, 4);
        (x, ny)
    }
}

fn jitter_one(
    state: &mut JitterState,
    axis: &AxisModel,
    fixed: f64,
    float_coord: f64,
    radius: f64,
    band: f64,
    series_index: usize,
    data_index: usize,
    seed: usize,
) -> f64 {
    if axis.jitter <= 0.0 {
        return float_coord;
    }
    if axis.jitter_overlap {
        return jitter_random(float_coord, axis.jitter, band, radius, series_index, data_index, seed);
    }
    jitter_avoid(state, fixed, float_coord, radius, axis.jitter, axis.jitter_margin, band)
}

fn unit_hash(series_index: usize, data_index: usize, seed: usize) -> f64 {
    let n = series_index
        .wrapping_mul(73856093)
        ^ data_index.wrapping_mul(19349663)
        ^ seed.wrapping_mul(83492791);
    (n % 10_000) as f64 / 10_000.0
}

fn jitter_random(
    float_coord: f64,
    jitter: f64,
    band: f64,
    radius: f64,
    series_index: usize,
    data_index: usize,
    seed: usize,
) -> f64 {
    let max_j = if band > 0.0 {
        (band - radius * 2.0).max(0.0)
    } else {
        jitter
    };
    let actual = jitter.min(max_j);
    float_coord + (unit_hash(series_index, data_index, seed) - 0.5) * actual
}

fn jitter_avoid(
    state: &mut JitterState,
    fixed: f64,
    float_coord: f64,
    radius: f64,
    jitter: f64,
    margin: f64,
    band: f64,
) -> f64 {
    let a = place_dir(&state.items, fixed, float_coord, radius, jitter, margin, 1.0);
    let b = place_dir(&state.items, fixed, float_coord, radius, jitter, margin, -1.0);
    let chosen = if (a - float_coord).abs() < (b - float_coord).abs() {
        a
    } else {
        b
    };
    let distance = (chosen - float_coord).abs();
    if distance > jitter / 2.0 || (band > 0.0 && distance > band / 2.0 - radius) {
        return jitter_random(float_coord, jitter, band, radius, 0, state.items.len(), 9);
    }
    state.items.push((fixed, chosen, radius));
    chosen
}

fn place_dir(
    items: &[(f64, f64, f64)],
    fixed: f64,
    float_coord: f64,
    radius: f64,
    jitter: f64,
    margin: f64,
    direction: f64,
) -> f64 {
    let mut y = float_coord;
    let mut i = 0;
    while i < items.len() {
        let (fx, fy, fr) = items[i];
        let dx = fixed - fx;
        let dy = y - fy;
        let r = radius + fr + margin;
        if dx * dx + dy * dy < r * r {
            let required = fy + (r * r - dx * dx).max(0.0).sqrt() * direction;
            if (required - float_coord).abs() > jitter / 2.0 {
                return f64::MAX;
            }
            if (direction > 0.0 && required > y) || (direction < 0.0 && required < y) {
                y = required;
                i = 0;
                continue;
            }
        }
        i += 1;
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_jitter_is_stable() {
        let a = jitter_random(100.0, 10.0, 20.0, 2.0, 0, 3, 1);
        let b = jitter_random(100.0, 10.0, 20.0, 2.0, 0, 3, 1);
        assert!((a - b).abs() < 1e-12);
        assert!((a - 100.0).abs() <= 5.0);
    }
}
