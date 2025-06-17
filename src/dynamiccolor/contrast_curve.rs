#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ContrastCurve {
  low: f64,
  normal: f64,
  medium: f64,
  high: f64,
}

impl ContrastCurve {
  pub const fn new(low: f64, normal: f64, medium: f64, high: f64) -> Self {
    Self {
      low,
      normal,
      medium,
      high,
    }
  }

  pub fn get(&self, contrast_level: f64) -> f64 {
    if contrast_level <= -1.0 {
      self.low
    } else if contrast_level < 0.0 {
      utils::math::lerp(self.low, self.normal, (contrast_level - -1.0) / 1.0)
    } else if contrast_level < 0.5 {
      utils::math::lerp(self.normal, self.medium, (contrast_level - 0.0) / 0.5)
    } else if contrast_level < 1.0 {
      utils::math::lerp(self.medium, self.high, (contrast_level - 0.5) / 0.5)
    } else {
      self.high
    }
  }
}
