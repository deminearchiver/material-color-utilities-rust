mod cli;
mod figma_legacy;
mod formats;
mod prompt;

use clap::Parser;
use console::{Term, style};
use csscolorparser::{Color, ParseColorError};
use dialoguer::{
  Input, MultiSelect, Select,
  theme::{ColorfulTheme, Theme},
};
use human_panic::setup_panic;
use material_color_utilities::{
  dynamiccolor::{DynamicSchemeBuilder, Platform, SpecVersion, Variant},
  utils::{color, string::TryParseArgb},
};

use crate::{
  cli::Cli,
  prompt::{
    InputPrompt, Prompt, PromptAction, PromptColor, PromptError, PromptTemplate, PromptVariant,
  },
};

pub trait Template {
  fn source_color(&self) -> Option<u32> {
    Some(0xff6750a4)
  }

  fn variant(&self) -> Option<Variant> {
    Some(Variant::TonalSpot)
  }

  fn platform(&self) -> Option<Platform> {
    Some(Platform::Phone)
  }

  fn spec_version(&self) -> Option<SpecVersion> {
    Some(SpecVersion::Spec2021)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmptyTemplate {}

impl Template for EmptyTemplate {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}

pub fn prompt_template(theme: &dyn Theme) -> Box<dyn Template> {
  let templates: Vec<_> = [None]
    .into_iter()
    .chain(PromptTemplate::VALUES.into_iter().map(Some))
    .collect();
  let template_labels: Vec<_> = templates
    .iter()
    .map(|value| {
      value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "None".into())
    })
    .collect();
  let default_template = Some(PromptTemplate::Baseline);
  let default_template_index = templates
    .iter()
    .position(|value| value == &default_template)
    .unwrap();
  let template_index = Select::with_theme(theme)
    .with_prompt("Pick a template")
    .default(default_template_index)
    .items(&template_labels)
    .interact()
    .unwrap();
  templates[template_index]
    .map(|template| Box::new(DefaultTemplate::from(template)) as Box<dyn Template>)
    .unwrap_or_else(|| Box::new(EmptyTemplate {}) as Box<dyn Template>)
}

fn main() {
  setup_panic!();

  let cli = Cli::parse();

  let theme = ColorfulTheme::default();

  // let term = Term::stdout();
  // term.write_line("Material Theme Builder CLI").unwrap();

  // println!(
  //   "{}",
  //   style("Material Theme Builder CLI").on_bright().on_green()
  // );

  // loop {
  //   let options = vec![
  //     PromptAction::SourceColor,
  //     PromptAction::CoreColors,
  //     PromptAction::Variant,
  //     PromptAction::Platform,
  //     PromptAction::SpecVersion,
  //     PromptAction::Export,
  //   ];
  //   let options = Select::with_theme(&theme)
  //     .with_prompt("Select action")
  //     .items(&options)
  //     .interact()
  //     .unwrap();
  // }

  let scheme = DynamicSchemeBuilder::new()
    .is_dark(false)
    .variant(Variant::Expressive)
    .platform(Platform::Phone)
    .contrast_level(0.0)
    .spec_version(SpecVersion::Spec2025)
    .build()
    .unwrap();

  // let template = prompt_template(&theme);

  // let source_color = Input::<String>::with_theme(&theme)
  //   .with_prompt("Source color")
  //   .interact()
  //   .unwrap()
  //   .parse::<Color>()
  //   .unwrap()
  //   .try_parse_argb()
  //   .unwrap();
  // // let primary_palette_key_color = InputPrompt::<Color>::new(
  // //   |value: &str| value.parse::<Color>(),
  // //   |value: &Color| value.to_css_hex(),
  // // )
  // // .with_theme(&theme)
  // // .interact()
  // // .unwrap();
  // let core_colors: Vec<_> = PromptCoreColor::VALUES.into_iter().collect();
  // MultiSelect::with_theme(&theme)
  //   .with_prompt("Override core colors?")
  //   .items(&core_colors)
  //   .interact()
  //   .unwrap();

  // // let a: String = Input::with_theme(&theme)
  // //   .with_prompt("Source color")
  // //   .validate_with(|value: &String| -> Result<(), PromptError> {
  // //     value
  // //       .parse::<Color>()
  // //       .map(|_| ())
  // //       .map_err(|_| PromptError::InvalidCssColor)
  // //   })
  // //   .interact()
  // //   .unwrap();

  // let variants: Vec<_> = PromptVariant::VALUES.into_iter().collect();
  // let default_variant = variants
  //   .iter()
  //   .position(|value| value == &PromptVariant::TonalSpot)
  //   .unwrap();

  // let variant = Select::with_theme(&theme)
  //   .with_prompt("Pick a variant")
  //   .default(default_variant)
  //   .items(&variants)
  //   .interact()
  //   .unwrap();

  // println!("You chose: {}", variants[variant]);
}
