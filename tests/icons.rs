use gpui::AssetSource;
use phosphor_gpui::{Assets, IconName, Weight};

#[test]
fn house_path_is_the_phosphor_regular_asset() {
	assert_eq!(
		IconName::House.path().as_ref(),
		"icons/phosphor/regular/house.svg"
	);
}

#[test]
fn duotone_path_uses_the_weight_suffix() {
	assert_eq!(
		IconName::House.duotone().path().as_ref(),
		"icons/phosphor/duotone/house-duotone.svg"
	);
}

#[test]
fn every_weight_has_a_file() {
	for weight in Weight::ALL {
		let path = IconName::House.with_weight(*weight).path();
		let bytes = Assets
			.load(path.as_ref())
			.unwrap()
			.unwrap_or_else(|| panic!("missing {path}"));
		assert!(bytes.starts_with(b"<svg"), "{path}");
	}
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
