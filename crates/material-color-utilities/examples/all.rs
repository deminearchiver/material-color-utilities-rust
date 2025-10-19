use std::{
  fs::{self, File},
  io::Write,
  path::PathBuf,
};

use indicatif::ProgressBar;
use material_color_utilities::{
  dynamiccolor::{
    DynamicScheme, DynamicSchemeBuilder, MaterialDynamicColors, Platform, SpecVersion, Variant,
  },
  palettes::TonalPalette,
  utils::color::is_opaque,
};
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlatformAndSpecVersion {
  Phone2021,
  Phone2025,
  Watch2025,
}

impl PlatformAndSpecVersion {
  pub const fn platform(&self) -> Platform {
    match *self {
      PlatformAndSpecVersion::Phone2021 | PlatformAndSpecVersion::Phone2025 => Platform::Phone,
      PlatformAndSpecVersion::Watch2025 => Platform::Watch,
    }
  }

  pub const fn spec_version(&self) -> SpecVersion {
    match *self {
      PlatformAndSpecVersion::Phone2021 => SpecVersion::Spec2021,
      PlatformAndSpecVersion::Phone2025 | PlatformAndSpecVersion::Watch2025 => {
        SpecVersion::Spec2025
      }
    }
  }
}

const VARIANTS: [Variant; 9] = [
  Variant::Monochrome,
  Variant::Neutral,
  Variant::TonalSpot,
  Variant::Vibrant,
  Variant::Expressive,
  Variant::Fidelity,
  Variant::Content,
  Variant::Rainbow,
  Variant::FruitSalad,
];
const PLATFORMS_AND_SPEC_VERSIONS: [PlatformAndSpecVersion; 3] = [
  PlatformAndSpecVersion::Phone2021,
  PlatformAndSpecVersion::Phone2025,
  PlatformAndSpecVersion::Watch2025,
];

pub fn template_name(template: &Template) -> String {
  match *template {
    Template::Baseline => "baseline",
    Template::NowInAndroid => "nowinandroid",
    Template::LocalSend => "localsend",
  }
  .into()
}

pub fn variant_name(variant: &Variant) -> String {
  match *variant {
    Variant::Monochrome => "monochrome",
    Variant::Neutral => "neutral",
    Variant::TonalSpot => "tonalspot",
    Variant::Vibrant => "vibrant",
    Variant::Expressive => "expressive",
    Variant::Fidelity => "fidelity",
    Variant::Content => "content",
    Variant::Rainbow => "rainbow",
    Variant::FruitSalad => "fruitsalad",
  }
  .into()
}
pub fn platform_name(platform: &Platform) -> String {
  match *platform {
    Platform::Phone => "phone",
    Platform::Watch => "watch",
  }
  .into()
}
pub fn spec_version_name(spec_version: &SpecVersion) -> String {
  match *spec_version {
    SpecVersion::Spec2021 => "2021",
    SpecVersion::Spec2025 => "2025",
  }
  .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Template {
  Baseline,
  NowInAndroid,
  LocalSend,
}

impl Template {
  pub fn source_color(&self) -> u32 {
    match *self {
      Template::Baseline => 0xff6750a4,
      Template::NowInAndroid => 0xff8c4190,
      Template::LocalSend => 0xff009688,
    }
  }

  pub fn primary_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xff8c4190),
      _ => None,
    }
  }

  pub fn secondary_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xffff8456),
      _ => None,
    }
  }

  pub fn tertiary_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xffb3e9ff),
      _ => None,
    }
  }

  pub fn neutral_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xff201a1b),
      _ => None,
    }
  }

  pub fn neutral_variant_palette_key_color(&self) -> Option<u32> {
    None
  }

  pub fn error_palette_key_color(&self) -> Option<u32> {
    None
  }
}

const TEMPLATES: [Template; 3] = [
  Template::Baseline,
  Template::NowInAndroid,
  Template::LocalSend,
];

fn main() {
  let total = TEMPLATES.len() * VARIANTS.len() * PLATFORMS_AND_SPEC_VERSIONS.len();
  let bar = ProgressBar::new(total as u64);
  for template in TEMPLATES {
    for variant in VARIANTS {
      for platform_and_spec_version in PLATFORMS_AND_SPEC_VERSIONS {
        let platform = platform_and_spec_version.platform();
        let spec_version = platform_and_spec_version.spec_version();

        let mut builder = DynamicSchemeBuilder::default()
          .source_color_hct(template.source_color().into())
          .variant(variant)
          .platform(platform)
          .spec_version(spec_version);

        if let Some(argb) = template.primary_palette_key_color() {
          builder = builder.primary_palette_key_color(argb.into());
        }
        if let Some(argb) = template.secondary_palette_key_color() {
          builder = builder.secondary_palette_key_color(argb.into());
        }
        if let Some(argb) = template.tertiary_palette_key_color() {
          builder = builder.tertiary_palette_key_color(argb.into());
        }
        if let Some(argb) = template.neutral_palette_key_color() {
          builder = builder.neutral_palette_key_color(argb.into());
        }
        if let Some(argb) = template.neutral_variant_palette_key_color() {
          builder = builder.neutral_variant_palette_key_color(argb.into());
        }
        if let Some(argb) = template.error_palette_key_color() {
          builder = builder.error_palette_key_color(argb.into());
        }
        let figma_core_colors = FigmaCoreColors {
          primary: template
            .primary_palette_key_color()
            .unwrap_or_else(|| template.source_color())
            .into(),
          secondary: template.secondary_palette_key_color(),
          tertiary: template.tertiary_palette_key_color(),
          neutral: template.neutral_palette_key_color(),
          neutral_variant: template.neutral_variant_palette_key_color(),
          error: template.error_palette_key_color(),
        };
        let figma_schemes = FigmaSchemes::from(&builder);
        let figma = Figma {
          description: "TYPE: CUSTOM\nMaterial Theme Builder export".into(),
          seed: template.source_color(),
          core_colors: figma_core_colors,
          extended_colors: vec![],
          schemes: figma_schemes,
          palettes: FigmaPalettes::default(),
        };
        let figma_json = serde_json::to_string_pretty(&figma).unwrap();

        let file_name = [
          template_name(&template),
          variant_name(&variant),
          platform_name(&platform),
          spec_version_name(&spec_version),
        ]
        .join("_");

        let dir_path = PathBuf::from("./gen/figma");
        fs::create_dir_all(&dir_path).unwrap();
        let file_path = dir_path.join(file_name).with_extension("json");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", figma_json.as_str()).unwrap();
        bar.inc(1);
      }
    }
  }
  bar.finish();
}

pub fn hex_from_argb(argb: &u32) -> String {
  format!(
    "#{:0>6X}",
    if is_opaque(*argb) {
      *argb & 0x00ffffff
    } else {
      *argb
    }
  )
}

fn serialize_argb_to_hex<'a, T, S>(argb: T, s: S) -> Result<S::Ok, S::Error>
where
  T: Into<Option<&'a u32>>,
  S: Serializer,
{
  let argb: Option<&u32> = argb.into();
  match argb {
    Some(argb) => {
      let hex = hex_from_argb(argb);
      s.serialize_str(hex.as_str())
    }
    None => s.serialize_none(),
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Figma {
  description: String,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  seed: u32,
  core_colors: FigmaCoreColors,
  extended_colors: Vec<FigmaExtendedColor>,
  schemes: FigmaSchemes,
  palettes: FigmaPalettes,
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaCoreColors {
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_argb_to_hex"
  )]
  primary: Option<u32>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_argb_to_hex"
  )]
  secondary: Option<u32>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_argb_to_hex"
  )]
  tertiary: Option<u32>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_argb_to_hex"
  )]
  neutral: Option<u32>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_argb_to_hex"
  )]
  neutral_variant: Option<u32>,
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_argb_to_hex"
  )]
  error: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaExtendedColor {
  name: String,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  color: u32,
  description: String,
  harmonized: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FigmaSchemes {
  light: FigmaScheme,
  light_medium_contrast: FigmaScheme,
  light_high_contrast: FigmaScheme,
  dark: FigmaScheme,
  dark_medium_contrast: FigmaScheme,
  dark_high_contrast: FigmaScheme,
}

impl From<&DynamicScheme> for FigmaSchemes {
  fn from(value: &DynamicScheme) -> Self {
    let builder = DynamicSchemeBuilder::from(value);
    Self::from(&builder)
  }
}

impl From<&DynamicSchemeBuilder> for FigmaSchemes {
  fn from(builder: &DynamicSchemeBuilder) -> Self {
    let light = builder.clone().is_dark(false).contrast_level(0.0).build();
    let light_medium_contrast = builder.clone().is_dark(false).contrast_level(0.5).build();
    let light_high_contrast = builder.clone().is_dark(false).contrast_level(1.0).build();
    let dark = builder.clone().is_dark(true).contrast_level(0.0).build();
    let dark_medium_contrast = builder.clone().is_dark(true).contrast_level(0.5).build();
    let dark_high_contrast = builder.clone().is_dark(true).contrast_level(1.0).build();
    Self {
      light: FigmaScheme::from(&light),
      light_medium_contrast: FigmaScheme::from(&light_medium_contrast),
      light_high_contrast: FigmaScheme::from(&light_high_contrast),
      dark: FigmaScheme::from(&dark),
      dark_medium_contrast: FigmaScheme::from(&dark_medium_contrast),
      dark_high_contrast: FigmaScheme::from(&dark_high_contrast),
    }
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaScheme {
  #[serde(serialize_with = "serialize_argb_to_hex")]
  primary_palette_key_color: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  secondary_palette_key_color: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  tertiary_palette_key_color: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  neutral_palette_key_color: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  neutral_variant_palette_key_color: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  error_palette_key_color: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  background: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_background: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_bright: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_container_lowest: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_container_low: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_container_high: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_container_highest: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_surface: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_variant: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_surface_variant: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  inverse_surface: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  inverse_on_surface: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  outline: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  outline_variant: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  shadow: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  scrim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  surface_tint: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  primary: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  primary_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_primary: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  primary_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_primary_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  inverse_primary: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  primary_fixed: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  primary_fixed_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_primary_fixed: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_primary_fixed_variant: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  secondary: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  secondary_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_secondary: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  secondary_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_secondary_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  secondary_fixed: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  secondary_fixed_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_secondary_fixed: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_secondary_fixed_variant: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  tertiary: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  tertiary_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_tertiary: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  tertiary_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_tertiary_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  tertiary_fixed: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  tertiary_fixed_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_tertiary_fixed: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_tertiary_fixed_variant: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  error: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  error_dim: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_error: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  error_container: u32,
  #[serde(serialize_with = "serialize_argb_to_hex")]
  on_error_container: u32,
}

impl From<&DynamicScheme> for FigmaScheme {
  fn from(value: &DynamicScheme) -> Self {
    const MATERIAL_DYNAMIC_COLORS: MaterialDynamicColors = MaterialDynamicColors::new();
    Self {
      primary_palette_key_color: MATERIAL_DYNAMIC_COLORS
        .primary_palette_key_color()
        .get_argb(value),
      secondary_palette_key_color: MATERIAL_DYNAMIC_COLORS
        .secondary_palette_key_color()
        .get_argb(value),
      tertiary_palette_key_color: MATERIAL_DYNAMIC_COLORS
        .tertiary_palette_key_color()
        .get_argb(value),
      neutral_palette_key_color: MATERIAL_DYNAMIC_COLORS
        .neutral_palette_key_color()
        .get_argb(value),
      neutral_variant_palette_key_color: MATERIAL_DYNAMIC_COLORS
        .neutral_variant_palette_key_color()
        .get_argb(value),
      error_palette_key_color: MATERIAL_DYNAMIC_COLORS
        .error_palette_key_color()
        .get_argb(value),
      background: MATERIAL_DYNAMIC_COLORS.background().get_argb(value),
      on_background: MATERIAL_DYNAMIC_COLORS.on_background().get_argb(value),
      surface: MATERIAL_DYNAMIC_COLORS.surface().get_argb(value),
      surface_dim: MATERIAL_DYNAMIC_COLORS.surface_dim().get_argb(value),
      surface_bright: MATERIAL_DYNAMIC_COLORS.surface_bright().get_argb(value),
      surface_container_lowest: MATERIAL_DYNAMIC_COLORS
        .surface_container_lowest()
        .get_argb(value),
      surface_container_low: MATERIAL_DYNAMIC_COLORS
        .surface_container_low()
        .get_argb(value),
      surface_container: MATERIAL_DYNAMIC_COLORS.surface_container().get_argb(value),
      surface_container_high: MATERIAL_DYNAMIC_COLORS
        .surface_container_high()
        .get_argb(value),
      surface_container_highest: MATERIAL_DYNAMIC_COLORS
        .surface_container_highest()
        .get_argb(value),
      on_surface: MATERIAL_DYNAMIC_COLORS.on_surface().get_argb(value),
      surface_variant: MATERIAL_DYNAMIC_COLORS.surface_variant().get_argb(value),
      on_surface_variant: MATERIAL_DYNAMIC_COLORS.on_surface_variant().get_argb(value),
      inverse_surface: MATERIAL_DYNAMIC_COLORS.inverse_surface().get_argb(value),
      inverse_on_surface: MATERIAL_DYNAMIC_COLORS.inverse_on_surface().get_argb(value),
      outline: MATERIAL_DYNAMIC_COLORS.outline().get_argb(value),
      outline_variant: MATERIAL_DYNAMIC_COLORS.outline_variant().get_argb(value),
      shadow: MATERIAL_DYNAMIC_COLORS.shadow().get_argb(value),
      scrim: MATERIAL_DYNAMIC_COLORS.scrim().get_argb(value),
      surface_tint: MATERIAL_DYNAMIC_COLORS.surface_tint().get_argb(value),
      primary: MATERIAL_DYNAMIC_COLORS.primary().get_argb(value),
      primary_dim: MATERIAL_DYNAMIC_COLORS.primary_dim().get_argb(value),
      on_primary: MATERIAL_DYNAMIC_COLORS.on_primary().get_argb(value),
      primary_container: MATERIAL_DYNAMIC_COLORS.primary_container().get_argb(value),
      on_primary_container: MATERIAL_DYNAMIC_COLORS
        .on_primary_container()
        .get_argb(value),
      inverse_primary: MATERIAL_DYNAMIC_COLORS.inverse_primary().get_argb(value),
      primary_fixed: MATERIAL_DYNAMIC_COLORS.primary_fixed().get_argb(value),
      primary_fixed_dim: MATERIAL_DYNAMIC_COLORS.primary_fixed_dim().get_argb(value),
      on_primary_fixed: MATERIAL_DYNAMIC_COLORS.on_primary_fixed().get_argb(value),
      on_primary_fixed_variant: MATERIAL_DYNAMIC_COLORS
        .on_primary_fixed_variant()
        .get_argb(value),
      secondary: MATERIAL_DYNAMIC_COLORS.secondary().get_argb(value),
      secondary_dim: MATERIAL_DYNAMIC_COLORS.secondary_dim().get_argb(value),
      on_secondary: MATERIAL_DYNAMIC_COLORS.on_secondary().get_argb(value),
      secondary_container: MATERIAL_DYNAMIC_COLORS
        .secondary_container()
        .get_argb(value),
      on_secondary_container: MATERIAL_DYNAMIC_COLORS
        .on_secondary_container()
        .get_argb(value),
      secondary_fixed: MATERIAL_DYNAMIC_COLORS.secondary_fixed().get_argb(value),
      secondary_fixed_dim: MATERIAL_DYNAMIC_COLORS
        .secondary_fixed_dim()
        .get_argb(value),
      on_secondary_fixed: MATERIAL_DYNAMIC_COLORS.on_secondary_fixed().get_argb(value),
      on_secondary_fixed_variant: MATERIAL_DYNAMIC_COLORS
        .on_secondary_fixed_variant()
        .get_argb(value),
      tertiary: MATERIAL_DYNAMIC_COLORS.tertiary().get_argb(value),
      tertiary_dim: MATERIAL_DYNAMIC_COLORS.tertiary_dim().get_argb(value),
      on_tertiary: MATERIAL_DYNAMIC_COLORS.on_tertiary().get_argb(value),
      tertiary_container: MATERIAL_DYNAMIC_COLORS.tertiary_container().get_argb(value),
      on_tertiary_container: MATERIAL_DYNAMIC_COLORS
        .on_tertiary_container()
        .get_argb(value),
      tertiary_fixed: MATERIAL_DYNAMIC_COLORS.tertiary_fixed().get_argb(value),
      tertiary_fixed_dim: MATERIAL_DYNAMIC_COLORS.tertiary_fixed_dim().get_argb(value),
      on_tertiary_fixed: MATERIAL_DYNAMIC_COLORS.on_tertiary_fixed().get_argb(value),
      on_tertiary_fixed_variant: MATERIAL_DYNAMIC_COLORS
        .on_tertiary_fixed_variant()
        .get_argb(value),
      error: MATERIAL_DYNAMIC_COLORS.error().get_argb(value),
      error_dim: MATERIAL_DYNAMIC_COLORS.error_dim().get_argb(value),
      on_error: MATERIAL_DYNAMIC_COLORS.on_error().get_argb(value),
      error_container: MATERIAL_DYNAMIC_COLORS.error_container().get_argb(value),
      on_error_container: MATERIAL_DYNAMIC_COLORS.on_error_container().get_argb(value),
    }
  }
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FigmaPalettes {
  #[serde(skip_serializing_if = "Option::is_none")]
  primary: Option<FigmaPalette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  secondary: Option<FigmaPalette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  tertiary: Option<FigmaPalette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  neutral: Option<FigmaPalette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  neutral_variant: Option<FigmaPalette>,
  #[serde(skip_serializing_if = "Option::is_none")]
  error: Option<FigmaPalette>,
}

impl From<&DynamicScheme> for FigmaPalettes {
  fn from(value: &DynamicScheme) -> Self {
    Self {
      primary: Some(value.primary_palette().into()),
      secondary: Some(value.secondary_palette().into()),
      tertiary: Some(value.tertiary_palette().into()),
      neutral: Some(value.neutral_palette().into()),
      neutral_variant: Some(value.neutral_variant_palette().into()),
      error: Some(value.error_palette().into()),
    }
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaPalette {
  #[serde(rename = "0", serialize_with = "serialize_argb_to_hex")]
  t0: u32,
  #[serde(rename = "5", serialize_with = "serialize_argb_to_hex")]
  t5: u32,
  #[serde(rename = "10", serialize_with = "serialize_argb_to_hex")]
  t10: u32,
  #[serde(rename = "15", serialize_with = "serialize_argb_to_hex")]
  t15: u32,
  #[serde(rename = "20", serialize_with = "serialize_argb_to_hex")]
  t20: u32,
  #[serde(rename = "25", serialize_with = "serialize_argb_to_hex")]
  t25: u32,
  #[serde(rename = "30", serialize_with = "serialize_argb_to_hex")]
  t30: u32,
  #[serde(rename = "35", serialize_with = "serialize_argb_to_hex")]
  t35: u32,
  #[serde(rename = "40", serialize_with = "serialize_argb_to_hex")]
  t40: u32,
  #[serde(rename = "50", serialize_with = "serialize_argb_to_hex")]
  t50: u32,
  #[serde(rename = "60", serialize_with = "serialize_argb_to_hex")]
  t60: u32,
  #[serde(rename = "70", serialize_with = "serialize_argb_to_hex")]
  t70: u32,
  #[serde(rename = "80", serialize_with = "serialize_argb_to_hex")]
  t80: u32,
  #[serde(rename = "90", serialize_with = "serialize_argb_to_hex")]
  t90: u32,
  #[serde(rename = "95", serialize_with = "serialize_argb_to_hex")]
  t95: u32,
  #[serde(rename = "98", serialize_with = "serialize_argb_to_hex")]
  t98: u32,
  #[serde(rename = "99", serialize_with = "serialize_argb_to_hex")]
  t99: u32,
  #[serde(rename = "100", serialize_with = "serialize_argb_to_hex")]
  t100: u32,
}

impl From<&TonalPalette> for FigmaPalette {
  fn from(value: &TonalPalette) -> Self {
    Self {
      t0: value.tone(0),
      t5: value.tone(5),
      t10: value.tone(10),
      t15: value.tone(15),
      t20: value.tone(20),
      t25: value.tone(25),
      t30: value.tone(30),
      t35: value.tone(35),
      t40: value.tone(40),
      t50: value.tone(50),
      t60: value.tone(60),
      t70: value.tone(70),
      t80: value.tone(80),
      t90: value.tone(90),
      t95: value.tone(95),
      t98: value.tone(98),
      t99: value.tone(99),
      t100: value.tone(100),
    }
  }
}
