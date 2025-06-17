use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::dynamiccolor::{
  ColorCalculationSpec, ColorCalculationSpec2021, ColorCalculationSpec2025, ColorSpec,
  ColorSpec2021, ColorSpec2025, PalettesSpec, PalettesSpec2021, PalettesSpec2025,
};

/// All available spec versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SpecVersion {
  Spec2021,
  Spec2025,
}

const COLOR_SPEC_2021: ColorSpec2021 = ColorSpec2021::new();
const COLOR_SPEC_2025: ColorSpec2025 = ColorSpec2025::new();
const COLOR_CALCULATION_SPEC_2021: ColorCalculationSpec2021 = ColorCalculationSpec2021::new();
const COLOR_CALCULATION_SPEC_2025: ColorCalculationSpec2025 = ColorCalculationSpec2025::new();
const PALETTES_SPEC_2021: PalettesSpec2021 = PalettesSpec2021::new();
const PALETTES_SPEC_2025: PalettesSpec2025 = PalettesSpec2025::new();

impl SpecVersion {
  pub fn color_spec(&self) -> &'static dyn ColorSpec {
    match *self {
      Self::Spec2021 => &COLOR_SPEC_2021,
      Self::Spec2025 => &COLOR_SPEC_2025,
    }
  }

  pub fn color_calculation_spec(&self) -> &'static dyn ColorCalculationSpec {
    match *self {
      Self::Spec2021 => &COLOR_CALCULATION_SPEC_2021,
      Self::Spec2025 => &COLOR_CALCULATION_SPEC_2025,
    }
  }

  pub fn palettes_spec(&self) -> &'static dyn PalettesSpec {
    match *self {
      Self::Spec2021 => &PALETTES_SPEC_2021,
      Self::Spec2025 => &PALETTES_SPEC_2025,
    }
  }
}

impl Display for SpecVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::Spec2021 => write!(f, "2021"),
      Self::Spec2025 => write!(f, "2025"),
    }
  }
}
