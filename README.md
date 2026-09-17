# phosphor-gpui

[Phosphor Icons](https://phosphoricons.com) for [GPUI](https://www.gpui.rs) and [GPUI Kit](https://gpui-kit.com).

The crate vendors all six Phosphor weights from [`@phosphor-icons/core`](https://github.com/phosphor-icons/core) 2.0.8 (`thin`, `light`, `regular`, `bold`, `fill`, `duotone`) and exposes them as a GPUI `AssetSource` plus an `IconName` catalog that implements GPUI Kit's `IconNamed`.

## Install

Pin a git revision:

```toml
[dependencies]
phosphor-gpui = { git = "https://github.com/AkaraChen/phosphor-gpui", rev = "COMMIT" }
```

## Usage with GPUI Kit

Chain Phosphor in front of Kit's default bundle. Phosphor only serves its own
paths (`icons/phosphor/regular/house.svg`, `icons/phosphor/duotone/house-duotone.svg`); Kit chrome still comes from Kit.

```rust
use phosphor_gpui::{Assets as PhosphorAssets, IconName};

let app = gpui_kit::application()
    .with_assets(PhosphorAssets.with_fallback(gpui_kit::assets::Assets));
```

```rust
SidebarMenuItem::new("Home").icon(IconName::House)
SidebarMenuItem::new("Home").icon(IconName::House.duotone())
```

`IconName` is regular weight. `.thin()`, `.light()`, `.regular()`, `.bold()`, `.fill()`, and `.duotone()` pick a weight.

## License

Crate code is MIT. SVG assets are MIT © [Phosphor Icons](https://github.com/phosphor-icons); see [LICENSE-PHOSPHOR](LICENSE-PHOSPHOR).
