mod app;
mod export_figma;

use std::{
  collections::HashMap,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Template {
  pub source_color: u32,
  pub primary_palette_key_color: Option<u32>,
  pub secondary_palette_key_color: Option<u32>,
  pub tertiary_palette_key_color: Option<u32>,
  pub neutral_palette_key_color: Option<u32>,
  pub neutral_variant_palette_key_color: Option<u32>,
  pub error_palette_key_color: Option<u32>,
}

impl Template {
  pub fn new(source_color: u32) -> Self {
    Self {
      source_color,
      primary_palette_key_color: None,
      secondary_palette_key_color: None,
      tertiary_palette_key_color: None,
      neutral_palette_key_color: None,
      neutral_variant_palette_key_color: None,
      error_palette_key_color: None,
    }
  }

  pub fn baseline() -> Self {
    Self {
      source_color: 0xff6750a4,
      primary_palette_key_color: None,
      secondary_palette_key_color: None,
      tertiary_palette_key_color: None,
      neutral_palette_key_color: None,
      neutral_variant_palette_key_color: None,
      error_palette_key_color: None,
    }
  }

  pub fn now_in_android() -> Self {
    Self {
      source_color: 0xff8c4190,
      primary_palette_key_color: Some(0xff8c4190),
      secondary_palette_key_color: Some(0xffff8456),
      tertiary_palette_key_color: Some(0xffb3e9ff),
      neutral_palette_key_color: Some(0xff201a1b),
      neutral_variant_palette_key_color: None,
      error_palette_key_color: None,
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

fn main() {
  app::run();
  // let templates: HashMap<String, Template> = HashMap::from([
  //   ("baseline".into(), Template::baseline()),
  //   ("now_in_android".into(), Template::now_in_android()),
  //   ("twitch".into(), Template::new(0x9146ff)),
  // ]);
  // let total = templates.len() * VARIANTS.len() * PLATFORMS_AND_SPEC_VERSIONS.len();
  // let bar = ProgressBar::new(total as u64);
  // for (template_name, template) in templates.into_iter() {
  //   for variant in VARIANTS {
  //     let vname = variant_name(&variant);
  //     for platform_and_spec_version in PLATFORMS_AND_SPEC_VERSIONS {
  //       let platform = platform_and_spec_version.platform();
  //       let spec_version = platform_and_spec_version.spec_version();

  //       let mut builder = DynamicSchemeBuilder::default()
  //         .source_color_hct(template.source_color().into())
  //         .variant(variant)
  //         .platform(platform)
  //         .spec_version(spec_version);

  //       if let Some(argb) = template.primary_palette_key_color() {
  //         builder = builder.primary_palette_key_color(argb.into());
  //       }
  //       if let Some(argb) = template.secondary_palette_key_color() {
  //         builder = builder.secondary_palette_key_color(argb.into());
  //       }
  //       if let Some(argb) = template.tertiary_palette_key_color() {
  //         builder = builder.tertiary_palette_key_color(argb.into());
  //       }
  //       if let Some(argb) = template.neutral_palette_key_color() {
  //         builder = builder.neutral_palette_key_color(argb.into());
  //       }
  //       if let Some(argb) = template.neutral_variant_palette_key_color() {
  //         builder = builder.neutral_variant_palette_key_color(argb.into());
  //       }
  //       if let Some(argb) = template.error_palette_key_color() {
  //         builder = builder.error_palette_key_color(argb.into());
  //       }
  //       let figma_core_colors = FigmaCoreColors {
  //         primary: template
  //           .primary_palette_key_color()
  //           .unwrap_or_else(|| template.source_color())
  //           .into(),
  //         secondary: template.secondary_palette_key_color(),
  //         tertiary: template.tertiary_palette_key_color(),
  //         neutral: template.neutral_palette_key_color(),
  //         neutral_variant: template.neutral_variant_palette_key_color(),
  //         error: template.error_palette_key_color(),
  //       };
  //       let figma_schemes = FigmaSchemes::from(&builder);
  //       let figma = Figma {
  //         description: "TYPE: CUSTOM\nMaterial Theme Builder export".into(),
  //         seed: template.source_color(),
  //         core_colors: figma_core_colors,
  //         extended_colors: vec![],
  //         schemes: figma_schemes,
  //         palettes: FigmaPalettes::default(),
  //       };
  //       let figma_json = serde_json::to_string_pretty(&figma).unwrap();

  //       let file_name = [
  //         vname.clone(),
  //         platform_name(&platform),
  //         spec_version_name(&spec_version),
  //       ]
  //       .join("_");

  //       let dir_path = PathBuf::from("./gen/figma");
  //       fs::create_dir_all(&dir_path).unwrap();
  //       let file_path = dir_path.join(file_name).with_extension("json");
  //       let mut file = File::create(&file_path).unwrap();
  //       write!(file, "{}", figma_json.as_str()).unwrap();
  //       bar.inc(1);
  //     }
  //   }
  // }
  // bar.finish();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynamicSchemes {
  light: DynamicScheme,
  light_medium_contrast: DynamicScheme,
  light_high_contrast: DynamicScheme,
  dark: DynamicScheme,
  dark_medium_contrast: DynamicScheme,
  dark_high_contrast: DynamicScheme,
}

impl DynamicSchemes {
  pub fn light(&self) -> &DynamicScheme {
    &self.light
  }
  pub fn light_medium_contrast(&self) -> &DynamicScheme {
    &self.light_medium_contrast
  }
  pub fn light_high_contrast(&self) -> &DynamicScheme {
    &self.light_high_contrast
  }
  pub fn dark(&self) -> &DynamicScheme {
    &self.dark
  }
  pub fn dark_medium_contrast(&self) -> &DynamicScheme {
    &self.dark_medium_contrast
  }
  pub fn dark_high_contrast(&self) -> &DynamicScheme {
    &self.dark_high_contrast
  }
}

impl From<&DynamicSchemeBuilder> for DynamicSchemes {
  fn from(value: &DynamicSchemeBuilder) -> Self {
    Self {
      light: value.clone().is_dark(false).contrast_level(0.0).build(),
      light_medium_contrast: value.clone().is_dark(false).contrast_level(0.5).build(),
      light_high_contrast: value.clone().is_dark(false).contrast_level(1.0).build(),
      dark: value.clone().is_dark(true).contrast_level(0.0).build(),
      dark_medium_contrast: value.clone().is_dark(true).contrast_level(0.5).build(),
      dark_high_contrast: value.clone().is_dark(true).contrast_level(1.0).build(),
    }
  }
}

impl From<DynamicScheme> for DynamicSchemes {
  fn from(value: DynamicScheme) -> Self {
    Self::from(&DynamicSchemeBuilder::from(value))
  }
}
