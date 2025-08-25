use std::marker::PhantomData;

use crate::dynamiccolor::{ColorSpec, ColorSpec2025, DynamicColor, DynamicScheme};

pub struct MaterialDynamicColors;

impl MaterialDynamicColors {
  const COLOR_SPEC: ColorSpec2025 = ColorSpec2025::new();

  pub const fn new() -> Self {
    Self
  }

  pub fn highest_surface(&self, s: &DynamicScheme) -> DynamicColor {
    Self::COLOR_SPEC.highest_surface(s)
  }

  ////////////////////////////////////////////////////////////////
  // Main Palettes                                              //
  ////////////////////////////////////////////////////////////////

  pub fn primary_palette_key_color(&self) -> DynamicColor {
    Self::COLOR_SPEC.primary_palette_key_color()
  }

  pub fn secondary_palette_key_color(&self) -> DynamicColor {
    Self::COLOR_SPEC.secondary_palette_key_color()
  }

  pub fn tertiary_palette_key_color(&self) -> DynamicColor {
    Self::COLOR_SPEC.tertiary_palette_key_color()
  }

  pub fn neutral_palette_key_color(&self) -> DynamicColor {
    Self::COLOR_SPEC.neutral_palette_key_color()
  }

  pub fn neutral_variant_palette_key_color(&self) -> DynamicColor {
    Self::COLOR_SPEC.neutral_variant_palette_key_color()
  }

  pub fn error_palette_key_color(&self) -> DynamicColor {
    Self::COLOR_SPEC.error_palette_key_color()
  }

  ////////////////////////////////////////////////////////////////
  // Surfaces [S]                                               //
  ////////////////////////////////////////////////////////////////

  pub fn background(&self) -> DynamicColor {
    Self::COLOR_SPEC.background()
  }

  pub fn on_background(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_background()
  }

  pub fn surface(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface()
  }

  pub fn surface_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_dim()
  }

  pub fn surface_bright(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_bright()
  }

  pub fn surface_container_lowest(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_container_lowest()
  }

  pub fn surface_container_low(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_container_low()
  }

  pub fn surface_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_container()
  }

  pub fn surface_container_high(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_container_high()
  }

  pub fn surface_container_highest(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_container_highest()
  }

  pub fn on_surface(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_surface()
  }

  pub fn surface_variant(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_variant()
  }

  pub fn on_surface_variant(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_surface_variant()
  }

  pub fn inverse_surface(&self) -> DynamicColor {
    Self::COLOR_SPEC.inverse_surface()
  }

  pub fn inverse_on_surface(&self) -> DynamicColor {
    Self::COLOR_SPEC.inverse_on_surface()
  }

  pub fn outline(&self) -> DynamicColor {
    Self::COLOR_SPEC.outline()
  }

  pub fn outline_variant(&self) -> DynamicColor {
    Self::COLOR_SPEC.outline_variant()
  }

  pub fn shadow(&self) -> DynamicColor {
    Self::COLOR_SPEC.shadow()
  }

  pub fn scrim(&self) -> DynamicColor {
    Self::COLOR_SPEC.scrim()
  }

  pub fn surface_tint(&self) -> DynamicColor {
    Self::COLOR_SPEC.surface_tint()
  }

  ////////////////////////////////////////////////////////////////
  // Primaries [P]                                              //
  ////////////////////////////////////////////////////////////////

  pub fn primary(&self) -> DynamicColor {
    Self::COLOR_SPEC.primary()
  }

  pub fn primary_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.primary_dim().unwrap()
  }

  pub fn on_primary(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_primary()
  }

  pub fn primary_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.primary_container()
  }

  pub fn on_primary_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_primary_container()
  }

  pub fn inverse_primary(&self) -> DynamicColor {
    Self::COLOR_SPEC.inverse_primary()
  }

  /////////////////////////////////////////////////////////////////
  // Primary Fixed Colors [PF]                                   //
  /////////////////////////////////////////////////////////////////

  pub fn primary_fixed(&self) -> DynamicColor {
    Self::COLOR_SPEC.primary_fixed()
  }

  pub fn primary_fixed_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.primary_fixed_dim()
  }

  pub fn on_primary_fixed(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_primary_fixed()
  }

  pub fn on_primary_fixed_variant(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_primary_fixed_variant()
  }

  ////////////////////////////////////////////////////////////////
  // Secondaries [Q]                                            //
  ////////////////////////////////////////////////////////////////

  pub fn secondary(&self) -> DynamicColor {
    Self::COLOR_SPEC.secondary()
  }

  pub fn secondary_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.secondary_dim().unwrap()
  }

  pub fn on_secondary(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_secondary()
  }

  pub fn secondary_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.secondary_container()
  }

  pub fn on_secondary_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_secondary_container()
  }

  /////////////////////////////////////////////////////////////////
  // Secondary Fixed Colors [QF]                                 //
  /////////////////////////////////////////////////////////////////

  pub fn secondary_fixed(&self) -> DynamicColor {
    Self::COLOR_SPEC.secondary_fixed()
  }

  pub fn secondary_fixed_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.secondary_fixed_dim()
  }

  pub fn on_secondary_fixed(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_secondary_fixed()
  }

  pub fn on_secondary_fixed_variant(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_secondary_fixed_variant()
  }

  ////////////////////////////////////////////////////////////////
  // Tertiaries [T]                                             //
  ////////////////////////////////////////////////////////////////

  pub fn tertiary(&self) -> DynamicColor {
    Self::COLOR_SPEC.tertiary()
  }

  pub fn tertiary_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.tertiary_dim().unwrap()
  }

  pub fn on_tertiary(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_tertiary()
  }

  pub fn tertiary_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.tertiary_container()
  }

  pub fn on_tertiary_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_tertiary_container()
  }

  /////////////////////////////////////////////////////////////////
  // Tertiary Fixed Colors [TF]                                  //
  /////////////////////////////////////////////////////////////////

  pub fn tertiary_fixed(&self) -> DynamicColor {
    Self::COLOR_SPEC.tertiary_fixed()
  }

  pub fn tertiary_fixed_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.tertiary_fixed_dim()
  }

  pub fn on_tertiary_fixed(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_tertiary_fixed()
  }

  pub fn on_tertiary_fixed_variant(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_tertiary_fixed_variant()
  }

  ////////////////////////////////////////////////////////////////
  // Errors [E]                                                 //
  ////////////////////////////////////////////////////////////////

  pub fn error(&self) -> DynamicColor {
    Self::COLOR_SPEC.error()
  }

  pub fn error_dim(&self) -> DynamicColor {
    Self::COLOR_SPEC.error_dim().unwrap()
  }

  pub fn on_error(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_error()
  }

  pub fn error_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.error_container()
  }

  pub fn on_error_container(&self) -> DynamicColor {
    Self::COLOR_SPEC.on_error_container()
  }

  ////////////////////////////////////////////////////////////////
  // Android-only colors                                        //
  ////////////////////////////////////////////////////////////////

  // These colors were present in Android framework before Android U, and used by MDC controls. They
  // should be avoided, if possible. It's unclear if they're used on multiple backgrounds, and if
  // they are, they can't be adjusted for contrast.* For now, they will be set with no background,
  // and those won't adjust for contrast, avoiding issues.
  //
  // <p>* For example, if the same color is on a white background _and_ black background, there's no
  // way to increase contrast with either without losing contrast with the other.

  // colorControlActivated documented as colorAccent in M3 & GM3.
  // colorAccent documented as colorSecondary in M3 and colorPrimary in GM3.
  // Android used Material's Container as Primary/Secondary/Tertiary at launch.
  // Therefore, this is a duplicated version of Primary Container.
  pub fn control_activated(&self) -> DynamicColor {
    Self::COLOR_SPEC.control_activated()
  }

  // colorControlNormal documented as textColorSecondary in M3 & GM3.
  // In Material, textColorSecondary points to onSurfaceVariant in the non-disabled state,
  // which is Neutral Variant T30/80 in light/dark.
  pub fn control_normal(&self) -> DynamicColor {
    Self::COLOR_SPEC.control_normal()
  }

  // colorControlHighlight documented, in both M3 & GM3:
  // Light mode: #1f000000 dark mode: #33ffffff.
  // These are black and white with some alpha.
  // 1F hex = 31 decimal; 31 / 255 = 12% alpha.
  // 33 hex = 51 decimal; 51 / 255 = 20% alpha.
  // DynamicColors do not support alpha currently, and _may_ not need it for this use case,
  // depending on how MDC resolved alpha for the other cases.
  // Returning black in dark mode, white in light mode.
  pub fn control_highlight(&self) -> DynamicColor {
    Self::COLOR_SPEC.control_highlight()
  }

  // textColorPrimaryInverse documented, in both M3 & GM3, documented as N10/N90.
  pub fn text_primary_inverse(&self) -> DynamicColor {
    Self::COLOR_SPEC.text_primary_inverse()
  }

  // textColorSecondaryInverse and textColorTertiaryInverse both documented, in both M3 & GM3, as
  // NV30/NV80
  pub fn text_secondary_and_tertiary_inverse(&self) -> DynamicColor {
    Self::COLOR_SPEC.text_secondary_and_tertiary_inverse()
  }

  // textColorPrimaryInverseDisableOnly documented, in both M3 & GM3, as N10/N90
  pub fn text_primary_inverse_disable_only(&self) -> DynamicColor {
    Self::COLOR_SPEC.text_primary_inverse_disable_only()
  }

  // textColorSecondaryInverse and textColorTertiaryInverse in disabled state both documented,
  // in both M3 & GM3, as N10/N90
  pub fn text_secondary_and_tertiary_inverse_disabled(&self) -> DynamicColor {
    Self::COLOR_SPEC.text_secondary_and_tertiary_inverse_disabled()
  }

  // textColorHintInverse documented, in both M3 & GM3, as N10/N90
  pub fn text_hint_inverse(&self) -> DynamicColor {
    Self::COLOR_SPEC.text_hint_inverse()
  }

  pub fn all_dynamic_colors(&self) -> Vec<DynamicColor> {
    vec![
      self.primary_palette_key_color(),
      self.secondary_palette_key_color(),
      self.tertiary_palette_key_color(),
      self.neutral_palette_key_color(),
      self.neutral_variant_palette_key_color(),
      self.error_palette_key_color(),
      self.background(),
      self.on_background(),
      self.surface(),
      self.surface_dim(),
      self.surface_bright(),
      self.surface_container_lowest(),
      self.surface_container_low(),
      self.surface_container(),
      self.surface_container_high(),
      self.surface_container_highest(),
      self.on_surface(),
      self.surface_variant(),
      self.on_surface_variant(),
      self.inverse_surface(),
      self.inverse_on_surface(),
      self.outline(),
      self.outline_variant(),
      self.shadow(),
      self.scrim(),
      self.surface_tint(),
      self.primary(),
      self.primary_dim(),
      self.on_primary(),
      self.primary_container(),
      self.on_primary_container(),
      self.inverse_primary(),
      self.primary_fixed(),
      self.primary_fixed_dim(),
      self.on_primary_fixed(),
      self.on_primary_fixed_variant(),
      self.secondary(),
      self.secondary_dim(),
      self.on_secondary(),
      self.secondary_container(),
      self.on_secondary_container(),
      self.secondary_fixed(),
      self.secondary_fixed_dim(),
      self.on_secondary_fixed(),
      self.on_secondary_fixed_variant(),
      self.tertiary(),
      self.tertiary_dim(),
      self.on_tertiary(),
      self.tertiary_container(),
      self.on_tertiary_container(),
      self.tertiary_fixed(),
      self.tertiary_fixed_dim(),
      self.on_tertiary_fixed(),
      self.on_tertiary_fixed_variant(),
      self.error(),
      self.error_dim(),
      self.on_error(),
      self.error_container(),
      self.on_error_container(),
      self.control_activated(),
      self.control_normal(),
      self.control_highlight(),
      self.text_primary_inverse(),
      self.text_secondary_and_tertiary_inverse(),
      self.text_primary_inverse_disable_only(),
      self.text_secondary_and_tertiary_inverse_disabled(),
      self.text_hint_inverse(),
    ]
  }
}

impl Default for MaterialDynamicColors {
  fn default() -> Self {
    Self::new()
  }
}
