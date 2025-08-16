use std::collections::HashMap;

use csscolorparser::Color;
use material_color_utilities::{
  dynamiccolor::{DynamicScheme, DynamicSchemeBuilder},
  palettes::TonalPalette,
  utils::string::{FromArgb, css_hex_from_argb},
};
use serde::{Deserialize, Serialize, ser::SerializeMap};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTheme {
  description: String,
  seed: Color,
  core_colors: CoreColors,
  extended_colors: Vec<ExtendedColor>,
  schemes: Schemes,
  palettes: Palettes,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreColors {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  primary: Option<Color>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  secondary: Option<Color>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tertiary: Option<Color>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  neutral: Option<Color>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  neutral_variant: Option<Color>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  error: Option<Color>,
}

#[derive(Debug, Deserialize, Serialize)]
// TODO: decide if rename_all should be added
pub struct ExtendedColor {
  name: String,
  color: Color,
  description: String,
  harmonized: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Schemes {
  light: Scheme,
  light_medium_contrast: Scheme,
  light_high_contrast: Scheme,
  dark: Scheme,
  dark_medium_contrast: Scheme,
  dark_high_contrast: Scheme,
}

impl From<&DynamicSchemeBuilder> for Schemes {
  fn from(builder: &DynamicSchemeBuilder) -> Self {
    let light = builder.clone().is_dark(false).contrast_level(0.0);
    let light_medium_contrast = light.clone().contrast_level(0.5);
    let light_high_contrast = light.clone().contrast_level(1.0);
    let dark = light.clone().is_dark(true);
    let dark_medium_contrast = dark.clone().contrast_level(0.5);
    let dark_high_contrast = dark.clone().contrast_level(1.0);
    Self {
      light: light.into(),
      light_medium_contrast: light_medium_contrast.into(),
      light_high_contrast: light_high_contrast.into(),
      dark: dark.into(),
      dark_medium_contrast: dark_medium_contrast.into(),
      dark_high_contrast: dark_high_contrast.into(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scheme {
  primary_palette_key_color: Color,
  secondary_palette_key_color: Color,
  tertiary_palette_key_color: Color,
  neutral_palette_key_color: Color,
  neutral_variant_palette_key_color: Color,
  error_palette_key_color: Color,
  background: Color,
  on_background: Color,
  surface: Color,
  surface_dim: Color,
  surface_bright: Color,
  surface_container_lowest: Color,
  surface_container_low: Color,
  surface_container: Color,
  surface_container_high: Color,
  surface_container_highest: Color,
  on_surface: Color,
  surface_variant: Color,
  on_surface_variant: Color,
  inverse_surface: Color,
  inverse_on_surface: Color,
  outline: Color,
  outline_variant: Color,
  shadow: Color,
  scrim: Color,
  surface_tint: Color,
  primary: Color,
  primary_dim: Color,
  on_primary: Color,
  primary_container: Color,
  on_primary_container: Color,
  inverse_primary: Color,
  primary_fixed: Color,
  primary_fixed_dim: Color,
  on_primary_fixed: Color,
  on_primary_fixed_variant: Color,
  secondary: Color,
  secondary_dim: Color,
  on_secondary: Color,
  secondary_container: Color,
  on_secondary_container: Color,
  secondary_fixed: Color,
  secondary_fixed_dim: Color,
  on_secondary_fixed: Color,
  on_secondary_fixed_variant: Color,
  tertiary: Color,
  tertiary_dim: Color,
  on_tertiary: Color,
  tertiary_container: Color,
  on_tertiary_container: Color,
  tertiary_fixed: Color,
  tertiary_fixed_dim: Color,
  on_tertiary_fixed: Color,
  on_tertiary_fixed_variant: Color,
  error: Color,
  error_dim: Color,
  on_error: Color,
  error_container: Color,
  on_error_container: Color,
  control_activated: Color,
  control_normal: Color,
  control_highlight: Color,
  text_primary_inverse: Color,
  text_secondary_and_tertiary_inverse: Color,
  text_primary_inverse_disable_only: Color,
  text_secondary_and_tertiary_inverse_disabled: Color,
  text_hint_inverse: Color,
}

impl From<&DynamicScheme> for Scheme {
  fn from(scheme: &DynamicScheme) -> Self {
    Self {
      primary_palette_key_color: Color::from_argb(scheme.primary_palette_key_color()),
      secondary_palette_key_color: Color::from_argb(scheme.secondary_palette_key_color()),
      tertiary_palette_key_color: Color::from_argb(scheme.tertiary_palette_key_color()),
      neutral_palette_key_color: Color::from_argb(scheme.neutral_palette_key_color()),
      neutral_variant_palette_key_color: Color::from_argb(
        scheme.neutral_variant_palette_key_color(),
      ),
      error_palette_key_color: Color::from_argb(scheme.error_palette_key_color()),
      background: Color::from_argb(scheme.background()),
      on_background: Color::from_argb(scheme.on_background()),
      surface: Color::from_argb(scheme.surface()),
      surface_dim: Color::from_argb(scheme.surface_dim()),
      surface_bright: Color::from_argb(scheme.surface_bright()),
      surface_container_lowest: Color::from_argb(scheme.surface_container_lowest()),
      surface_container_low: Color::from_argb(scheme.surface_container_low()),
      surface_container: Color::from_argb(scheme.surface_container()),
      surface_container_high: Color::from_argb(scheme.surface_container_high()),
      surface_container_highest: Color::from_argb(scheme.surface_container_highest()),
      on_surface: Color::from_argb(scheme.on_surface()),
      surface_variant: Color::from_argb(scheme.surface_variant()),
      on_surface_variant: Color::from_argb(scheme.on_surface_variant()),
      inverse_surface: Color::from_argb(scheme.inverse_surface()),
      inverse_on_surface: Color::from_argb(scheme.inverse_on_surface()),
      outline: Color::from_argb(scheme.outline()),
      outline_variant: Color::from_argb(scheme.outline_variant()),
      shadow: Color::from_argb(scheme.shadow()),
      scrim: Color::from_argb(scheme.scrim()),
      surface_tint: Color::from_argb(scheme.surface_tint()),
      primary: Color::from_argb(scheme.primary()),
      primary_dim: Color::from_argb(scheme.primary_dim()),
      on_primary: Color::from_argb(scheme.on_primary()),
      primary_container: Color::from_argb(scheme.primary_container()),
      on_primary_container: Color::from_argb(scheme.on_primary_container()),
      inverse_primary: Color::from_argb(scheme.inverse_primary()),
      primary_fixed: Color::from_argb(scheme.primary_fixed()),
      primary_fixed_dim: Color::from_argb(scheme.primary_fixed_dim()),
      on_primary_fixed: Color::from_argb(scheme.on_primary_fixed()),
      on_primary_fixed_variant: Color::from_argb(scheme.on_primary_fixed_variant()),
      secondary: Color::from_argb(scheme.secondary()),
      secondary_dim: Color::from_argb(scheme.secondary_dim()),
      on_secondary: Color::from_argb(scheme.on_secondary()),
      secondary_container: Color::from_argb(scheme.secondary_container()),
      on_secondary_container: Color::from_argb(scheme.on_secondary_container()),
      secondary_fixed: Color::from_argb(scheme.secondary_fixed()),
      secondary_fixed_dim: Color::from_argb(scheme.secondary_fixed_dim()),
      on_secondary_fixed: Color::from_argb(scheme.on_secondary_fixed()),
      on_secondary_fixed_variant: Color::from_argb(scheme.on_secondary_fixed_variant()),
      tertiary: Color::from_argb(scheme.tertiary()),
      tertiary_dim: Color::from_argb(scheme.tertiary_dim()),
      on_tertiary: Color::from_argb(scheme.on_tertiary()),
      tertiary_container: Color::from_argb(scheme.tertiary_container()),
      on_tertiary_container: Color::from_argb(scheme.on_tertiary_container()),
      tertiary_fixed: Color::from_argb(scheme.tertiary_fixed()),
      tertiary_fixed_dim: Color::from_argb(scheme.tertiary_fixed_dim()),
      on_tertiary_fixed: Color::from_argb(scheme.on_tertiary_fixed()),
      on_tertiary_fixed_variant: Color::from_argb(scheme.on_tertiary_fixed_variant()),
      error: Color::from_argb(scheme.error()),
      error_dim: Color::from_argb(scheme.error_dim()),
      on_error: Color::from_argb(scheme.on_error()),
      error_container: Color::from_argb(scheme.error_container()),
      on_error_container: Color::from_argb(scheme.on_error_container()),
      control_activated: Color::from_argb(scheme.control_activated()),
      control_normal: Color::from_argb(scheme.control_normal()),
      control_highlight: Color::from_argb(scheme.control_highlight()),
      text_primary_inverse: Color::from_argb(scheme.text_primary_inverse()),
      text_secondary_and_tertiary_inverse: Color::from_argb(
        scheme.text_secondary_and_tertiary_inverse(),
      ),
      text_primary_inverse_disable_only: Color::from_argb(
        scheme.text_primary_inverse_disable_only(),
      ),
      text_secondary_and_tertiary_inverse_disabled: Color::from_argb(
        scheme.text_secondary_and_tertiary_inverse_disabled(),
      ),
      text_hint_inverse: Color::from_argb(scheme.text_hint_inverse()),
    }
  }
}

impl From<DynamicSchemeBuilder> for Scheme {
  fn from(value: DynamicSchemeBuilder) -> Self {
    Self::from(&value.build())
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

impl From<DynamicSchemeBuilder> for Palettes {
  fn from(value: DynamicSchemeBuilder) -> Self {
    Self::from(&value.build())
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
  use material_color_utilities::{
    dynamiccolor::{DynamicSchemeBuilder, Platform, SpecVersion, Variant},
    hct::Hct,
  };

  use super::*;

  #[test]
  fn test() {
    let argb = 0xFFFF0000;
    let builder = DynamicSchemeBuilder::default()
      .is_dark(false)
      .source_color_hct(Hct::from_int(argb))
      .variant(Variant::Vibrant)
      .platform(Platform::Watch)
      .spec_version(SpecVersion::Spec2025);

    // let palettes: Palettes = builder.clone().try_into().unwrap();
    // println!("{}", serde_json::to_string_pretty(&palettes).unwrap());
    // let schemes: Schemes = (&builder).try_into().unwrap();
    // println!("{}", serde_json::to_string_pretty(&schemes).unwrap());

    let material_theme = MaterialTheme {
      description: "".into(),
      seed: Color::from_argb(argb),
      core_colors: CoreColors {
        primary: Color::from_argb(argb).into(),
        ..Default::default()
      },
      extended_colors: vec![],
      schemes: (&builder).try_into().unwrap(),
      palettes: builder.clone().try_into().unwrap(),
    };
    println!("{}", serde_json::to_string_pretty(&material_theme).unwrap());
  }
}
