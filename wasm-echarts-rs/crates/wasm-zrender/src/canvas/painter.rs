//! Painter：多 zlevel Layer 合成

use std::collections::BTreeMap;

use crate::canvas::backend::CanvasBackend;
use crate::canvas::brush::{BrushScope, brush};
use crate::canvas::layer::Layer;
use crate::core::types::RgbaBuffer;
use crate::storage::Storage;

pub struct Painter<B: CanvasBackend> {
    base_layer: Layer<B>,
    width: u32,
    height: u32,
}

impl<B: CanvasBackend> Painter<B> {
    pub fn new(backend: B, width: u32, height: u32) -> Self {
        Self {
            base_layer: Layer::new(backend),
            width,
            height,
        }
    }

    pub fn refresh(
        &mut self,
        storage: &mut Storage,
    ) -> Result<RgbaBuffer, crate::canvas::backend::BackendError> {
        let items: Vec<_> = storage
            .get_display_list(true)
            .iter()
            .map(|item| (item.zlevel, item.path_index))
            .collect();

        let scope = BrushScope::new(self.width as f64, self.height as f64);

        let mut by_zlevel: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
        for (zlevel, path_index) in items {
            let key = zlevel_to_key(zlevel);
            by_zlevel.entry(key).or_default().push(path_index);
        }

        if by_zlevel.len() <= 1 {
            return self.refresh_single_layer(storage, &scope, &by_zlevel);
        }

        self.refresh_multi_layer(storage, &scope, &by_zlevel)
    }

    fn refresh_single_layer(
        &mut self,
        storage: &mut Storage,
        scope: &BrushScope,
        by_zlevel: &BTreeMap<i64, Vec<usize>>,
    ) -> Result<RgbaBuffer, crate::canvas::backend::BackendError> {
        self.base_layer.clear();
        let ctx = self.base_layer.backend_mut() as &mut dyn crate::canvas::backend::CanvasContext;
        for indices in by_zlevel.values() {
            for &path_index in indices {
                brush(ctx, storage, path_index, scope)?;
            }
        }
        Ok(self.base_layer.backend().get_rgba())
    }

    fn refresh_multi_layer(
        &mut self,
        storage: &mut Storage,
        scope: &BrushScope,
        by_zlevel: &BTreeMap<i64, Vec<usize>>,
    ) -> Result<RgbaBuffer, crate::canvas::backend::BackendError> {
        self.base_layer.clear();
        let mut overlay_layers: Vec<Layer<B>> = Vec::new();

        for (i, (_key, indices)) in by_zlevel.iter().enumerate() {
            if i == 0 {
                let ctx =
                    self.base_layer.backend_mut() as &mut dyn crate::canvas::backend::CanvasContext;
                for &path_index in indices {
                    brush(ctx, storage, path_index, scope)?;
                }
            } else {
                let backend = B::create(self.width, self.height)?;
                let mut layer = Layer::new(backend);
                layer.clear();
                let ctx = layer.backend_mut() as &mut dyn crate::canvas::backend::CanvasContext;
                for &path_index in indices {
                    brush(ctx, storage, path_index, scope)?;
                }
                overlay_layers.push(layer);
            }
        }

        for layer in &overlay_layers {
            composite_layer(&mut self.base_layer, layer)?;
        }

        Ok(self.base_layer.backend().get_rgba())
    }
}

fn zlevel_to_key(zlevel: f64) -> i64 {
    (zlevel * 1000.0).round() as i64
}

fn composite_layer<B: CanvasBackend>(
    base: &mut Layer<B>,
    overlay: &Layer<B>,
) -> Result<(), crate::canvas::backend::BackendError> {
    let overlay_rgba = overlay.backend().get_rgba();
    base.backend_mut().draw_image_rgba(
        &overlay_rgba,
        overlay.width(),
        overlay.height(),
        0.0,
        0.0,
        overlay.width() as f32,
        overlay.height() as f32,
    )
}
