use gpui::{
	AnyElement, App, IntoElement, RenderOnce, SharedString, Styled, Window, svg,
};
use gpui_kit_assets::IconNamed;

include!(concat!(env!("OUT_DIR"), "/icon_name.rs"));

impl IconNamed for IconName {
	fn path(self) -> SharedString {
		IconName::path(self)
	}
}

impl RenderOnce for IconName {
	fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
		let text_style = window.text_style();
		svg()
			.path(self.path())
			.flex_shrink_0()
			.size(text_style.font_size.to_pixels(window.rem_size()))
			.text_color(text_style.color)
	}
}

impl From<IconName> for AnyElement {
	fn from(name: IconName) -> Self {
		name.into_any_element()
	}
}
