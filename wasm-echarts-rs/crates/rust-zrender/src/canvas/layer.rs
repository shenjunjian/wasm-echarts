//! 单层 Canvas Layer

use crate::canvas::backend::CanvasBackend;

pub struct Layer<B: CanvasBackend> {
    backend: B,
}

impl<B: CanvasBackend> Layer<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn clear(&mut self) {
        self.backend.clear();
    }

    pub fn width(&self) -> u32 {
        self.backend.width()
    }

    pub fn height(&self) -> u32 {
        self.backend.height()
    }
}
