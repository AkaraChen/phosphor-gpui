use std::borrow::Cow;

use anyhow::Result;
use gpui::{AssetSource, SharedString};
use rust_embed::RustEmbed;

use crate::lucide::{LUCIDE_OVERRIDES, lucide_stem};

#[derive(RustEmbed)]
#[folder = "assets/regular"]
#[include = "*.svg"]
struct Regular;

/// Phosphor regular icons, plus GPUI Kit default-icon path aliases.
#[derive(Clone, Copy, Debug, Default)]
pub struct Assets;

impl Assets {
	/// Try Phosphor first, then `fallback` (typically `gpui_kit::assets::Assets`).
	pub fn with_fallback<F: AssetSource>(
		self,
		fallback: F,
	) -> WithFallback<Self, F> {
		WithFallback {
			primary: self,
			fallback,
		}
	}
}

impl AssetSource for Assets {
	fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
		if path.is_empty() {
			return Ok(None);
		}
		if let Some(name) = path.strip_prefix("icons/phosphor/regular/") {
			return Ok(Regular::get(name).map(|file| file.data));
		}
		if let Some(stem) = lucide_path_stem(path) {
			let file = format!("{}.svg", lucide_stem(stem));
			return Ok(Regular::get(&file).map(|file| file.data));
		}
		Ok(None)
	}

	fn list(&self, path: &str) -> Result<Vec<SharedString>> {
		let mut paths: Vec<SharedString> = Regular::iter()
			.map(|name| format!("icons/phosphor/regular/{name}").into())
			.filter(|name: &SharedString| name.starts_with(path))
			.collect();
		paths.extend(Regular::iter().filter_map(|name| {
			let lucide = format!("icons/{name}");
			lucide.starts_with(path).then(|| lucide.into())
		}));
		paths.extend(LUCIDE_OVERRIDES.iter().filter_map(|&(stem, _)| {
			let lucide = format!("icons/{stem}.svg");
			lucide.starts_with(path).then(|| lucide.into())
		}));
		paths.sort();
		paths.dedup();
		Ok(paths)
	}
}

/// Two asset sources, primary then fallback.
pub struct WithFallback<P, F> {
	primary: P,
	fallback: F,
}

impl<P: AssetSource, F: AssetSource> AssetSource for WithFallback<P, F> {
	fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
		match self.primary.load(path)? {
			Some(bytes) => Ok(Some(bytes)),
			None => self.fallback.load(path),
		}
	}

	fn list(&self, path: &str) -> Result<Vec<SharedString>> {
		let mut paths = self.primary.list(path)?;
		paths.extend(self.fallback.list(path)?);
		paths.sort();
		paths.dedup();
		Ok(paths)
	}
}

fn lucide_path_stem(path: &str) -> Option<&str> {
	let rest = path.strip_prefix("icons/")?;
	if rest.contains('/') {
		return None;
	}
	rest.strip_suffix(".svg")
}
