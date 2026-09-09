use crate::option::OptionValue;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// 解析数字或 `'12%'`。百分比相对 `relative`；缺省返回 `default`。
pub fn parse_percent(value: Option<&OptionValue>, relative: f64, default: f64) -> f64 {
    match value {
        Some(OptionValue::Number(n)) => *n,
        Some(OptionValue::String(s)) => {
            let t = s.trim();
            if let Some(p) = t.strip_suffix('%') {
                p.parse::<f64>()
                    .map(|pct| relative * pct / 100.0)
                    .unwrap_or(default)
            } else {
                t.parse::<f64>().unwrap_or(default)
            }
        }
        _ => default,
    }
}

pub fn format_axis_number(v: f64) -> String {
    if v.abs() >= 1000.0 || (v.abs() < 0.01 && v != 0.0) {
        format!("{v:.1e}")
    } else if (v - v.round()).abs() < 0.001 {
        format!("{}", v.round() as i64)
    } else {
        format!("{v:.1}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_of_relative() {
        assert!((parse_percent(Some(&OptionValue::String("50%".into())), 200.0, 0.0) - 100.0).abs() < 1e-9);
        assert!((parse_percent(Some(&OptionValue::Number(12.0)), 200.0, 0.0) - 12.0).abs() < 1e-9);
        assert!((parse_percent(None, 200.0, 7.0) - 7.0).abs() < 1e-9);
    }
}
