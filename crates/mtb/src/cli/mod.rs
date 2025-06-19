use clap::{Parser, Subcommand, ValueEnum, arg};
use console;
use dialoguer::{self, Select, Sort, theme::ColorfulTheme};
use indicatif;
use material_color_utilities::dynamiccolor::{Platform, SpecVersion, Variant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
enum CliTemplate {
  /// Standard color scheme in Material You.
  /// Aliases: 'b', 'base'.
  #[value(name = "baseline", aliases = ["b", "base"])]
  Baseline,
  /// Resembles the color scheme of the "Now in Android" case study
  /// by the Material 3 Design team.
  /// Aliases: 'nia'.
  #[value(name = "now-in-android", aliases = ["nia"])]
  NowInAndroid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
enum CliVariant {
  /// Monochrome.
  /// Aliases: 'm', 'mono'.
  #[value(name = "monochrome", aliases = ["m", "mono"])]
  Monochrome,
  /// Neutral.
  /// Aliases: 'n'.
  #[value(name = "neutral", aliases = ["n"])]
  Neutral,
  /// Tonal spot.
  /// Aliases: 'ts', 'tonal', 'spot'.
  #[value(name = "tonal-spot", aliases = ["ts", "tonal", "spot"])]
  TonalSpot,
  /// Vibrant.
  /// Aliases: 'v'.
  #[value(name = "vibrant", aliases = ["v"])]
  Vibrant,
  /// Expressive.
  /// Aliases: 'e', 'expr'.
  #[value(name = "expressive", aliases = ["e", "expr"])]
  Expressive,
  /// Fidelity.
  /// Aliases: 'f'.
  #[value(name = "fidelity", aliases = ["f"])]
  Fidelity,
  /// Content.
  /// Aliases: 'c'.
  #[value(name = "content", aliases = ["c"])]
  Content,
  /// Rainbow.
  /// Aliases: 'r'.
  #[value(name = "rainbow", aliases = ["r"])]
  Rainbow,
  /// Fruit salad.
  /// Aliases: 'fs', 'fruit', 'salad'.
  #[value(name = "fruit-salad", aliases = ["fs", "fruit", "salad"])]
  FruitSalad,
}

impl From<CliVariant> for Variant {
  fn from(value: CliVariant) -> Self {
    match value {
      CliVariant::Monochrome => Self::Monochrome,
      CliVariant::Neutral => Self::Neutral,
      CliVariant::TonalSpot => Self::TonalSpot,
      CliVariant::Vibrant => Self::Vibrant,
      CliVariant::Expressive => Self::Expressive,
      CliVariant::Fidelity => Self::Fidelity,
      CliVariant::Content => Self::Content,
      CliVariant::Rainbow => Self::Rainbow,
      CliVariant::FruitSalad => Self::FruitSalad,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
enum CliPlatform {
  /// Default platform used before the introduction of other platforms.
  /// Aliases: 'p'.
  #[value(name = "phone", aliases = ["p"])]
  Phone,
  /// Platform which makes the scheme acquire AMOLED-like characteristics.
  /// Has no effect if spec is lower than 2025.
  /// Aliases: 'w'.
  #[value(name = "watch", aliases = ["w"])]
  Watch,
}

impl From<CliPlatform> for Platform {
  fn from(value: CliPlatform) -> Self {
    match value {
      CliPlatform::Phone => Self::Phone,
      CliPlatform::Watch => Self::Watch,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
enum CliSpec {
  /// Palettes, colors generation and color roles spec
  /// introduced in 2021 with Material 3 Baseline.
  /// Aliases: '21'.
  #[value(name = "2021", aliases = ["21"])]
  Spec2021,
  /// Palettes, colors generation and color roles spec
  /// introduced in 2025 with Material 3 Expressive.
  /// Aliases: '25'.
  #[value(name = "2025", aliases = ["25"])]
  Spec2025,
}

impl From<CliSpec> for SpecVersion {
  fn from(value: CliSpec) -> Self {
    match value {
      CliSpec::Spec2021 => Self::Spec2021,
      CliSpec::Spec2025 => Self::Spec2025,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
enum CliFormat {
  FigmaLegacy,
}

#[derive(Debug, Parser)]
#[command(name = "Material Theme Builder")]
#[command(version)]
#[command(about = "Does awesome things", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
  /// Provide an optional template to use
  #[arg(short, long, value_enum)]
  template: Option<CliTemplate>,

  /// Provide an optional variant to use
  #[arg(short, long, value_enum)]
  variant: Option<CliVariant>,

  /// Provide an optional variant to use
  #[arg(short, long, value_enum)]
  platform: Option<CliPlatform>,

  /// Provide an optional spec version to use
  #[arg(short, long, value_enum)]
  spec: Option<CliSpec>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  // /// Does testing things
  // Test {
  //   /// Lists test values
  //   #[arg(short, long)]
  //   list: bool,
  // },
}
