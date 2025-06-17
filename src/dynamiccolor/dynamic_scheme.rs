use std::fmt::Display;

use crate::{
  dynamiccolor::{DynamicColor, Platform, SpecVersion, Variant},
  hct::Hct,
  palettes::TonalPalette,
  utils,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicScheme {
  source_color_argb: u32,
  source_color_hct: Hct,
  variant: Variant,
  is_dark: bool,
  platform: Platform,
  contrast_level: f64,
  spec_version: SpecVersion,
  primary_palette: TonalPalette,
  secondary_palette: TonalPalette,
  tertiary_palette: TonalPalette,
  neutral_palette: TonalPalette,
  neutral_variant_palette: TonalPalette,
  error_palette: TonalPalette,
}

impl DynamicScheme {
  pub const fn default_spec_version() -> SpecVersion {
    SpecVersion::Spec2021
  }

  pub const fn default_platform() -> Platform {
    Platform::Phone
  }

  pub fn get_piecewise_value(source_color_hct: &Hct, hue_breakpoints: &[f64], hues: &[f64]) -> f64 {
    let size = isize::min(hue_breakpoints.len() as isize - 1, hues.len() as isize) as usize;
    let source_hue = source_color_hct.hue();
    for i in 0..size {
      if source_hue >= hue_breakpoints[i] && source_hue < hue_breakpoints[i + 1] {
        return utils::math::sanitize_degrees(hues[i]);
      }
    }
    source_hue
  }

  pub fn get_rotated_hue(
    source_color_hct: &Hct,
    hue_breakpoints: &[f64],
    rotations: &[f64],
  ) -> f64 {
    let mut rotation = Self::get_piecewise_value(source_color_hct, hue_breakpoints, rotations);
    if isize::min(hue_breakpoints.len() as isize - 1, rotations.len() as isize) <= 0 {
      rotation = 0.0;
    }
    utils::math::sanitize_degrees(source_color_hct.hue() + rotation)
  }

  #[allow(clippy::too_many_arguments)]
  pub fn new(
    source_color_hct: Hct,
    variant: Variant,
    is_dark: bool,
    platform: Platform,
    contrast_level: f64,
    spec_version: SpecVersion,
    primary_palette: TonalPalette,
    secondary_palette: TonalPalette,
    tertiary_palette: TonalPalette,
    neutral_palette: TonalPalette,
    neutral_variant_palette: TonalPalette,
    error_palette: TonalPalette,
  ) -> Self {
    Self {
      source_color_argb: source_color_hct.to_int(),
      source_color_hct,
      variant,
      is_dark,
      platform,
      contrast_level,
      spec_version,
      primary_palette,
      secondary_palette,
      tertiary_palette,
      neutral_palette,
      neutral_variant_palette,
      error_palette,
    }
  }

  pub fn source_color_argb(&self) -> u32 {
    self.source_color_argb
  }
  pub fn source_color_hct(&self) -> &Hct {
    &self.source_color_hct
  }
  pub fn variant(&self) -> &Variant {
    &self.variant
  }
  pub fn is_dark(&self) -> bool {
    self.is_dark
  }
  pub fn platform(&self) -> &Platform {
    &self.platform
  }
  pub fn contrast_level(&self) -> f64 {
    self.contrast_level
  }
  pub fn spec_version(&self) -> &SpecVersion {
    &self.spec_version
  }
  pub fn primary_palette(&self) -> &TonalPalette {
    &self.primary_palette
  }
  pub fn secondary_palette(&self) -> &TonalPalette {
    &self.secondary_palette
  }
  pub fn tertiary_palette(&self) -> &TonalPalette {
    &self.tertiary_palette
  }
  pub fn neutral_palette(&self) -> &TonalPalette {
    &self.neutral_palette
  }
  pub fn neutral_variant_palette(&self) -> &TonalPalette {
    &self.neutral_variant_palette
  }
  pub fn error_palette(&self) -> &TonalPalette {
    &self.error_palette
  }

  pub fn get_hct<'a>(&'a self, dynamic_color: &'a DynamicColor<'a>) -> Hct {
    dynamic_color.get_hct(self)
  }

  pub fn get_argb<'a>(&'a self, dynamic_color: &'a DynamicColor<'a>) -> u32 {
    dynamic_color.get_argb(self)
  }
}

impl Display for DynamicScheme {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Scheme: variant={:?}, mode={}, platform={:?}, contrastLevel={}, seed={}, specVersion={:?}",
      self.variant(),
      if self.is_dark() { "dark" } else { "light" },
      self.platform(),
      self.contrast_level(),
      self.source_color_hct(),
      self.spec_version()
    )
  }
}

#[derive(Debug, Clone, PartialEq)]
enum DynamicSchemePalette {
  KeyColor(Hct),
  Palette(TonalPalette),
}

impl DynamicSchemePalette {
  pub fn tonal_palette<F>(
    palette: Option<Self>,
    f: F,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette
  where
    F: FnOnce(&Variant, &Hct, bool, &Platform, f64) -> TonalPalette,
  {
    match palette {
      Some(DynamicSchemePalette::Palette(palette)) => palette,
      Some(DynamicSchemePalette::KeyColor(palette_key_color)) => f(
        variant,
        &palette_key_color,
        is_dark,
        platform,
        contrast_level,
      ),
      None => f(variant, source_color_hct, is_dark, platform, contrast_level),
    }
  }

  pub fn tonal_palette_or_none<F>(
    palette: Option<Self>,
    f: F,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> Option<TonalPalette>
  where
    F: FnOnce(&Variant, &Hct, bool, &Platform, f64) -> Option<TonalPalette>,
  {
    match palette {
      Some(DynamicSchemePalette::Palette(palette)) => Some(palette),
      Some(DynamicSchemePalette::KeyColor(palette_key_color)) => f(
        variant,
        &palette_key_color,
        is_dark,
        platform,
        contrast_level,
      ),
      None => f(variant, source_color_hct, is_dark, platform, contrast_level),
    }
  }
}

trait OptionExt {
  fn tonal_palette<F>(
    self,
    f: F,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette
  where
    F: FnOnce(&Variant, &Hct, bool, &Platform, f64) -> TonalPalette;

  fn tonal_palette_or_none<F>(
    self,
    f: F,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> Option<TonalPalette>
  where
    F: FnOnce(&Variant, &Hct, bool, &Platform, f64) -> Option<TonalPalette>;
}

impl OptionExt for Option<DynamicSchemePalette> {
  fn tonal_palette<F>(
    self,
    f: F,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette
  where
    F: FnOnce(&Variant, &Hct, bool, &Platform, f64) -> TonalPalette,
  {
    DynamicSchemePalette::tonal_palette(
      self,
      f,
      variant,
      source_color_hct,
      is_dark,
      platform,
      contrast_level,
    )
  }

  fn tonal_palette_or_none<F>(
    self,
    f: F,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> Option<TonalPalette>
  where
    F: FnOnce(&Variant, &Hct, bool, &Platform, f64) -> Option<TonalPalette>,
  {
    DynamicSchemePalette::tonal_palette_or_none(
      self,
      f,
      variant,
      source_color_hct,
      is_dark,
      platform,
      contrast_level,
    )
  }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DynamicSchemeBuilder {
  source_color_hct: Option<Hct>,
  variant: Option<Variant>,
  is_dark: Option<bool>,
  platform: Option<Platform>,
  contrast_level: Option<f64>,
  spec_version: Option<SpecVersion>,
  primary_palette: Option<DynamicSchemePalette>,
  secondary_palette: Option<DynamicSchemePalette>,
  tertiary_palette: Option<DynamicSchemePalette>,
  neutral_palette: Option<DynamicSchemePalette>,
  neutral_variant_palette: Option<DynamicSchemePalette>,
  error_palette: Option<DynamicSchemePalette>,
}

impl DynamicSchemeBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn source_color(mut self, source_color_hct: Hct) -> Self {
    self.source_color_hct = Some(source_color_hct);
    self
  }

  pub fn variant(mut self, variant: Variant) -> Self {
    self.variant = Some(variant);
    self
  }

  pub fn is_dark(mut self, is_dark: bool) -> Self {
    self.is_dark = Some(is_dark);
    self
  }

  pub fn platform(mut self, platform: Platform) -> Self {
    self.platform = Some(platform);
    self
  }

  pub fn contrast_level(mut self, contrast_level: f64) -> Self {
    self.contrast_level = Some(contrast_level);
    self
  }

  pub fn spec_version(mut self, spec_version: SpecVersion) -> Self {
    self.spec_version = Some(spec_version);
    self
  }

  pub fn primary_palette(mut self, primary_palette: TonalPalette) -> Self {
    self.primary_palette = Some(DynamicSchemePalette::Palette(primary_palette));
    self
  }

  pub fn secondary_palette(mut self, secondary_palette: TonalPalette) -> Self {
    self.secondary_palette = Some(DynamicSchemePalette::Palette(secondary_palette));
    self
  }

  pub fn tertiary_palette(mut self, tertiary_palette: TonalPalette) -> Self {
    self.tertiary_palette = Some(DynamicSchemePalette::Palette(tertiary_palette));
    self
  }

  pub fn neutral_palette(mut self, neutral_palette: TonalPalette) -> Self {
    self.neutral_palette = Some(DynamicSchemePalette::Palette(neutral_palette));
    self
  }

  pub fn neutral_variant_palette(mut self, neutral_variant_palette: TonalPalette) -> Self {
    self.neutral_variant_palette = Some(DynamicSchemePalette::Palette(neutral_variant_palette));
    self
  }

  pub fn error_palette(mut self, error_palette: TonalPalette) -> Self {
    self.error_palette = Some(DynamicSchemePalette::Palette(error_palette));
    self
  }

  pub fn primary_palette_key_color(mut self, primary_palette_key_color: Hct) -> Self {
    self.primary_palette = Some(DynamicSchemePalette::KeyColor(primary_palette_key_color));
    self
  }

  pub fn secondary_palette_key_color(mut self, secondary_palette_key_color: Hct) -> Self {
    self.secondary_palette = Some(DynamicSchemePalette::KeyColor(secondary_palette_key_color));
    self
  }

  pub fn tertiary_palette_key_color(mut self, tertiary_palette_key_color: Hct) -> Self {
    self.tertiary_palette = Some(DynamicSchemePalette::KeyColor(tertiary_palette_key_color));
    self
  }

  pub fn neutral_palette_key_color(mut self, neutral_palette_key_color: Hct) -> Self {
    self.neutral_palette = Some(DynamicSchemePalette::KeyColor(neutral_palette_key_color));
    self
  }

  pub fn neutral_variant_palette_key_color(
    mut self,
    neutral_variant_palette_key_color: Hct,
  ) -> Self {
    self.neutral_variant_palette = Some(DynamicSchemePalette::KeyColor(
      neutral_variant_palette_key_color,
    ));
    self
  }

  pub fn error_palette_key_color(mut self, error_palette_key_color: Hct) -> Self {
    self.error_palette = Some(DynamicSchemePalette::KeyColor(error_palette_key_color));
    self
  }

  pub fn build(self) -> Result<DynamicScheme, String> {
    let source_color_hct = self
      .source_color_hct
      .unwrap_or_else(|| Hct::from_int(0xff6750a4));
    let variant = self.variant.unwrap_or(Variant::TonalSpot);
    let is_dark = self.is_dark.ok_or_else(|| "is_dark".to_string())?;
    let platform = self
      .platform
      .unwrap_or_else(DynamicScheme::default_platform);
    let contrast_level = self.contrast_level.unwrap_or(0.0);
    let spec_version = self
      .spec_version
      .unwrap_or_else(DynamicScheme::default_spec_version);

    let spec = spec_version.palettes_spec();
    let primary_palette = self.primary_palette.tonal_palette(
      |variant, source_color_hct, is_dark, platform, contrast_level| {
        spec.primary_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      },
      &variant,
      &source_color_hct,
      is_dark,
      &platform,
      contrast_level,
    );
    let secondary_palette = self.secondary_palette.tonal_palette(
      |variant, source_color_hct, is_dark, platform, contrast_level| {
        spec.secondary_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      },
      &variant,
      &source_color_hct,
      is_dark,
      &platform,
      contrast_level,
    );
    let tertiary_palette = self.tertiary_palette.tonal_palette(
      |variant, source_color_hct, is_dark, platform, contrast_level| {
        spec.tertiary_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      },
      &variant,
      &source_color_hct,
      is_dark,
      &platform,
      contrast_level,
    );
    let neutral_palette = self.neutral_palette.tonal_palette(
      |variant, source_color_hct, is_dark, platform, contrast_level| {
        spec.neutral_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      },
      &variant,
      &source_color_hct,
      is_dark,
      &platform,
      contrast_level,
    );
    let neutral_variant_palette = self.neutral_variant_palette.tonal_palette(
      |variant, source_color_hct, is_dark, platform, contrast_level| {
        spec.neutral_variant_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      },
      &variant,
      &source_color_hct,
      is_dark,
      &platform,
      contrast_level,
    );
    let error_palette = self
      .error_palette
      .tonal_palette_or_none(
        |variant, source_color_hct, is_dark, platform, contrast_level| {
          spec.error_palette(variant, source_color_hct, is_dark, platform, contrast_level)
        },
        &variant,
        &source_color_hct,
        is_dark,
        &platform,
        contrast_level,
      )
      .unwrap_or_else(|| TonalPalette::from_hue_and_chroma(25.0, 84.0));

    let scheme = DynamicScheme::new(
      source_color_hct,
      variant,
      is_dark,
      platform,
      contrast_level,
      spec_version,
      primary_palette,
      secondary_palette,
      tertiary_palette,
      neutral_palette,
      neutral_variant_palette,
      error_palette,
    );
    Ok(scheme)
  }
}

impl From<&DynamicScheme> for DynamicSchemeBuilder {
  fn from(value: &DynamicScheme) -> Self {
    Self::new()
      .source_color(value.source_color_hct().clone())
      .variant(*value.variant())
      .is_dark(value.is_dark())
      .platform(*value.platform())
      .contrast_level(value.contrast_level())
      .spec_version(*value.spec_version())
      .primary_palette(value.primary_palette().clone())
      .secondary_palette(value.secondary_palette().clone())
      .tertiary_palette(value.tertiary_palette().clone())
      .neutral_palette(value.neutral_palette().clone())
      .neutral_variant_palette(value.neutral_variant_palette().clone())
      .error_palette(value.error_palette().clone())
  }
}

impl From<DynamicScheme> for DynamicSchemeBuilder {
  fn from(value: DynamicScheme) -> Self {
    Self::new()
      .source_color(value.source_color_hct)
      .variant(value.variant)
      .is_dark(value.is_dark)
      .platform(value.platform)
      .contrast_level(value.contrast_level)
      .spec_version(value.spec_version)
      .primary_palette(value.primary_palette)
      .secondary_palette(value.secondary_palette)
      .tertiary_palette(value.tertiary_palette)
      .neutral_palette(value.neutral_palette)
      .neutral_variant_palette(value.neutral_variant_palette)
      .error_palette(value.error_palette)
  }
}
