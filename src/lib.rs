//! Phosphor Icons for GPUI and GPUI Kit.
//!
//! Register [`Assets`] (optionally chained with GPUI Kit's default bundle)
//! and pass [`IconName`] to any control that accepts `IconNamed`.

mod assets;
mod icon;
mod lucide;

pub use assets::{Assets, WithFallback};
pub use gpui_kit_assets::IconNamed;
pub use icon::IconName;
