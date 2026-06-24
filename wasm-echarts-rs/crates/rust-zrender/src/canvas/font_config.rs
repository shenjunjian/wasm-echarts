//! WASM 环境无系统字体，需嵌入默认字体供 cosmic-text 使用。

use std::sync::{Arc, OnceLock};

use vl_convert_canvas2d::{CustomFont, FontConfig, GenericFamilyMap, ResolvedFontConfig};

const EMBEDDED_FONT: &[u8] = include_bytes!("../../assets/fonts/NotoSansSC-Regular.ttf");
const FONT_FAMILY: &str = "Noto Sans SC";

static WASM_RESOLVED: OnceLock<ResolvedFontConfig> = OnceLock::new();

fn wasm_font_config() -> FontConfig {
    let mut generic = GenericFamilyMap::defaults();
    generic.sans_serif = vec![FONT_FAMILY.into()];
    generic.monospace = vec![FONT_FAMILY.into()];
    generic.serif = vec![FONT_FAMILY.into()];

    FontConfig {
        custom_fonts: vec![CustomFont {
            data: Arc::new(EMBEDDED_FONT.to_vec()),
            family_name: Some(FONT_FAMILY.into()),
        }],
        generic_families: generic,
        load_system_fonts: false,
        font_dirs: Vec::new(),
        hinting_enabled: false,
    }
}

pub fn resolved_font_config() -> &'static ResolvedFontConfig {
    WASM_RESOLVED.get_or_init(|| wasm_font_config().resolve())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vl_convert_canvas2d::Canvas2dContext;

    #[test]
    fn embedded_font_loads_and_renders_text() {
        let resolved = wasm_font_config().resolve();
        let mut ctx = Canvas2dContext::with_resolved(200, 100, &resolved).unwrap();
        ctx.set_font("18px sans-serif").unwrap();
        ctx.fill_text("wasm-zrender 文本", 10.0, 50.0);
        let data = ctx.get_image_data(0, 0, 200, 100);
        assert!(data.chunks(4).any(|px| px[3] > 0));
    }
}
