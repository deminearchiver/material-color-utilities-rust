use std::{
  fmt::{self},
  iter::Map,
  str::FromStr,
};

use inquire::{
  CustomType, error::InquireResult, formatter::CustomTypeFormatter, parser::CustomTypeParser,
  ui::RenderConfig, validator::CustomTypeValidator,
};

pub struct Format<'a, T> {
  inner: T,
  fmt: &'a dyn Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
}

impl<'a, T> Format<'a, T> {
  pub fn new(inner: T, fmt: &'a dyn Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result) -> Self {
    Self { inner, fmt }
  }

  pub fn inner(&self) -> &T {
    &self.inner
  }

  pub fn into_inner(self) -> T {
    self.inner
  }
}

impl<T> fmt::Display for Format<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    (*self.fmt)(&self.inner, f)
  }
}

pub trait IteratorExt<'a, T>: Sized {
  fn into_format(
    self,
    fmt: &'a dyn Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
  ) -> impl Iterator<Item = Format<'a, T>>;
}

impl<'a, T, I> IteratorExt<'a, T> for I
where
  I: Iterator<Item = T>,
{
  fn into_format(
    self,
    fmt: &'a dyn Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
  ) -> impl Iterator<Item = Format<'a, T>> {
    self.map(move |value| Format::new(value, fmt))
  }
}

#[derive(Clone)]
pub struct Custom<'a, T> {
  message: &'a str,
  starting_input: Option<&'a str>,
  default: Option<T>,
  placeholder: Option<&'a str>,
  help_message: Option<&'a str>,
  formatter: CustomTypeFormatter<'a, T>,
  default_value_formatter: CustomTypeFormatter<'a, T>,
  parser: CustomTypeParser<'a, T>,
  validators: Vec<Box<dyn CustomTypeValidator<T>>>,
  error_message: String,
  render_config: RenderConfig<'a>,
}

impl<'a, T> Custom<'a, T> {
  pub const DEFAULT_VALIDATORS: Vec<Box<dyn CustomTypeValidator<T>>> = vec![];

  pub fn new_with(
    message: &'a str,
    parser: CustomTypeParser<'a, T>,
    formatter: CustomTypeFormatter<'a, T>,
  ) -> Self {
    Self {
      message,
      starting_input: None,
      default: None,
      placeholder: None,
      help_message: None,
      formatter,
      default_value_formatter: formatter,
      parser,
      validators: Self::DEFAULT_VALIDATORS,
      error_message: "Invalid input".into(),
      render_config: RenderConfig::default(),
    }
  }

  pub fn new_with_parser(message: &'a str, parser: CustomTypeParser<'a, T>) -> Self
  where
    T: ToString,
  {
    Self::new_with(message, parser, &|val| val.to_string())
  }

  pub fn new_with_formatters(message: &'a str, formatter: CustomTypeFormatter<'a, T>) -> Self
  where
    T: FromStr,
  {
    Self::new_with(message, &|a| a.parse::<T>().map_err(|_| ()), formatter)
  }

  pub fn new(message: &'a str) -> Self
  where
    T: FromStr + ToString,
  {
    Self::new_with(message, &|a| a.parse::<T>().map_err(|_| ()), &|val| {
      val.to_string()
    })
  }

  pub fn with_message(mut self, message: &'a str) -> Self {
    self.message = message;
    self
  }

  pub fn with_starting_input(mut self, starting_input: Option<&'a str>) -> Self {
    self.starting_input = starting_input;
    self
  }

  pub fn with_default(mut self, default: T) -> Self {
    self.default = Some(default);
    self
  }
  pub fn with_default_or(mut self, default: Option<T>) -> Self {
    self.default = default;
    self
  }

  pub fn with_placeholder(mut self, placeholder: Option<&'a str>) -> Self {
    self.placeholder = placeholder;
    self
  }

  pub fn with_help_message(mut self, help_message: &'a str) -> Self {
    self.help_message = Some(help_message);
    self
  }

  pub fn with_formatter(mut self, formatter: CustomTypeFormatter<'a, T>) -> Self {
    self.formatter = formatter;
    self
  }

  pub fn with_default_value_formatter(
    mut self,
    default_value_formatter: CustomTypeFormatter<'a, T>,
  ) -> Self {
    self.default_value_formatter = default_value_formatter;
    self
  }

  pub fn with_formatters(mut self, formatter: CustomTypeFormatter<'a, T>) -> Self {
    self.formatter = formatter;
    self.default_value_formatter = formatter;
    self
  }

  pub fn with_parser(mut self, parser: CustomTypeParser<'a, T>) -> Self {
    self.parser = parser;
    self
  }

  pub fn with_validator<V>(mut self, validator: V) -> Self
  where
    V: CustomTypeValidator<T> + 'static,
  {
    self.validators.push(Box::new(validator));
    self
  }

  pub fn with_validators(mut self, validators: &[Box<dyn CustomTypeValidator<T>>]) -> Self {
    for validator in validators {
      #[allow(suspicious_double_ref_op)]
      self.validators.push(validator.clone());
    }
    self
  }

  pub fn with_error_message(mut self, error_message: &'a str) -> Self {
    self.error_message = String::from(error_message);
    self
  }

  pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
    self.render_config = render_config;
    self
  }

  pub fn prompt_skippable(self) -> InquireResult<Option<T>>
  where
    T: Clone,
  {
    CustomType::from(self).prompt_skippable()
  }

  pub fn prompt(self) -> InquireResult<T>
  where
    T: Clone,
  {
    CustomType::from(self).prompt()
  }
}

impl<'a, T> From<Custom<'a, T>> for CustomType<'a, T> {
  fn from(value: Custom<'a, T>) -> Self {
    CustomType {
      message: value.message,
      starting_input: value.starting_input,
      default: value.default,
      placeholder: value.placeholder,
      help_message: value.help_message,
      formatter: value.formatter,
      default_value_formatter: value.default_value_formatter,
      parser: value.parser,
      validators: value.validators,
      error_message: value.error_message,
      render_config: value.render_config,
    }
  }
}

impl<'a, T> From<&Custom<'a, T>> for CustomType<'a, T>
where
  T: Clone,
{
  fn from(value: &Custom<'a, T>) -> Self {
    CustomType {
      message: value.message,
      starting_input: value.starting_input,
      default: value.default.clone(),
      placeholder: value.placeholder,
      help_message: value.help_message,
      formatter: value.formatter,
      default_value_formatter: value.default_value_formatter,
      parser: value.parser,
      validators: value.validators.clone(),
      error_message: value.error_message.clone(),
      render_config: value.render_config,
    }
  }
}
