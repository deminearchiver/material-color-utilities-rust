use crate::dynamiccolor::{DynamicColor, DynamicScheme};

/// A delegate that provides the dynamic color constraints for
/// [`MaterialDynamicColors`](crate::dynamiccolor::MaterialDynamicColors).
///
/// This is used to allow for different color constraints for different spec
/// versions.
pub trait ColorSpec {
  ////////////////////////////////////////////////////////////////
  // Main Palettes                                              //
  ////////////////////////////////////////////////////////////////

  fn primary_palette_key_color(&self) -> DynamicColor;

  fn secondary_palette_key_color(&self) -> DynamicColor;

  fn tertiary_palette_key_color(&self) -> DynamicColor;

  fn neutral_palette_key_color(&self) -> DynamicColor;

  fn neutral_variant_palette_key_color(&self) -> DynamicColor;

  fn error_palette_key_color(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Surfaces [S]                                               //
  ////////////////////////////////////////////////////////////////

  fn background(&self) -> DynamicColor;

  fn on_background(&self) -> DynamicColor;

  fn surface(&self) -> DynamicColor;

  fn surface_dim(&self) -> DynamicColor;

  fn surface_bright(&self) -> DynamicColor;

  fn surface_container_lowest(&self) -> DynamicColor;

  fn surface_container_low(&self) -> DynamicColor;

  fn surface_container(&self) -> DynamicColor;

  fn surface_container_high(&self) -> DynamicColor;

  fn surface_container_highest(&self) -> DynamicColor;

  fn on_surface(&self) -> DynamicColor;

  fn surface_variant(&self) -> DynamicColor;

  fn on_surface_variant(&self) -> DynamicColor;

  fn inverse_surface(&self) -> DynamicColor;

  fn inverse_on_surface(&self) -> DynamicColor;

  fn outline(&self) -> DynamicColor;

  fn outline_variant(&self) -> DynamicColor;

  fn shadow(&self) -> DynamicColor;

  fn scrim(&self) -> DynamicColor;

  fn surface_tint(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Primaries [P]                                              //
  ////////////////////////////////////////////////////////////////

  fn primary(&self) -> DynamicColor;

  fn primary_dim(&self) -> Option<DynamicColor>;

  fn on_primary(&self) -> DynamicColor;

  fn primary_container(&self) -> DynamicColor;

  fn on_primary_container(&self) -> DynamicColor;

  fn inverse_primary(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Secondaries [Q]                                            //
  ////////////////////////////////////////////////////////////////

  fn secondary(&self) -> DynamicColor;

  fn secondary_dim(&self) -> Option<DynamicColor>;

  fn on_secondary(&self) -> DynamicColor;

  fn secondary_container(&self) -> DynamicColor;

  fn on_secondary_container(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Tertiaries [T]                                             //
  ////////////////////////////////////////////////////////////////

  fn tertiary(&self) -> DynamicColor;

  fn tertiary_dim(&self) -> Option<DynamicColor>;

  fn on_tertiary(&self) -> DynamicColor;

  fn tertiary_container(&self) -> DynamicColor;

  fn on_tertiary_container(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Errors [E]                                                 //
  ////////////////////////////////////////////////////////////////

  fn error(&self) -> DynamicColor;

  fn error_dim(&self) -> Option<DynamicColor>;

  fn on_error(&self) -> DynamicColor;

  fn error_container(&self) -> DynamicColor;

  fn on_error_container(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Primary Fixed Colors [PF]                                  //
  ////////////////////////////////////////////////////////////////

  fn primary_fixed(&self) -> DynamicColor;

  fn primary_fixed_dim(&self) -> DynamicColor;

  fn on_primary_fixed(&self) -> DynamicColor;

  fn on_primary_fixed_variant(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Secondary Fixed Colors [QF]                                //
  ////////////////////////////////////////////////////////////////

  fn secondary_fixed(&self) -> DynamicColor;

  fn secondary_fixed_dim(&self) -> DynamicColor;

  fn on_secondary_fixed(&self) -> DynamicColor;

  fn on_secondary_fixed_variant(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Tertiary Fixed Colors [TF]                                 //
  ////////////////////////////////////////////////////////////////

  fn tertiary_fixed(&self) -> DynamicColor;

  fn tertiary_fixed_dim(&self) -> DynamicColor;

  fn on_tertiary_fixed(&self) -> DynamicColor;

  fn on_tertiary_fixed_variant(&self) -> DynamicColor;

  //////////////////////////////////////////////////////////////////
  // Android-only Colors                                          //
  //////////////////////////////////////////////////////////////////

  fn control_activated(&self) -> DynamicColor;

  fn control_normal(&self) -> DynamicColor;

  fn control_highlight(&self) -> DynamicColor;

  fn text_primary_inverse(&self) -> DynamicColor;

  fn text_secondary_and_tertiary_inverse(&self) -> DynamicColor;

  fn text_primary_inverse_disable_only(&self) -> DynamicColor;

  fn text_secondary_and_tertiary_inverse_disabled(&self) -> DynamicColor;

  fn text_hint_inverse(&self) -> DynamicColor;

  ////////////////////////////////////////////////////////////////
  // Other                                                      //
  ////////////////////////////////////////////////////////////////

  fn highest_surface(&self, s: &DynamicScheme) -> DynamicColor;
}
