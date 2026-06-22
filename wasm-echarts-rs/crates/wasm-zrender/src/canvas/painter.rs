//! Painter：遍历 displayList 并 brush

use crate::canvas::backend::CanvasBackend;
use crate::canvas::brush::{BrushScope, brush};
use crate::canvas::layer::Layer;
use crate::core::types::RgbaBuffer;
use crate::storage::Storage;

pub struct Painter<B: CanvasBackend> {
    layer: Layer<B>,
    width: u32,
    height: u32,
}

impl<B: CanvasBackend> Painter<B> {
    pub fn new(backend: B, width: u32, height: u32) -> Self {
        Self {
            layer: Layer::new(backend),
            width,
            height,
        }
    }

    pub fn refresh(
        &mut self,
        storage: &mut Storage,
    ) -> Result<RgbaBuffer, crate::canvas::backend::BackendError> {
        self.layer.clear();
        let scope = BrushScope::new(self.width as f64, self.height as f64);
        let display_list: Vec<usize> = storage
            .get_display_list(true)
            .iter()
            .map(|item| item.path_index)
            .collect();

        let ctx = self.layer.backend_mut() as &mut dyn crate::canvas::backend::CanvasContext;
        for path_index in display_list {
            let path = storage.path_mut(path_index);
            brush(ctx, path, &scope)?;
        }

        Ok(self.layer.backend().get_rgba())
    }
}
