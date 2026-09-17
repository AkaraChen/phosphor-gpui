use gpui::{
	AnyElement, App, IntoElement, RenderOnce, SharedString, Styled, Window, svg,
};
use gpui_kit_assets::IconNamed;

include!(concat!(env!("OUT_DIR"), "/icon_name.rs"));

/// Phosphor icon weight.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum Weight {
	Thin,
	Light,
	#[default]
	Regular,
	Bold,
	Fill,
	Duotone,
}

impl Weight {
	/// Every Phosphor weight.
	pub const ALL: &'static [Self] = &[
		Self::Thin,
		Self::Light,
		Self::Regular,
		Self::Bold,
		Self::Fill,
		Self::Duotone,
	];

	/// Directory and suffix name used by `@phosphor-icons/core`.
	pub fn as_str(self) -> &'static str {
		match self {
			Self::Thin => "thin",
			Self::Light => "light",
			Self::Regular => "regular",
			Self::Bold => "bold",
			Self::Fill => "fill",
			Self::Duotone => "duotone",
		}
	}

	fn file_name(self, name: &str) -> String {
		match self {
			Self::Regular => format!("{name}.svg"),
			_ => format!("{name}-{}.svg", self.as_str()),
		}
	}
}

/// A Phosphor icon at a specific [`Weight`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, gpui::IntoElement)]
pub struct Icon {
	name: IconName,
	weight: Weight,
}

impl Icon {
	/// Regular-weight icon.
	pub fn new(name: IconName) -> Self {
		Self {
			name,
			weight: Weight::Regular,
		}
	}

	/// Set the icon weight.
	pub fn weight(mut self, weight: Weight) -> Self {
		self.weight = weight;
		self
	}

	/// The path understood by [`crate::Assets`].
	pub fn path(self) -> SharedString {
		format!(
			"icons/phosphor/{}/{}",
			self.weight.as_str(),
			self.weight.file_name(self.name.name())
		)
		.into()
	}
}

impl IconName {
	/// The regular-weight asset path.
	pub fn path(self) -> SharedString {
		Icon::new(self).path()
	}

	/// Pair this name with a weight.
	pub fn with_weight(self, weight: Weight) -> Icon {
		Icon::new(self).weight(weight)
	}

	pub fn thin(self) -> Icon {
		self.with_weight(Weight::Thin)
	}

	pub fn light(self) -> Icon {
		self.with_weight(Weight::Light)
	}

	pub fn regular(self) -> Icon {
		self.with_weight(Weight::Regular)
	}

	pub fn bold(self) -> Icon {
		self.with_weight(Weight::Bold)
	}

	pub fn fill(self) -> Icon {
		self.with_weight(Weight::Fill)
	}

	pub fn duotone(self) -> Icon {
		self.with_weight(Weight::Duotone)
	}
}

impl IconNamed for IconName {
	fn path(self) -> SharedString {
		IconName::path(self)
	}
}

impl IconNamed for Icon {
	fn path(self) -> SharedString {
		Icon::path(self)
	}
}

fn render_svg(path: SharedString, window: &mut Window) -> impl IntoElement {
	let text_style = window.text_style();
	svg()
		.path(path)
		.flex_shrink_0()
		.size(text_style.font_size.to_pixels(window.rem_size()))
		.text_color(text_style.color)
}

impl RenderOnce for IconName {
	fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
		render_svg(self.path(), window)
	}
}

impl RenderOnce for Icon {
	fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
		render_svg(self.path(), window)
	}
}

impl From<IconName> for AnyElement {
	fn from(name: IconName) -> Self {
		name.into_any_element()
	}
}

impl From<Icon> for AnyElement {
	fn from(icon: Icon) -> Self {
		icon.into_any_element()
	}
}
