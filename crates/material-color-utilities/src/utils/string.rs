use std::convert::Infallible;

use crate::utils::color::{
  alpha_from_argb, argb_from_rgb, blue_from_argb, green_from_argb, red_from_argb,
};

use csscolorparser::{Color, ParseColorError};
pub use csscolorparser::{Color as CssColor, ParseColorError as Error};

#[inline]
pub fn hex_from_argb(argb: u32) -> String {
  let r = red_from_argb(argb);
  let g = green_from_argb(argb);
  let b = blue_from_argb(argb);
  format!("#{r:02x}{g:02x}{b:02x}")
}

pub fn argb_from_hex(hex: &str) -> Result<u32, ParseColorError> {
  let hex = hex.trim();
  if !hex.starts_with("#") {
    return Err(ParseColorError::InvalidHex);
  }
  let color = hex.parse::<Color>()?;
  let [r, g, b, _] = color.to_rgba8();
  let argb = argb_from_rgb(r, g, b);
  Ok(argb)
}

pub fn css_hex_from_argb(argb: u32) -> String {
  Color::from_argb(argb).to_css_hex()
}

pub fn argb_from_css_hex(hex: &str) -> Result<u32, ParseColorError> {
  let hex = hex.trim();
  if hex.starts_with("#") {
    argb_from_css_color(hex)
  } else {
    Err(ParseColorError::InvalidHex)
  }
}

pub fn argb_from_css_color(css_color: &str) -> Result<u32, ParseColorError> {
  css_color.try_parse_argb()
}

pub trait ParseArgb: Sized {
  fn parse_argb(self) -> u32;
}

pub trait TryParseArgb: Sized {
  type Error;

  fn try_parse_argb(self) -> Result<u32, Self::Error>;
}

impl<T> TryParseArgb for T
where
  T: ParseArgb,
{
  type Error = Infallible;

  fn try_parse_argb(self) -> Result<u32, Self::Error> {
    Ok(self.parse_argb())
  }
}

impl TryParseArgb for &str {
  type Error = ParseColorError;

  fn try_parse_argb(self) -> Result<u32, Self::Error> {
    self.parse::<Color>()?.try_parse_argb()
  }
}

impl TryParseArgb for Color {
  type Error = ParseColorError;

  fn try_parse_argb(self) -> Result<u32, Self::Error> {
    let [r, g, b, a] = self.to_rgba8();
    let argb = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    Ok(argb)
  }
}

pub trait FromArgb: Sized {
  fn from_argb(argb: u32) -> Self;
}

pub trait TryFromArgb: Sized {
  type Error;

  fn try_from_argb(argb: u32) -> Result<Self, Self::Error>;
}
impl<T> TryFromArgb for T
where
  T: FromArgb,
{
  type Error = Infallible;

  fn try_from_argb(argb: u32) -> Result<Self, Self::Error> {
    Ok(Self::from_argb(argb))
  }
}

impl FromArgb for Color {
  fn from_argb(argb: u32) -> Self {
    let r = red_from_argb(argb);
    let g = green_from_argb(argb);
    let b = blue_from_argb(argb);
    let a = alpha_from_argb(argb);
    Self::from_rgba8(r, g, b, a)
  }
}
