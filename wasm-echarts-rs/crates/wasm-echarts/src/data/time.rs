//! 时间值解析：数字时间戳或 ISO 日期字符串 → 毫秒。

use crate::option::OptionValue;

/// 把 option 值收成时间轴用的毫秒时间戳。
pub fn parse_time_value(value: &OptionValue) -> Option<f64> {
    match value {
        OptionValue::Number(n) if n.is_finite() => Some(*n),
        OptionValue::String(s) => parse_time_string(s),
        _ => None,
    }
}

pub fn parse_time_string(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    if let Ok(n) = t.parse::<f64>() {
        if n.is_finite() {
            return Some(n);
        }
    }
    parse_iso_like(t)
}

/// `YYYY-MM-DD` / `YYYY/MM/DD` / 可选 `T` 或空格 + `HH:mm[:ss][.sss]`
fn parse_iso_like(s: &str) -> Option<f64> {
    let (date_part, time_part) = split_date_time(s)?;
    let seps: Vec<char> = date_part.chars().filter(|c| *c == '-' || *c == '/').collect();
    let sep = *seps.first()?;
    let mut date_bits = date_part.split(sep);
    let year: i32 = date_bits.next()?.parse().ok()?;
    let month: u32 = date_bits.next()?.parse().ok()?;
    let day: u32 = date_bits.next()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let (hour, minute, second, millis) = parse_clock(time_part.unwrap_or(""));
    let unix_days = days_from_civil(year, month, day)?;
    let ms = unix_days as f64 * 86_400_000.0
        + hour as f64 * 3_600_000.0
        + minute as f64 * 60_000.0
        + second as f64 * 1_000.0
        + millis as f64;
    Some(ms)
}

fn split_date_time(s: &str) -> Option<(&str, Option<&str>)> {
    if let Some(i) = s.find('T') {
        return Some((&s[..i], Some(&s[i + 1..])));
    }
    if let Some(i) = s.find(' ') {
        return Some((&s[..i], Some(&s[i + 1..])));
    }
    Some((s, None))
}

fn parse_clock(s: &str) -> (u32, u32, u32, u32) {
    let s = s.trim().trim_end_matches('Z').trim_end_matches('z');
    if s.is_empty() {
        return (0, 0, 0, 0);
    }
    let (hms, frac) = match s.split_once('.') {
        Some((a, b)) => (a, b),
        None => (s, ""),
    };
    let mut parts = hms.split(':');
    let hour = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    let minute = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    let second = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
    let millis = if frac.is_empty() {
        0
    } else {
        let digits: String = frac.chars().filter(|c| c.is_ascii_digit()).take(3).collect();
        let mut ms = digits.parse::<u32>().unwrap_or(0);
        for _ in digits.len()..3 {
            ms *= 10;
        }
        ms
    };
    (hour.min(23), minute.min(59), second.min(59), millis.min(999))
}

/// Howard Hinnant civil-from-days 的逆：公历 → 儒略日偏移（days since 0000-03-01 体系）
pub fn days_from_civil(year: i32, month: u32, day: u32) -> Option<i64> {
    if month == 0 || month > 12 || day == 0 || day > 31 {
        return None;
    }
    let y = if month <= 2 { year - 1 } else { year } as i64;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let mp = if month > 2 { month - 3 } else { month + 9 } as u64;
    let doy = (153 * mp + 2) / 5 + day as u64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146_097 + doe as i64 - 719_468)
}

/// 周日=0 … 周六=6（UTC，与日历格子一致）
pub fn weekday_sun0(ms: f64) -> i32 {
    let days = (ms / 86_400_000.0).floor() as i64;
    ((days + 4).rem_euclid(7)) as i32
}

pub fn ymd_of(ms: f64) -> (i32, u32, u32) {
    let unix_days = (ms / 86_400_000.0).floor() as i64;
    civil_from_days(unix_days)
}

pub fn format_time_label(ms: f64) -> String {
    if !ms.is_finite() {
        return String::new();
    }
    let unix_days = (ms / 86_400_000.0).floor() as i64;
    let (year, month, day) = civil_from_days(unix_days);
    let rem = ms - unix_days as f64 * 86_400_000.0;
    let rem = if rem < 0.0 { rem + 86_400_000.0 } else { rem };
    let hour = (rem / 3_600_000.0).floor() as i32;
    if hour == 0 && rem < 60_000.0 {
        format!("{year:04}-{month:02}-{day:02}")
    } else {
        let minute = ((rem / 60_000.0) as i32) % 60;
        format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}")
    }
}

pub fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_date_to_unix() {
        let ms = parse_time_string("1970-01-01").unwrap();
        assert!((ms - 0.0).abs() < 1.0);
        let ms = parse_time_string("1970-01-02").unwrap();
        assert!((ms - 86_400_000.0).abs() < 1.0);
    }

    #[test]
    fn format_roundtrip_day() {
        let ms = parse_time_string("2020-01-15").unwrap();
        assert_eq!(format_time_label(ms), "2020-01-15");
    }
}
