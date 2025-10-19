use std::{
  fmt::Error,
  fs::{self, File},
  io,
  path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use csscolorparser::Color;
use material_color_utilities::{
  dynamiccolor::{
    DynamicScheme, DynamicSchemeBuilder, MaterialDynamicColors, Platform, SpecVersion, Variant,
  },
  hct::Hct,
  utils::{
    color::is_opaque,
    string::{FromArgb, css_hex_from_argb},
  },
};
// use std::fmt::Write as FmtWrite;
use std::io::Write;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetSpec {
  #[default]
  Phone2021,
  Phone2025,
  Watch2025,
}

impl PresetSpec {
  pub fn default_from_spec_version(spec_version: SpecVersion) -> Self {
    match spec_version {
      SpecVersion::Spec2021 => Self::Phone2021,
      SpecVersion::Spec2025 => Self::Phone2025,
    }
  }
  pub fn from_platform_and_spec_version(platform: Platform, spec_version: SpecVersion) -> Self {
    match (platform, spec_version) {
      (Platform::Phone, SpecVersion::Spec2025) => Self::Phone2025,
      (Platform::Watch, _) => Self::Watch2025,
      (_, SpecVersion::Spec2021) => Self::Phone2021,
    }
  }
  pub fn latest_from_platform(platform: Platform) -> Self {
    match platform {
      Platform::Phone => Self::Phone2025,
      Platform::Watch => Self::Watch2025,
    }
  }

  // Getters

  pub fn platform(&self) -> Platform {
    match *self {
      Self::Phone2021 | Self::Phone2025 => Platform::Phone,
      Self::Watch2025 => Platform::Watch,
    }
  }

  pub fn spec_version(&self) -> SpecVersion {
    match *self {
      Self::Phone2021 => SpecVersion::Spec2021,
      Self::Phone2025 | Self::Watch2025 => SpecVersion::Spec2025,
    }
  }

  // Other

  pub fn is_dark(&self, is_dark: bool) -> bool {
    match *self {
      Self::Watch2025 => true,
      _ => is_dark,
    }
  }

  pub fn variant(&self, variant: Variant) -> Variant {
    variant
  }

  pub fn contrast_level(&self, contrast_level: f64) -> f64 {
    match *self {
      PresetSpec::Phone2021 => contrast_level.clamp(-1.0, 1.0),
      PresetSpec::Phone2025 | PresetSpec::Watch2025 => contrast_level.clamp(0.0, 1.0),
    }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresetColors {
  source_color: u32,
  primary_palette_key_color: Option<u32>,
  secondary_palette_key_color: Option<u32>,
  tertiary_palette_key_color: Option<u32>,
  neutral_palette_key_color: Option<u32>,
  neutral_variant_palette_key_color: Option<u32>,
  error_palette_key_color: Option<u32>,
}

impl PresetColors {
  pub fn from_source_color(source_color: u32) -> Self {
    Self {
      source_color,
      ..Default::default()
    }
  }

  pub fn baseline() -> Self {
    Self {
      source_color: 0xff6750a4,
      ..Default::default()
    }
  }

  pub fn now_in_android() -> Self {
    Self {
      source_color: 0xff8c4190,
      primary_palette_key_color: Some(0xff8c4190),
      secondary_palette_key_color: Some(0xffff8456),
      tertiary_palette_key_color: Some(0xffb3e9ff),
      neutral_palette_key_color: Some(0xff201a1b),
      ..Default::default()
    }
  }
  pub fn source_color(&self) -> u32 {
    self.source_color
  }

  pub fn primary_palette_key_color(&self) -> Option<u32> {
    self.primary_palette_key_color
  }

  pub fn secondary_palette_key_color(&self) -> Option<u32> {
    self.secondary_palette_key_color
  }

  pub fn tertiary_palette_key_color(&self) -> Option<u32> {
    self.tertiary_palette_key_color
  }

  pub fn neutral_palette_key_color(&self) -> Option<u32> {
    self.neutral_palette_key_color
  }

  pub fn neutral_variant_palette_key_color(&self) -> Option<u32> {
    self.neutral_variant_palette_key_color
  }

  pub fn error_palette_key_color(&self) -> Option<u32> {
    self.error_palette_key_color
  }

  pub fn with_source_color(mut self, value: u32) -> Self {
    self.source_color = value;
    self
  }

  pub fn with_primary_palette_key_color(mut self, value: Option<u32>) -> Self {
    self.primary_palette_key_color = value;
    self
  }
  pub fn with_secondary_palette_key_color(mut self, value: Option<u32>) -> Self {
    self.secondary_palette_key_color = value;
    self
  }
  pub fn with_tertiary_palette_key_color(mut self, value: Option<u32>) -> Self {
    self.tertiary_palette_key_color = value;
    self
  }
  pub fn with_neutral_palette_key_color(mut self, value: Option<u32>) -> Self {
    self.neutral_palette_key_color = value;
    self
  }
  pub fn with_neutral_variant_palette_key_color(mut self, value: Option<u32>) -> Self {
    self.neutral_variant_palette_key_color = value;
    self
  }
  pub fn with_error_palette_key_color(mut self, value: Option<u32>) -> Self {
    self.error_palette_key_color = value;
    self
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PresetItem {
  variant: Variant,
  spec: PresetSpec,
  colors: PresetColors,
}

impl PresetItem {
  pub fn variant(&self) -> &Variant {
    &self.variant
  }

  pub fn spec(&self) -> &PresetSpec {
    &self.spec
  }

  pub fn colors(&self) -> &PresetColors {
    &self.colors
  }

  pub fn builder(&self, is_dark: bool, contrast_level: f64) -> DynamicSchemeBuilder {
    let variant = *self.variant();
    let spec = self.spec();
    let colors = self.colors();
    let mut builder = DynamicSchemeBuilder::default()
      // Spec
      .platform(spec.platform())
      .spec_version(spec.spec_version())
      // Spec-dependent
      .variant(spec.variant(variant))
      .is_dark(spec.is_dark(is_dark))
      .contrast_level(spec.contrast_level(contrast_level))
      // Colors
      .source_color_hct(self.colors().source_color().into());
    if let Some(primary_palette_key_color) = colors.primary_palette_key_color() {
      builder = builder.primary_palette_key_color(primary_palette_key_color.into());
    }
    if let Some(secondary_palette_key_color) = colors.secondary_palette_key_color() {
      builder = builder.secondary_palette_key_color(secondary_palette_key_color.into());
    }
    if let Some(tertiary_palette_key_color) = colors.tertiary_palette_key_color() {
      builder = builder.tertiary_palette_key_color(tertiary_palette_key_color.into());
    }
    if let Some(neutral_palette_key_color) = colors.neutral_palette_key_color() {
      builder = builder.neutral_palette_key_color(neutral_palette_key_color.into());
    }
    if let Some(neutral_variant_palette_key_color) = colors.neutral_variant_palette_key_color() {
      builder = builder.neutral_variant_palette_key_color(neutral_variant_palette_key_color.into());
    }
    if let Some(error_palette_key_color) = colors.error_palette_key_color() {
      builder = builder.error_palette_key_color(error_palette_key_color.into());
    }
    builder
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Preset {
  inner: Vec<PresetItem>,
}

impl Preset {
  pub fn new(inner: impl IntoIterator<Item = PresetItem>) -> Self {
    Self {
      inner: inner.into_iter().collect(),
    }
  }

  pub fn with(variants: &[Variant], specs: &[PresetSpec], colors: PresetColors) -> Self {
    let mut items: Vec<PresetItem> = Vec::with_capacity(variants.len() * specs.len());

    for &variant in variants {
      for &spec in specs {
        let item = PresetItem {
          variant,
          spec,
          colors,
        };
        items.push(item);
      }
    }
    Self { inner: items }
  }

  pub fn with_colors(colors: PresetColors) -> Self {
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
    const SPECS: [PresetSpec; 3] = [
      PresetSpec::Phone2021,
      PresetSpec::Phone2025,
      PresetSpec::Watch2025,
    ];
    Self::with(&VARIANTS, &SPECS, colors)
  }

  pub fn inner(&self) -> &Vec<PresetItem> {
    &self.inner
  }

  pub fn into_inner(self) -> Vec<PresetItem> {
    self.inner
  }
}

pub fn run() {
  let baseline = Preset::with_colors(PresetColors::baseline());
  let now_in_android = Preset::with_colors(PresetColors::now_in_android());
  let twitch = Preset::with_colors(PresetColors::from_source_color(0xff9146ff));
  let youtube = Preset::with_colors(PresetColors::from_source_color(0xffff0033));
  let discord = Preset::with_colors(PresetColors::from_source_color(0xff5865f2));
  let localsend = Preset::with_colors(PresetColors::from_source_color(0xff009688));
  let base_path = PathBuf::from("./gen/custom");
  let name_to_preset = [
    ("baseline", baseline),
    ("now_in_android", now_in_android),
    ("twitch", twitch),
    ("youtube", youtube),
    ("discord", discord),
    ("localsend", localsend),
  ];
  for (name, preset) in name_to_preset.iter() {
    save_preset(preset, base_path.join(name)).unwrap();
  }
}

fn save_preset(preset: &Preset, path: impl AsRef<Path>) -> Result<(), io::Error> {
  let dir_path = path.as_ref();
  fs::create_dir_all(dir_path)?;

  for item in preset.inner().iter() {
    let variant = *item.variant();
    let spec = item.spec();
    let platform = spec.platform();
    let spec_version = spec.spec_version();
    let colors = item.colors();

    let light = item.builder(false, 0.0).build();
    let light_medium_contrast = item.builder(false, 0.5).build();
    let light_high_contrast = item.builder(false, 1.0).build();
    let dark = item.builder(true, 0.0).build();
    let dark_medium_contrast = item.builder(true, 0.5).build();
    let dark_high_contrast = item.builder(true, 1.0).build();

    let schemes = vec![
      (light, ".light"),
      (light_medium_contrast, ".light-medium-contrast"),
      (light_high_contrast, ".light-high-contrast"),
      (dark, ".dark"),
      (dark_medium_contrast, ".dark-medium-contrast"),
      (dark_high_contrast, ".dark-high-contrast"),
    ];

    let prefix = "md-sys-color";

    let file_name = [
      variant_name(&variant),
      platform_name(&platform),
      spec_version_name(&spec_version),
    ]
    .join("_");
    let file_path = dir_path.join(file_name).with_extension("css");
    let mut file = File::create(file_path)?;
    for (scheme, selector) in schemes.iter() {
      write_css(&mut file, scheme, selector, prefix).unwrap();
      writeln!(file).unwrap();
    }
  }
  Ok(())
}

fn write_css(
  buffer: &mut dyn Write,
  scheme: &DynamicScheme,
  selector: impl AsRef<str>,
  prefix: impl AsRef<str>,
) -> Result<(), io::Error> {
  let css_selector = selector.as_ref();
  let css_prefix = prefix.as_ref().to_case(Case::Kebab);
  writeln!(buffer, "{css_selector} {{")?;
  let dynamic_colors = MaterialDynamicColors.all_dynamic_colors();
  for dynamic_color in dynamic_colors.into_iter() {
    let argb = dynamic_color.get_argb(scheme);
    let css_color = Color::from_argb(argb);
    let css_hex = css_color.to_css_hex();
    let css_value = css_hex;
    let css_name = dynamic_color.name().to_case(Case::Kebab);
    let css_property = format!("--{css_prefix}-{css_name}");
    let css_rule = format!("{css_property}: {css_value};");
    writeln!(buffer, "  {css_rule}")?;
  }
  writeln!(buffer, "}}")?;
  Ok(())
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

// fn create_css_selector(selector: impl AsRef<str>, indent_size: usize, child: String) {}

// fn create_css_properties(
//   scheme: &DynamicScheme,
//   prefix: impl AsRef<str>,
//   indent_size: usize,
// ) -> Result<String, Error> {
//   let css_prefix = prefix.as_ref().to_case(Case::Kebab);
//   let mut result = String::new();
//   let dynamic_colors = MaterialDynamicColors.all_dynamic_colors();
//   for dynamic_color in dynamic_colors.into_iter() {
//     let argb = dynamic_color.get_argb(scheme);
//     let css_color = Color::from_argb(argb);
//     let css_hex = css_color.to_css_hex();
//     let css_value = css_hex;
//     let css_name = dynamic_color.name().to_case(Case::Kebab);
//     let css_property = format!("--{css_prefix}-{css_name}");
//     let css_rule = format!("{css_property}: {css_value};");
//     writeln!(result, "{css_rule}")?;
//   }
//   Ok(result)
// }
