//! Handler：命中检测（findHover）

use crate::element::EcData;
use crate::storage::{DisplayItem, Storage};

/// 命中结果
#[derive(Debug, Clone, PartialEq)]
pub struct HitResult {
    pub x: f64,
    pub y: f64,
    pub path_index: usize,
    pub top_path_index: usize,
    pub ec_data: EcData,
    pub silent: bool,
}

pub struct Handler;

impl Handler {
    /// 反向遍历 displayList，返回最上层命中图元
    pub fn find_hover(storage: &mut Storage, x: f64, y: f64) -> Option<HitResult> {
        let items: Vec<DisplayItem> = storage.get_display_list(true).to_vec();

        for item in items.iter().rev() {
            if let Some(result) = Self::hit_test_item(storage, item, x, y) {
                return Some(result);
            }
        }
        None
    }

    fn hit_test_item(
        storage: &mut Storage,
        item: &DisplayItem,
        x: f64,
        y: f64,
    ) -> Option<HitResult> {
        if !Self::passes_clip_chain(storage, &item.clip_chain, x, y) {
            return None;
        }

        let path = storage.path_mut(item.path_index);
        if path.base.ignore || path.displayable.invisible {
            return None;
        }

        if !path.hit_test(x, y) {
            return None;
        }

        Some(HitResult {
            x,
            y,
            path_index: item.path_index,
            top_path_index: item.path_index,
            ec_data: path.ec_data.clone(),
            silent: path.silent,
        })
    }

    fn passes_clip_chain(storage: &mut Storage, clip_chain: &[usize], x: f64, y: f64) -> bool {
        for &clip_idx in clip_chain {
            let clip = storage.path_mut(clip_idx);
            if !clip.contains(x, y) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic::displayable::DisplayableProps;
    use crate::graphic::group::ChildRef;
    use crate::graphic::path::Path;
    use crate::graphic::shapes::{CircleShape, RectShape, Shape};
    use crate::graphic::style::{FillStrokeStyle, PathStyle};

    #[test]
    fn find_topmost_path() {
        let mut storage = Storage::new();
        let bottom = storage.create_path(Path::new(
            Shape::Rect(RectShape {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 200.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#5470c6"),
                ..Default::default()
            },
        )
        .with_ec_data(EcData::new(0, 0)));

        let top = storage.create_path(
            Path::new(
                Shape::Circle(CircleShape {
                    cx: 100.0,
                    cy: 100.0,
                    r: 30.0,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color("#ee6666"),
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                z: 1.0,
                ..Default::default()
            })
            .with_ec_data(EcData::new(0, 1)),
        );

        storage.add_root(ChildRef::Path(bottom));
        storage.add_root(ChildRef::Path(top));

        let hit = Handler::find_hover(&mut storage, 100.0, 100.0).unwrap();
        assert_eq!(hit.path_index, top);
        assert_eq!(hit.ec_data.data_index, Some(1));
    }

    #[test]
    fn clip_chain_filters_hit() {
        let mut storage = Storage::new();
        let clip = storage.create_path(Path::new(
            Shape::Circle(CircleShape {
                cx: 50.0,
                cy: 50.0,
                r: 40.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#000"),
                ..Default::default()
            },
        ));

        let rect = storage.create_path(
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
            .with_clip_path(clip),
        );

        storage.add_root(ChildRef::Path(rect));

        assert!(Handler::find_hover(&mut storage, 50.0, 50.0).is_some());
        assert!(Handler::find_hover(&mut storage, 10.0, 10.0).is_none());
    }
}
