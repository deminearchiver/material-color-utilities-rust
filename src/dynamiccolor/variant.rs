#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Themes for Dynamic Color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Variant {
  Monochrome,
  Neutral,
  TonalSpot,
  Vibrant,
  Expressive,
  Fidelity,
  Content,
  Rainbow,
  FruitSalad,
}
