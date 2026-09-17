# phosphor-gpui

[Phosphor Icons](https://phosphoricons.com) for [GPUI](https://www.gpui.rs) and [GPUI Kit](https://gpui-kit.com).

The crate vendors Phosphor **regular** SVGs from [`@phosphor-icons/core`](https://github.com/phosphor-icons/core) 2.0.8 and exposes them as a GPUI `AssetSource` plus an `IconName` catalog that implements GPUI Kit's `IconNamed`.

## Install

Pin a git revision:

```toml
[dependencies]
phosphor-gpui = { git = "https://github.com/AkaraChen/phosphor-gpui", rev = "COMMIT" }
```

## Usage with GPUI Kit

Keep GPUI Kit's default Lucide bundle as fallback, and let Phosphor answer first. Phosphor also serves the Kit default-icon paths (`icons/search.svg`, `icons/chevron-down.svg`, …) so chrome and application icons stay in one family.

```rust
use phosphor_gpui::{Assets as PhosphorAssets, IconName};

let app = gpui_kit::application()
    .with_assets(PhosphorAssets.with_fallback(gpui_kit::assets::Assets));
```

```rust
SidebarMenuItem::new("Home").icon(IconName::House)
```

`IconName` is regular-weight Phosphor. Paths look like `icons/phosphor/regular/house.svg`.

## License

Crate code is MIT. SVG assets are MIT © [Phosphor Icons](https://github.com/phosphor-icons); see [LICENSE-PHOSPHOR](LICENSE-PHOSPHOR).
