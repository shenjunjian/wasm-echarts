//! timsort 稳定排序（对齐 zrender Storage displayList 排序）

use std::cmp::Ordering;

/// 对 displayList 做稳定 timsort 风格排序（Rust 标准库 sort 已稳定，比较器与 zrender 一致）
pub fn sort_display_list<T, F>(items: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    items.sort_by(|a, b| compare(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_sort_preserves_equal_z_order() {
        let mut items = vec![(0.0, 0, 1), (0.0, 0, 2), (1.0, 0, 0)];
        sort_display_list(&mut items, |a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(Ordering::Equal)
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.cmp(&b.2))
        });
        assert_eq!(items[0].2, 1);
        assert_eq!(items[1].2, 2);
        assert_eq!(items[2].0, 1.0);
    }
}
