use std::{collections::HashMap, hash::Hash};

use material_color_utilities::{
  dynamiccolor::{DynamicColorBuilder, DynamicScheme, DynamicSchemeBuilder},
  palettes::TonalPalette,
  utils::string::{argb_from_css_hex, css_hex_from_argb},
};
use serde::{
  Deserialize, Serialize,
  de::Visitor,
  ser::{SerializeMap, SerializeStruct},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Figma {
  #[serde(default)]
  description: String,
  seed: u32,
  core_colors: CoreColors,
  #[serde(default)]
  extended_colors: Vec<ExtendedColor>,
  schemes: Schemes,
  palettes: Palettes,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreColors {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  primary: Option<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  secondary: Option<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tertiary: Option<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  neutral: Option<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  neutral_variant: Option<u32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  error: Option<u32>,
}

impl CoreColors {
  pub fn primary(&self) -> Option<u32> {
    self.primary.as_ref().copied()
  }

  pub fn set_primary(&mut self, value: Option<u32>) {
    self.primary = value;
  }

  pub fn with_primary(mut self, value: u32) -> Self {
    self.primary = Some(value);
    self
  }

  pub fn with_primary_or(mut self, value: Option<u32>) -> Self {
    self.primary = value;
    self
  }

  pub fn secondary(&self) -> Option<u32> {
    self.secondary.as_ref().copied()
  }

  pub fn set_secondary(&mut self, value: Option<u32>) {
    self.secondary = value;
  }

  pub fn with_secondary(mut self, value: u32) -> Self {
    self.secondary = Some(value);
    self
  }

  pub fn with_secondary_or(mut self, value: Option<u32>) -> Self {
    self.secondary = value;
    self
  }

  pub fn tertiary(&self) -> Option<u32> {
    self.tertiary.as_ref().copied()
  }

  pub fn set_tertiary(&mut self, value: Option<u32>) {
    self.tertiary = value;
  }

  pub fn with_tertiary(mut self, value: u32) -> Self {
    self.tertiary = Some(value);
    self
  }

  pub fn with_tertiary_or(mut self, value: Option<u32>) -> Self {
    self.tertiary = value;
    self
  }
  pub fn neutral(&self) -> Option<u32> {
    self.neutral.as_ref().copied()
  }

  pub fn set_neutral(&mut self, value: Option<u32>) {
    self.neutral = value;
  }

  pub fn with_neutral(mut self, value: u32) -> Self {
    self.neutral = Some(value);
    self
  }

  pub fn with_neutral_or(mut self, value: Option<u32>) -> Self {
    self.neutral = value;
    self
  }
  pub fn neutral_variant(&self) -> Option<u32> {
    self.neutral_variant.as_ref().copied()
  }

  pub fn set_neutral_variant(&mut self, value: Option<u32>) {
    self.neutral_variant = value;
  }

  pub fn with_neutral_variant(mut self, value: u32) -> Self {
    self.neutral_variant = Some(value);
    self
  }

  pub fn with_neutral_variant_or(mut self, value: Option<u32>) -> Self {
    self.neutral = value;
    self
  }
  pub fn error(&self) -> Option<u32> {
    self.error.as_ref().copied()
  }

  pub fn set_error(&mut self, value: Option<u32>) {
    self.error = value;
  }

  pub fn with_error(mut self, value: u32) -> Self {
    self.error = Some(value);
    self
  }

  pub fn with_error_or(mut self, value: Option<u32>) -> Self {
    self.neutral = value;
    self
  }
}

impl From<&DynamicScheme> for CoreColors {
  fn from(scheme: &DynamicScheme) -> Self {
    Self::default()
      .with_primary(scheme.primary_palette_key_color())
      .with_secondary(scheme.secondary_palette_key_color())
      .with_tertiary(scheme.tertiary_palette_key_color())
      .with_tertiary(scheme.tertiary_palette_key_color())
      .with_neutral(scheme.neutral_palette_key_color())
      .with_neutral_variant(scheme.neutral_variant_palette_key_color())
      .with_error(scheme.error_palette_key_color())
  }
}

#[derive(Debug, Default, Serialize)]
pub struct ExtendedColor {
  name: String,
  color: u32,
  description: String,
  harmonized: bool,
}

impl ExtendedColor {
  pub fn new(name: impl Into<String>, color: u32) -> Self {
    Self {
      name: name.into(),
      color,
      description: "".into(),
      harmonized: false,
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn color(&self) -> u32 {
    self.color
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn harmonized(&self) -> bool {
    self.harmonized
  }

  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  pub fn with_color(mut self, color: u32) -> Self {
    self.color = color;
    self
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = description.into();
    self
  }

  pub fn with_harmonized(mut self, harmonized: bool) -> Self {
    self.harmonized = harmonized;
    self
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Schemes {
  light: Scheme,
  light_medium_contrast: Scheme,
  light_high_contrast: Scheme,
  dark: Scheme,
  dark_medium_contrast: Scheme,
  dark_high_contrast: Scheme,
}

impl TryFrom<&DynamicSchemeBuilder> for Schemes {
  type Error = String;

  fn try_from(builder: &DynamicSchemeBuilder) -> Result<Self, Self::Error> {
    let light = builder.clone().is_dark(false).contrast_level(0.0);
    let light_medium_contrast = light.clone().contrast_level(0.5);
    let light_high_contrast = light.clone().contrast_level(1.0);
    let dark = light.clone().is_dark(true);
    let dark_medium_contrast = dark.clone().contrast_level(0.5);
    let dark_high_contrast = dark.clone().contrast_level(1.0);
    let schemes = Self {
      light: light.try_into()?,
      light_medium_contrast: light_medium_contrast.try_into()?,
      light_high_contrast: light_high_contrast.try_into()?,
      dark: dark.try_into()?,
      dark_medium_contrast: dark_medium_contrast.try_into()?,
      dark_high_contrast: dark_high_contrast.try_into()?,
    };
    Ok(schemes)
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scheme {
  primary_palette_key_color: u32,
  secondary_palette_key_color: u32,
  tertiary_palette_key_color: u32,
  neutral_palette_key_color: u32,
  neutral_variant_palette_key_color: u32,
  error_palette_key_color: u32,
  background: u32,
  on_background: u32,
  surface: u32,
  surface_dim: u32,
  surface_bright: u32,
  surface_container_lowest: u32,
  surface_container_low: u32,
  surface_container: u32,
  surface_container_high: u32,
  surface_container_highest: u32,
  on_surface: u32,
  surface_variant: u32,
  on_surface_variant: u32,
  inverse_surface: u32,
  inverse_on_surface: u32,
  outline: u32,
  outline_variant: u32,
  shadow: u32,
  scrim: u32,
  surface_tint: u32,
  primary: u32,
  primary_dim: u32,
  on_primary: u32,
  primary_container: u32,
  on_primary_container: u32,
  inverse_primary: u32,
  primary_fixed: u32,
  primary_fixed_dim: u32,
  on_primary_fixed: u32,
  on_primary_fixed_variant: u32,
  secondary: u32,
  secondary_dim: u32,
  on_secondary: u32,
  secondary_container: u32,
  on_secondary_container: u32,
  secondary_fixed: u32,
  secondary_fixed_dim: u32,
  on_secondary_fixed: u32,
  on_secondary_fixed_variant: u32,
  tertiary: u32,
  tertiary_dim: u32,
  on_tertiary: u32,
  tertiary_container: u32,
  on_tertiary_container: u32,
  tertiary_fixed: u32,
  tertiary_fixed_dim: u32,
  on_tertiary_fixed: u32,
  on_tertiary_fixed_variant: u32,
  error: u32,
  error_dim: u32,
  on_error: u32,
  error_container: u32,
  on_error_container: u32,
  control_activated: u32,
  control_normal: u32,
  control_highlight: u32,
  text_primary_inverse: u32,
  text_secondary_and_tertiary_inverse: u32,
  text_primary_inverse_disable_only: u32,
  text_secondary_and_tertiary_inverse_disabled: u32,
  text_hint_inverse: u32,
}

impl From<&DynamicScheme> for Scheme {
  fn from(scheme: &DynamicScheme) -> Self {
    Self {
      primary_palette_key_color: scheme.primary_palette_key_color(),
      secondary_palette_key_color: scheme.secondary_palette_key_color(),
      tertiary_palette_key_color: scheme.tertiary_palette_key_color(),
      neutral_palette_key_color: scheme.neutral_palette_key_color(),
      neutral_variant_palette_key_color: scheme.neutral_variant_palette_key_color(),
      error_palette_key_color: scheme.error_palette_key_color(),
      background: scheme.background(),
      on_background: scheme.on_background(),
      surface: scheme.surface(),
      surface_dim: scheme.surface_dim(),
      surface_bright: scheme.surface_bright(),
      surface_container_lowest: scheme.surface_container_lowest(),
      surface_container_low: scheme.surface_container_low(),
      surface_container: scheme.surface_container(),
      surface_container_high: scheme.surface_container_high(),
      surface_container_highest: scheme.surface_container_highest(),
      on_surface: scheme.on_surface(),
      surface_variant: scheme.surface_variant(),
      on_surface_variant: scheme.on_surface_variant(),
      inverse_surface: scheme.inverse_surface(),
      inverse_on_surface: scheme.inverse_on_surface(),
      outline: scheme.outline(),
      outline_variant: scheme.outline_variant(),
      shadow: scheme.shadow(),
      scrim: scheme.scrim(),
      surface_tint: scheme.surface_tint(),
      primary: scheme.primary(),
      primary_dim: scheme.primary_dim(),
      on_primary: scheme.on_primary(),
      primary_container: scheme.primary_container(),
      on_primary_container: scheme.on_primary_container(),
      inverse_primary: scheme.inverse_primary(),
      primary_fixed: scheme.primary_fixed(),
      primary_fixed_dim: scheme.primary_fixed_dim(),
      on_primary_fixed: scheme.on_primary_fixed(),
      on_primary_fixed_variant: scheme.on_primary_fixed_variant(),
      secondary: scheme.secondary(),
      secondary_dim: scheme.secondary_dim(),
      on_secondary: scheme.on_secondary(),
      secondary_container: scheme.secondary_container(),
      on_secondary_container: scheme.on_secondary_container(),
      secondary_fixed: scheme.secondary_fixed(),
      secondary_fixed_dim: scheme.secondary_fixed_dim(),
      on_secondary_fixed: scheme.on_secondary_fixed(),
      on_secondary_fixed_variant: scheme.on_secondary_fixed_variant(),
      tertiary: scheme.tertiary(),
      tertiary_dim: scheme.tertiary_dim(),
      on_tertiary: scheme.on_tertiary(),
      tertiary_container: scheme.tertiary_container(),
      on_tertiary_container: scheme.on_tertiary_container(),
      tertiary_fixed: scheme.tertiary_fixed(),
      tertiary_fixed_dim: scheme.tertiary_fixed_dim(),
      on_tertiary_fixed: scheme.on_tertiary_fixed(),
      on_tertiary_fixed_variant: scheme.on_tertiary_fixed_variant(),
      error: scheme.error(),
      error_dim: scheme.error_dim(),
      on_error: scheme.on_error(),
      error_container: scheme.error_container(),
      on_error_container: scheme.on_error_container(),
      control_activated: scheme.control_activated(),
      control_normal: scheme.control_normal(),
      control_highlight: scheme.control_highlight(),
      text_primary_inverse: scheme.text_primary_inverse(),
      text_secondary_and_tertiary_inverse: scheme.text_secondary_and_tertiary_inverse(),
      text_primary_inverse_disable_only: scheme.text_primary_inverse_disable_only(),
      text_secondary_and_tertiary_inverse_disabled: scheme
        .text_secondary_and_tertiary_inverse_disabled(),
      text_hint_inverse: scheme.text_hint_inverse(),
    }
  }
}

impl TryFrom<DynamicSchemeBuilder> for Scheme {
  type Error = String;

  fn try_from(builder: DynamicSchemeBuilder) -> Result<Self, Self::Error> {
    let scheme = builder.build()?;
    Ok(Scheme::from(&scheme))
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Palettes {
  #[serde(skip_serializing_if = "Option::is_none")]
  primary: Option<Palette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  secondary: Option<Palette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  tertiary: Option<Palette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  neutral: Option<Palette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  neutral_variant: Option<Palette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  error: Option<Palette>,
}

impl From<&DynamicScheme> for Palettes {
  fn from(value: &DynamicScheme) -> Self {
    Self {
      primary: Some(value.primary_palette().into()),
      secondary: Some(value.primary_palette().into()),
      tertiary: Some(value.primary_palette().into()),
      neutral: Some(value.primary_palette().into()),
      neutral_variant: Some(value.primary_palette().into()),
      error: Some(value.primary_palette().into()),
    }
  }
}

#[derive(Debug, Default)]
pub struct Palette(HashMap<u8, u32>);

impl Palette {
  pub fn get(&self, tone: u8) -> Option<u32> {
    self.0.get(&tone).copied()
  }

  pub fn inner(&self) -> &HashMap<u8, u32> {
    &self.0
  }

  pub fn into_inner(self) -> HashMap<u8, u32> {
    self.0
  }
}

impl From<HashMap<u8, u32>> for Palette {
  fn from(value: HashMap<u8, u32>) -> Self {
    Self(value)
  }
}

impl From<&TonalPalette> for Palette {
  fn from(value: &TonalPalette) -> Self {
    let tones: HashMap<_, _> = [
      0, 5, 10, 15, 20, 25, 30, 35, 40, 50, 60, 70, 80, 90, 95, 98, 99, 100,
    ]
    .into_iter()
    .map(|tone| (tone, value.tone(tone)))
    .collect();
    tones.into()
  }
}

impl Serialize for Palette {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_map(Some(self.0.len()))?;
    let mut value: Vec<_> = self.0.iter().collect();
    value.sort_by_key(|&(&key, _)| key);
    for (&tone, &argb) in value {
      let hex = css_hex_from_argb(argb);
      state.serialize_entry(&tone, &hex)?;
    }
    state.end()
  }
}

#[cfg(test)]
mod tests {
  use material_color_utilities::dynamiccolor::DynamicSchemeBuilder;

  use super::*;

  #[test]
  fn test() {
    let scheme = DynamicSchemeBuilder::new().is_dark(false).build().unwrap();
    let palette: Palette = scheme.primary_palette().into();
    println!("{}", serde_json::to_string_pretty(&palette).unwrap());
  }
}
