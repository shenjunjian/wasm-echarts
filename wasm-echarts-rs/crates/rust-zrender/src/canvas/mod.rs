//! Canvas Painter / Layer / brush 与后端抽象

pub mod backend;
mod font_registry;
pub mod brush;
pub mod demo;
pub mod helper;
pub mod image;
pub mod layer;
pub mod painter;
pub mod text_brush;

pub use font_registry::{
    clear_fonts, register_font, with_resolved_font_config, FontRegistryError, RegisterFontOptions,
};
pub use image::draw_image_rgba;
