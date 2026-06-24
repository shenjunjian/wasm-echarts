//! 可注册的字体库，供离屏 Canvas 文本渲染使用（WASM / Wasmer 等无系统字体环境）。

use std::sync::{Arc, Mutex, MutexGuard};

use vl_convert_canvas2d::{CustomFont, FontConfig, GenericFamilyMap, ResolvedFontConfig};

#[derive(Debug, thiserror::Error)]
pub enum FontRegistryError {
    #[error("font registry lock poisoned")]
    LockPoisoned,
}

/// 注册字体时的可选配置。
#[derive(Debug, Clone, Default)]
pub struct RegisterFontOptions {
    /// 覆盖字体族名；不传则使用字体文件 name table 中的名称。
    pub family_name: Option<String>,
    /// 将 CSS `sans-serif` 映射到这些族名（按优先级）；不传且提供了 `family_name` 时，默认映射到该族名。
    pub sans_serif: Option<Vec<String>>,
}

struct FontRegistryState {
    custom_fonts: Vec<CustomFont>,
    sans_serif_override: Option<Vec<String>>,
    resolved: Option<ResolvedFontConfig>,
}

impl FontRegistryState {
    fn new() -> Self {
        Self {
            custom_fonts: Vec::new(),
            sans_serif_override: None,
            resolved: None,
        }
    }

    fn register_font(&mut self, data: Vec<u8>, options: RegisterFontOptions) {
        let family_for_mapping = options.family_name.clone();
        self.custom_fonts.push(CustomFont {
            data: Arc::new(data),
            family_name: options.family_name,
        });
        if let Some(families) = options.sans_serif {
            self.sans_serif_override = Some(families);
        } else if let Some(family) = family_for_mapping {
            self.sans_serif_override = Some(vec![family]);
        }
        self.resolved = None;
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    fn build_config(&self) -> FontConfig {
        let mut generic = GenericFamilyMap::defaults();
        if let Some(families) = &self.sans_serif_override {
            generic.sans_serif = families.clone();
        }

        FontConfig {
            custom_fonts: self.custom_fonts.clone(),
            generic_families: generic,
            load_system_fonts: cfg!(not(target_arch = "wasm32")),
            font_dirs: Vec::new(),
            hinting_enabled: false,
        }
    }

    fn resolved(&mut self) -> &ResolvedFontConfig {
        if self.resolved.is_none() {
            self.resolved = Some(self.build_config().resolve());
        }
        self.resolved.as_ref().unwrap()
    }
}

static FONT_REGISTRY: Mutex<FontRegistryState> = Mutex::new(FontRegistryState {
    custom_fonts: Vec::new(),
    sans_serif_override: None,
    resolved: None,
});

fn lock_registry() -> Result<MutexGuard<'static, FontRegistryState>, FontRegistryError> {
    FONT_REGISTRY
        .lock()
        .map_err(|_| FontRegistryError::LockPoisoned)
}

/// 注册一份字体文件（TTF / OTF / WOFF）。
pub fn register_font(data: Vec<u8>, options: RegisterFontOptions) -> Result<(), FontRegistryError> {
    lock_registry()?.register_font(data, options);
    Ok(())
}

/// 清空已注册字体（主要用于测试）。
pub fn clear_fonts() -> Result<(), FontRegistryError> {
    lock_registry()?.clear();
    Ok(())
}

/// 在全局字体配置解析结果上执行回调（创建 Canvas 或热更新字体库时使用）。
pub fn with_resolved_font_config<F, R>(f: F) -> Result<R, FontRegistryError>
where
    F: FnOnce(&ResolvedFontConfig) -> R,
{
    let mut guard = lock_registry()?;
    Ok(f(guard.resolved()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use vl_convert_canvas2d::Canvas2dContext;

    const TEST_FONT: &[u8] = include_bytes!("../../tests/fixtures/NotoSansSC-Regular.ttf");

    fn reset() {
        clear_fonts().unwrap();
    }

    #[test]
    fn registered_font_renders_text() {
        reset();
        register_font(
            TEST_FONT.to_vec(),
            RegisterFontOptions {
                family_name: Some("Noto Sans SC".into()),
                sans_serif: Some(vec!["Noto Sans SC".into()]),
            },
        )
        .unwrap();

        with_resolved_font_config(|resolved| {
            let mut ctx = Canvas2dContext::with_resolved(200, 100, resolved).unwrap();
            ctx.set_font("18px sans-serif").unwrap();
            ctx.fill_text("wasm-zrender 文本", 10.0, 50.0);
            let data = ctx.get_image_data(0, 0, 200, 100);
            assert!(data.chunks(4).any(|px| px[3] > 0));
        })
        .unwrap();
    }
}
