//! Phosphor Icons for GPUI and GPUI Kit.
//!
//! Register [`Assets`] (optionally chained with another bundle) and pass
//! [`IconName`] or [`Icon`] to any control that accepts `IconNamed`.

mod assets;
mod icon;

pub use assets::{Assets, WithFallback};
pub use gpui_kit_assets::IconNamed;
pub use icon::{Icon, IconName, Weight};
