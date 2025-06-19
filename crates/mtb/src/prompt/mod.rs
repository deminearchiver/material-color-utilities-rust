use std::{
  fmt::{self, Display, Formatter},
  marker::PhantomData,
  str::FromStr,
  sync::{Arc, Mutex},
};

use console::Term;
use csscolorparser::ParseColorError;
use dialoguer::{Input, InputValidator, theme::Theme};
use material_color_utilities::dynamiccolor::{Platform, SpecVersion, Variant};
use thiserror::Error;

use crate::DefaultTemplate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PromptTemplate {
  Baseline,
  NowInAndroid,
}

impl PromptTemplate {
  pub const VALUES: [PromptTemplate; 2] = [PromptTemplate::Baseline, PromptTemplate::NowInAndroid];
}

impl Display for PromptTemplate {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      PromptTemplate::Baseline => write!(f, "Baseline"),
      PromptTemplate::NowInAndroid => write!(f, "Now in Android"),
    }
  }
}

impl From<PromptTemplate> for DefaultTemplate {
  fn from(value: PromptTemplate) -> Self {
    match value {
      PromptTemplate::Baseline => Self::Baseline,
      PromptTemplate::NowInAndroid => Self::NowInAndroid,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PromptVariant {
  Monochrome,
  Neutral,
  TonalSpot,
  Vibrant,
  Expressive,
  Fidelity,
  Content,
  Rainbow,
  FruitSalad,
}

impl PromptVariant {
  pub const VALUES: [PromptVariant; 9] = [
    PromptVariant::Monochrome,
    PromptVariant::Neutral,
    PromptVariant::TonalSpot,
    PromptVariant::Vibrant,
    PromptVariant::Expressive,
    PromptVariant::Fidelity,
    PromptVariant::Content,
    PromptVariant::Rainbow,
    PromptVariant::FruitSalad,
  ];
}

impl Display for PromptVariant {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      PromptVariant::Monochrome => write!(f, "Monochrome"),
      PromptVariant::Neutral => write!(f, "Neutral"),
      PromptVariant::TonalSpot => write!(f, "Tonal spot"),
      PromptVariant::Vibrant => write!(f, "Vibrant"),
      PromptVariant::Expressive => write!(f, "Expressive"),
      PromptVariant::Fidelity => write!(f, "Fidelity"),
      PromptVariant::Content => write!(f, "Content"),
      PromptVariant::Rainbow => write!(f, "Rainbow"),
      PromptVariant::FruitSalad => write!(f, "Fruit salad"),
    }
  }
}

impl From<PromptVariant> for Variant {
  fn from(value: PromptVariant) -> Self {
    match value {
      PromptVariant::Monochrome => Self::Monochrome,
      PromptVariant::Neutral => Self::Neutral,
      PromptVariant::TonalSpot => Self::TonalSpot,
      PromptVariant::Vibrant => Self::Vibrant,
      PromptVariant::Expressive => Self::Expressive,
      PromptVariant::Fidelity => Self::Fidelity,
      PromptVariant::Content => Self::Content,
      PromptVariant::Rainbow => Self::Rainbow,
      PromptVariant::FruitSalad => Self::FruitSalad,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PromptPlatform {
  Phone,
  Watch,
}

impl PromptPlatform {
  pub const VALUES: [PromptPlatform; 2] = [PromptPlatform::Phone, PromptPlatform::Watch];
}

impl Display for PromptPlatform {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      PromptPlatform::Phone => write!(f, "Phone"),
      PromptPlatform::Watch => write!(f, "Watch"),
    }
  }
}

impl From<PromptPlatform> for Platform {
  fn from(value: PromptPlatform) -> Self {
    match value {
      PromptPlatform::Phone => Self::Phone,
      PromptPlatform::Watch => Self::Watch,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PromptSpec {
  Spec2021,
  Spec2025,
}

impl PromptSpec {
  pub const VALUES: [PromptSpec; 2] = [PromptSpec::Spec2021, PromptSpec::Spec2025];
}

impl Display for PromptSpec {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      PromptSpec::Spec2021 => write!(f, "2021"),
      PromptSpec::Spec2025 => write!(f, "2025"),
    }
  }
}

impl From<PromptSpec> for SpecVersion {
  fn from(value: PromptSpec) -> Self {
    match value {
      PromptSpec::Spec2021 => Self::Spec2021,
      PromptSpec::Spec2025 => Self::Spec2025,
    }
  }
}

#[derive(Debug, Error)]
pub enum PromptError {
  #[error("{0}")]
  CssColorParser(#[from] ParseColorError),
  #[error("{0}")]
  Dialoguer(#[from] dialoguer::Error),
}

pub struct InputPrompt<'a, T> {
  theme: Option<&'a dyn Theme>,
  parser: Arc<Mutex<dyn FnMut(&str) -> Result<T, String> + 'a>>,
  formatter: Arc<Mutex<dyn FnMut(&T) -> String + 'a>>,
  // validator: Option<Arc<Mutex<dyn FnMut(&T) -> Option<String> + 'a>>>,
  phantom: PhantomData<&'a T>,
}

impl<'a, T> InputPrompt<'a, T> {
  pub fn new(
    mut parser: impl InputParser<T, Error = impl ToString> + 'a,
    mut formatter: impl InputFormatter<T> + 'a,
  ) -> Self {
    Self {
      theme: None,
      parser: Arc::new(Mutex::new(move |value: &str| {
        parser.parse(value).map_err(|error| error.to_string())
      })),
      formatter: Arc::new(Mutex::new(move |value: &T| formatter.format(value))),
      phantom: PhantomData,
    }
  }

  pub fn with_theme(mut self, theme: &'a dyn Theme) -> Self {
    self.theme = Some(theme);
    self
  }

  pub fn build(&mut self) -> Input<'_, String> {
    let input = match self.theme {
      Some(theme) => Input::with_theme(theme),
      None => Input::new(),
    };
    input
      .validate_with(move |value: &String| -> Result<(), String> { self.parse(value).map(|_| ()) })
  }
}

impl<'a, T> InputParser<T> for InputPrompt<'a, T> {
  type Error = String;

  fn parse(&mut self, value: &str) -> Result<T, Self::Error> {
    self.parser.lock().unwrap()(value)
  }
}

impl<'a, T> InputFormatter<T> for InputPrompt<'a, T> {
  fn format(&mut self, value: &T) -> String {
    self.formatter.lock().unwrap()(value)
  }
}

impl<'a, T> Prompt<T> for InputPrompt<'a, T> {
  fn interact(&mut self) -> dialoguer::Result<T> {
    self
      .build()
      .interact()
      .map(|value| self.parse(&value).unwrap())
  }

  fn interact_on(&mut self, term: &Term) -> dialoguer::Result<T> {
    self
      .build()
      .interact_on(term)
      .map(|value| self.parse(&value).unwrap())
  }
}

pub trait Prompt<T> {
  fn interact(&mut self) -> dialoguer::Result<T>;
  fn interact_on(&mut self, term: &Term) -> dialoguer::Result<T>;
}

pub trait InputParser<T> {
  type Error;

  fn parse(&mut self, value: &str) -> Result<T, Self::Error>;
}

impl<T, F, E> InputParser<T> for F
where
  F: FnMut(&str) -> Result<T, E>,
{
  type Error = E;

  fn parse(&mut self, value: &str) -> Result<T, Self::Error> {
    self(value)
  }
}

pub trait InputFormatter<T> {
  fn format(&mut self, value: &T) -> String;
}

impl<T, F> InputFormatter<T> for F
where
  F: FnMut(&T) -> String,
{
  fn format(&mut self, value: &T) -> String {
    self(value)
  }
}
