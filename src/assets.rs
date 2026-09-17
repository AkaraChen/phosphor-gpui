use std::borrow::Cow;

use anyhow::Result;
use gpui::{AssetSource, SharedString};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "**/*.svg"]
struct Bundled;

/// Bundled Phosphor icons for every weight.
#[derive(Clone, Copy, Debug, Default)]
pub struct Assets;

impl Assets {
	/// Serve Phosphor paths from this source, then `fallback`.
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
		let Some(name) = path.strip_prefix("icons/phosphor/") else {
			return Ok(None);
		};
		Ok(Bundled::get(name).map(|file| file.data))
	}

	fn list(&self, path: &str) -> Result<Vec<SharedString>> {
		Ok(Bundled::iter()
			.map(|name| format!("icons/phosphor/{name}").into())
			.filter(|name: &SharedString| name.starts_with(path))
			.collect())
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
