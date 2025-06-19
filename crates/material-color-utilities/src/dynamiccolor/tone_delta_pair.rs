#![allow(deprecated)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::dynamiccolor::DynamicColor;

/// Describes the relationship in lightness between two colors.
///
/// [RelativeDarker](TonePolarity::RelativeDarker) and [RelativeLighter](TonePolarity::RelativeLighter)
/// describes the tone adjustment relative to the surface
/// color trend (white in light mode; black in dark mode).
/// For instance, ToneDeltaPair(A, B, 10, 'relative_lighter', 'farther') states that A should be at least 10 lighter than B in light mode,
/// and at least 10 darker than B in dark mode.
///
/// See [ToneDeltaPair] for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TonePolarity {
  Darker,
  Ligher,
  RelativeDarker,
  RelativeLighter,
  #[deprecated(note = "Use DeltaConstraint instead")]
  Nearer,
  #[deprecated(note = "Use DeltaConstraint instead")]
  Farther,
}

/// Describes how to fulfill a tone delta pair constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum DeltaConstraint {
  Exact,
  Nearer,
  Farther,
}

/// Documents a constraint between two DynamicColors, in which their tones must have a certain
/// distance from each other.
///
/// Prefer a DynamicColor with a background, this is for special cases when designers want tonal
/// distance, literally contrast, between two colors that don't have a background / foreground
/// relationship or a contrast guarantee.
pub struct ToneDeltaPair<'a> {
  role_a: DynamicColor<'a>,
  role_b: DynamicColor<'a>,
  delta: f64,
  polarity: TonePolarity,
  stay_together: bool,
  constraint: DeltaConstraint,
}

impl<'a> ToneDeltaPair<'a> {
  pub fn with_stay_together(
    role_a: DynamicColor<'a>,
    role_b: DynamicColor<'a>,
    delta: f64,
    polarity: TonePolarity,
    stay_together: bool,
  ) -> Self {
    Self {
      role_a,
      role_b,
      delta,
      polarity,
      stay_together,
      constraint: DeltaConstraint::Exact,
    }
  }

  pub fn with_constraint(
    role_a: DynamicColor<'a>,
    role_b: DynamicColor<'a>,
    delta: f64,
    polarity: TonePolarity,
    constraint: DeltaConstraint,
  ) -> Self {
    Self {
      role_a,
      role_b,
      delta,
      polarity,
      stay_together: true,
      constraint,
    }
  }

  pub fn role_a(&self) -> &DynamicColor<'a> {
    &self.role_a
  }

  pub fn role_b(&self) -> &DynamicColor<'a> {
    &self.role_b
  }

  pub fn delta(&self) -> f64 {
    self.delta
  }

  pub fn polarity(&self) -> &TonePolarity {
    &self.polarity
  }

  pub fn stay_together(&self) -> bool {
    self.stay_together
  }

  pub fn constraint(&self) -> &DeltaConstraint {
    &self.constraint
  }
}
