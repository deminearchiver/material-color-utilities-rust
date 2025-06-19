mod utils;
use convert_case::Case;
use csscolorparser::Color;
use std::{
  fmt::{self, Display, Formatter},
  iter::{self, once},
  str::FromStr,
};

use clap::{Parser, Subcommand, arg};
use inquire::{CustomType, Select, ui::RenderConfig};
use material_color_utilities::{
  dynamiccolor::{Platform, SpecVersion, Variant},
  utils::color::{
    alpha_from_argb, argb_from_linrgb, blue_from_argb, green_from_argb, red_from_argb,
  },
};

use crate::utils::{Custom, Format, IteratorExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  /// Optional name to operate on
  name: Option<String>,

  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// does testing things
  Test {
    /// lists test values
    #[arg(short, long)]
    list: bool,
  },
}

fn format_variant(variant: &Variant, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  match *variant {
    Variant::Monochrome => write!(f, "Monochrome"),
    Variant::Neutral => write!(f, "Neutral"),
    Variant::TonalSpot => write!(f, "Tonal spot"),
    Variant::Vibrant => write!(f, "Vibrant"),
    Variant::Expressive => write!(f, "Expressive"),
    Variant::Fidelity => write!(f, "Fidelity"),
    Variant::Content => write!(f, "Content"),
    Variant::Rainbow => write!(f, "Rainbow"),
    Variant::FruitSalad => write!(f, "Fruit salad"),
  }
}
fn format_spec_version(spec_version: &SpecVersion, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  match *spec_version {
    SpecVersion::Spec2021 => write!(f, "2021"),
    SpecVersion::Spec2025 => write!(f, "2025"),
  }
}

fn format_platform(platform: &Platform, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  match *platform {
    Platform::Phone => write!(f, "Phone"),
    Platform::Watch => write!(f, "Watch"),
  }
}

fn format_platform_and_spec_version(
  value: &PlatformAndSpecVersion,
  f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
  match *value {
    PlatformAndSpecVersion::Phone2021 => write!(f, "Phone (2021)"),
    PlatformAndSpecVersion::Phone2025 => write!(f, "Phone (2025)"),
    PlatformAndSpecVersion::Watch2025 => write!(f, "Watch (2025)"),
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
const SPEC_VERSIONS: [SpecVersion; 2] = [SpecVersion::Spec2021, SpecVersion::Spec2025];
const PLATFORMS: [Platform; 2] = [Platform::Phone, Platform::Watch];
const PLATFORMS_AND_SPEC_VERSIONS: [PlatformAndSpecVersion; 3] = [
  PlatformAndSpecVersion::Phone2021,
  PlatformAndSpecVersion::Phone2025,
  PlatformAndSpecVersion::Watch2025,
];

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Source {
  Color,
  Image,
}

pub struct State {
  source_color: u32,
}

pub trait Template {
  fn source_color(&self) -> Option<u32> {
    None
  }
  fn primary_palette_key_color(&self) -> Option<u32> {
    None
  }
  fn secondary_palette_key_color(&self) -> Option<u32> {
    None
  }
  fn tertiary_palette_key_color(&self) -> Option<u32> {
    None
  }
  fn neutral_palette_key_color(&self) -> Option<u32> {
    None
  }
  fn neutral_variant_palette_key_color(&self) -> Option<u32> {
    None
  }
  fn error_palette_key_color(&self) -> Option<u32> {
    None
  }
}

const DEFAULT_TEMPLATES: [DefaultTemplate; 2] =
  [DefaultTemplate::Baseline, DefaultTemplate::NowInAndroid];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefaultTemplate {
  Baseline,
  NowInAndroid,
}

impl Template for DefaultTemplate {
  fn source_color(&self) -> Option<u32> {
    match *self {
      DefaultTemplate::Baseline => Some(0xff6750a4),
      DefaultTemplate::NowInAndroid => Some(0xff8c4190),
    }
  }

  fn primary_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xff8c4190),
      _ => None,
    }
  }

  fn secondary_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xffff8456),
      _ => None,
    }
  }

  fn tertiary_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xffb3e9ff),
      _ => None,
    }
  }

  fn neutral_palette_key_color(&self) -> Option<u32> {
    match *self {
      Self::NowInAndroid => Some(0xff201a1b),
      _ => None,
    }
  }

  fn neutral_variant_palette_key_color(&self) -> Option<u32> {
    None
  }

  fn error_palette_key_color(&self) -> Option<u32> {
    None
  }
}

impl<T: Template> Template for Option<T> {
  fn source_color(&self) -> Option<u32> {
    self.as_ref().and_then(|template| template.source_color())
  }
  fn primary_palette_key_color(&self) -> Option<u32> {
    self
      .as_ref()
      .and_then(|template| template.primary_palette_key_color())
  }
  fn secondary_palette_key_color(&self) -> Option<u32> {
    self
      .as_ref()
      .and_then(|template| template.secondary_palette_key_color())
  }
  fn tertiary_palette_key_color(&self) -> Option<u32> {
    self
      .as_ref()
      .and_then(|template| template.tertiary_palette_key_color())
  }
  fn neutral_palette_key_color(&self) -> Option<u32> {
    self
      .as_ref()
      .and_then(|template| template.neutral_palette_key_color())
  }
  fn neutral_variant_palette_key_color(&self) -> Option<u32> {
    self
      .as_ref()
      .and_then(|template| template.neutral_variant_palette_key_color())
  }
  fn error_palette_key_color(&self) -> Option<u32> {
    self
      .as_ref()
      .and_then(|template| template.error_palette_key_color())
  }
}

fn format_template(value: &DefaultTemplate, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  match *value {
    DefaultTemplate::Baseline => write!(f, "Baseline"),
    DefaultTemplate::NowInAndroid => write!(f, "Now in Android"),
  }
}

fn format_template_or(value: &Option<DefaultTemplate>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  match *value {
    Some(value) => format_template(&value, f),
    None => write!(f, "None"),
  }
}

const DEFAULT_TEMPLATES_OR: [Option<DefaultTemplate>; 3] = [
  None,
  Some(DefaultTemplate::Baseline),
  Some(DefaultTemplate::NowInAndroid),
];

pub fn argb_from_css(color: Color) -> u32 {
  let [red, green, blue, alpha] = color.to_rgba8();
  ((alpha as u32) << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}

pub fn css_from_argb(argb: u32) -> Color {
  let r = red_from_argb(argb);
  let g = green_from_argb(argb);
  let b = blue_from_argb(argb);
  let a = alpha_from_argb(argb);
  Color::from_rgba8(r, g, b, a)
}

fn prompt_css<'a>(message: &'a str) -> CustomType<'a, Color> {
  CustomType {
    message,
    starting_input: None,
    default: None,
    placeholder: None,
    formatter: &&|value: Color| value.to_css_hex(),
    default_value_formatter: &&|value: Color| value.to_css_hex(),
    parser: &&|s: &str| csscolorparser::parse(s).map_err(|_| ()),
    validators: CustomType::DEFAULT_VALIDATORS,
    help_message: "Input must be a valid CSS color".into(),
    error_message: "Invalid CSS color".into(),
    render_config: RenderConfig::default(),
  }
}

fn palette_key_color<'a>(message: &'a str) -> Custom<'a, Option<Color>> {
  Custom::new_with(
    message,
    &|input| input.parse::<Color>().map(Some).map_err(|_| ()),
    &|value: Option<Color>| {
      value
        .as_ref()
        .map(Color::to_css_hex)
        .unwrap_or_else(|| "None".into())
    },
  )
  .with_help_message("Input must be a valid CSS color")
  .with_error_message("Invalid CSS color")
}

fn main() {
  let cli = Cli::parse();
  let render_config = RenderConfig::default();

  let templates: Vec<_> = DEFAULT_TEMPLATES_OR
    .into_iter()
    .into_format(&format_template_or)
    .collect();
  let default_template = templates
    .iter()
    .position(|value| value.inner().is_none())
    .unwrap();

  let template = Select::new("Choose a template:", templates)
    .with_starting_cursor(default_template)
    .prompt()
    .unwrap()
    .into_inner();

  let source_color_css: Color =
    Custom::new_with_formatters("Enter a source color", &|value: Color| value.to_css_hex())
      .with_help_message("Input must be a valid CSS color")
      .with_error_message("Invalid CSS color")
      .with_default_or(template.source_color().map(css_from_argb))
      .with_render_config(render_config)
      .prompt()
      .unwrap();

  let primary_palette_key_color_css: Option<Color> = palette_key_color("Primary")
    .with_default(template.primary_palette_key_color().map(css_from_argb))
    .prompt()
    .unwrap();
  let secondary_palette_key_color_css: Option<Color> = palette_key_color("Secondary")
    .with_default(template.secondary_palette_key_color().map(css_from_argb))
    .prompt()
    .unwrap();
  let tertiary_palette_key_color_css: Option<Color> = palette_key_color("Tertiary")
    .with_default(template.tertiary_palette_key_color().map(css_from_argb))
    .prompt()
    .unwrap();
  let neutral_palette_key_color_css: Option<Color> = palette_key_color("Neutral")
    .with_default(template.neutral_palette_key_color().map(css_from_argb))
    .prompt()
    .unwrap();
  let neutral_variant_palette_key_color_css: Option<Color> = palette_key_color("Neutral variant")
    .with_default(
      template
        .neutral_variant_palette_key_color()
        .map(css_from_argb),
    )
    .prompt()
    .unwrap();
  let error_variant_palette_key_color_css: Option<Color> = palette_key_color("Error")
    .with_default(template.error_palette_key_color().map(css_from_argb))
    .prompt()
    .unwrap();

  let variants: Vec<_> = VARIANTS.into_iter().into_format(&format_variant).collect();
  let default_variant = variants
    .iter()
    .position(|value| value.inner() == &Variant::TonalSpot)
    .unwrap();
  let variant = Select::new("Variant", variants)
    .with_starting_cursor(default_variant)
    .prompt()
    .unwrap()
    .into_inner();

  let platforms_and_spec_versions: Vec<_> = PLATFORMS_AND_SPEC_VERSIONS
    .into_iter()
    .into_format(&format_platform_and_spec_version)
    .collect();
  let default_platform_and_spec_version = platforms_and_spec_versions
    .iter()
    .position(|value| value.inner() == &PlatformAndSpecVersion::Phone2021)
    .unwrap();
  let platform_and_spec_version = Select::new("Platform", platforms_and_spec_versions)
    .with_help_message("Platform \"Watch (2021)\" is unavailable")
    .with_starting_cursor(default_platform_and_spec_version)
    .prompt()
    .unwrap()
    .into_inner();
  let platform = platform_and_spec_version.platform();
  let spec_version = platform_and_spec_version.spec_version();
}
