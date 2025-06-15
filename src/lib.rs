//! # Material Color Utilities
//!
//! Algorithms and utilities that power the Material Design 3 (M3) color system,
//! including choosing theme colors from images and creating tones of colors;
//! all in a new color space.
//!
//! See the shared repository
//! [deminearchiver/material-color-utilities](https://github.com/deminearchiver/material-color-utilities)
//! for additional information.

pub mod blend;
pub mod contrast;
pub mod dislike_analyzer;
pub mod hct;
pub mod palettes;
pub mod score;
pub mod temperature_cache;
pub mod utils;

#[cfg(test)]
pub(crate) mod test;
