use crate::{
  dynamiccolor::{Platform, Variant},
  hct::Hct,
  palettes::TonalPalette,
};

pub trait PalettesSpec {
  fn primary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette;

  fn secondary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette;

  fn tertiary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette;

  fn neutral_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette;

  fn neutral_variant_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette;

  fn error_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> Option<TonalPalette>;
}
