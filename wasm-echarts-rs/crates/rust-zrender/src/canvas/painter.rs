//! Painter：多 zlevel Layer 合成

use std::collections::BTreeMap;

use crate::canvas::backend::CanvasBackend;
use crate::canvas::brush::{BrushScope, brush};
use crate::canvas::image_brush::brush_image;
use crate::storage::DisplayElementRef;
use crate::canvas::layer::Layer;
use crate::canvas::text_brush::brush_text;
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

    pub fn update_font_database(&mut self, resolved: &crate::ResolvedFontConfig)
    where
        B: CanvasBackend,
    {
        self.base_layer.backend_mut().update_font_database(resolved);
    }

    pub fn refresh(
        &mut self,
        storage: &mut Storage,
    ) -> Result<RgbaBuffer, crate::canvas::backend::BackendError> {
        let items: Vec<_> = storage
            .get_display_list(true)
            .iter()
            .map(|item| (item.zlevel, item.element))
            .collect();

        let scope = BrushScope::new(self.width as f64, self.height as f64);

        let mut by_zlevel: BTreeMap<i64, Vec<DisplayElementRef>> = BTreeMap::new();
        for (zlevel, element) in items {
            let key = zlevel_to_key(zlevel);
            by_zlevel.entry(key).or_default().push(element);
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
        by_zlevel: &BTreeMap<i64, Vec<DisplayElementRef>>,
    ) -> Result<RgbaBuffer, crate::canvas::backend::BackendError> {
        self.base_layer.clear();
        let ctx = self.base_layer.backend_mut() as &mut dyn crate::canvas::backend::CanvasContext;
        for elements in by_zlevel.values() {
            brush_elements(ctx, storage, elements, scope)?;
        }
        Ok(self.base_layer.backend().get_rgba())
    }

    fn refresh_multi_layer(
        &mut self,
        storage: &mut Storage,
        scope: &BrushScope,
        by_zlevel: &BTreeMap<i64, Vec<DisplayElementRef>>,
    ) -> Result<RgbaBuffer, crate::canvas::backend::BackendError> {
        self.base_layer.clear();
        let mut overlay_layers: Vec<Layer<B>> = Vec::new();

        for (i, (_key, elements)) in by_zlevel.iter().enumerate() {
            if i == 0 {
                let ctx =
                    self.base_layer.backend_mut() as &mut dyn crate::canvas::backend::CanvasContext;
                brush_elements(ctx, storage, elements, scope)?;
            } else {
                let backend = B::create(self.width, self.height)?;
                let mut layer = Layer::new(backend);
                layer.clear();
                let ctx = layer.backend_mut() as &mut dyn crate::canvas::backend::CanvasContext;
                brush_elements(ctx, storage, elements, scope)?;
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

fn brush_elements(
    ctx: &mut dyn crate::canvas::backend::CanvasContext,
    storage: &mut Storage,
    elements: &[DisplayElementRef],
    scope: &BrushScope,
) -> Result<(), crate::canvas::backend::BackendError> {
    for element in elements {
        match element {
            DisplayElementRef::Path(path_index) => {
                brush(ctx, storage, *path_index, scope)?;
            }
            DisplayElementRef::Image(image_index) => {
                brush_image(ctx, storage, *image_index, scope)?;
            }
            DisplayElementRef::Text(text_index) => {
                let text = storage.text(*text_index);
                if !text.base.ignore && !text.displayable.invisible {
                    brush_text(ctx, text)?;
                }
            }
        }
    }
    Ok(())
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
