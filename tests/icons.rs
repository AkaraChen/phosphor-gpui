use gpui::AssetSource;
use phosphor_gpui::{Assets, IconName};

#[test]
fn house_path_is_the_phosphor_regular_asset() {
	assert_eq!(
		IconName::House.path().as_ref(),
		"icons/phosphor/regular/house.svg"
	);
}

#[test]
fn assets_load_phosphor_paths() {
	let bytes = Assets
		.load("icons/phosphor/regular/house.svg")
		.unwrap()
		.expect("house.svg");
	assert!(bytes.starts_with(b"<svg"));
}

#[test]
fn assets_map_gpui_kit_default_lucide_paths() {
	let bytes = Assets
		.load("icons/search.svg")
		.unwrap()
		.expect("mapped search");
	assert!(bytes.starts_with(b"<svg"));
	assert!(Assets.load("icons/chevron-down.svg").unwrap().is_some());
}

#[test]
fn unknown_paths_are_absent_not_errors() {
	assert!(Assets.load("icons/not-an-icon.svg").unwrap().is_none());
	assert!(Assets.load("").unwrap().is_none());
}

#[test]
fn catalog_is_the_regular_set() {
	assert_eq!(IconName::ALL.len(), 1248);
}

#[test]
fn every_gpui_kit_default_icon_has_a_phosphor_file() {
	let source = Assets;
	for line in include_str!("lucide-default-icons.txt").lines() {
		if line.is_empty() {
			continue;
		}
		let bytes = source
			.load(line)
			.unwrap_or_else(|err| panic!("{line}: {err}"));
		assert!(bytes.is_some(), "missing phosphor mapping for {line}");
	}
}
