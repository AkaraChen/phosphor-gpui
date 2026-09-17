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
fn unknown_paths_are_absent_not_errors() {
	assert!(Assets.load("icons/not-an-icon.svg").unwrap().is_none());
	assert!(Assets.load("icons/search.svg").unwrap().is_none());
	assert!(Assets.load("").unwrap().is_none());
}

#[test]
fn catalog_is_the_regular_set() {
	assert_eq!(IconName::ALL.len(), 1248);
}
