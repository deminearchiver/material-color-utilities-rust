#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

use crate::hct::Hct;

/// A convenience class for retrieving colors that are constant in hue and chroma, but vary in tone.
///
/// TonalPalette is intended for use in a single thread due to its stateful caching.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TonalPalette {
  #[cfg_attr(feature = "serde", serde(skip_serializing, default))]
  cache: RefCell<HashMap<u8, u32>>,
  hue: f64,
  chroma: f64,
  key_color: Hct,
}

impl TonalPalette {
  fn new(hue: f64, chroma: f64, key_color: Hct) -> Self {
    Self {
      cache: Default::default(),
      hue,
      chroma,
      key_color,
    }
  }

  /// Create tones using the HCT hue and chroma from a color.
  pub fn from_int(argb: u32) -> Self {
    Self::from_hct(Hct::from_int(argb))
  }

  /// Create tones using a HCT color.
  pub fn from_hct(hct: Hct) -> Self {
    Self::new(hct.hue(), hct.hue(), hct)
  }

  /// Create tones from a defined HCT hue and chroma.
  pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
    let key_color = KeyColor::new(hue, chroma).create();
    Self::new(hue, chroma, key_color)
  }

  /// The hue of the Tonal Palette, in HCT. Ranges from 0 to 360.
  pub fn hue(&self) -> f64 {
    self.hue
  }

  /// The chroma of the Tonal Palette, in HCT. Ranges from 0 to ~130 (for sRGB gamut).
  pub fn chroma(&self) -> f64 {
    self.chroma
  }

  /// The key color is the first tone, starting from T50, that matches the palette's chroma.
  pub fn key_color(&self) -> &Hct {
    &self.key_color
  }

  /// Create an ARGB color with HCT hue and chroma of this Tones instance, and the provided HCT tone.
  pub fn tone(&self, tone: u8) -> u32 {
    if let Some(color) = self.cache.borrow().get(&tone) {
      *color
    } else {
      let color = if tone == 99 && Hct::is_yellow(self.hue) {
        Self::average_argb(self.tone(98), self.tone(100))
      } else {
        Hct::from(self.hue, self.chroma, tone as f64).to_int()
      };
      self.cache.borrow_mut().insert(tone, color);
      color
    }
  }

  /// Given a tone, use hue and chroma of palette to create a color, and return it as HCT.
  pub fn hct(&self, tone: f64) -> Hct {
    Hct::from(self.hue, self.chroma, tone)
  }

  fn average_argb(argb1: u32, argb2: u32) -> u32 {
    let red1 = (argb1 >> 16) & 0xff;
    let green1 = (argb1 >> 8) & 0xff;
    let blue1 = argb1 & 0xff;
    let red2 = (argb2 >> 16) & 0xff;
    let green2 = (argb2 >> 8) & 0xff;
    let blue2 = argb2 & 0xff;
    let red = ((red1 + red2) as f64 / 2.0).round() as u32;
    let green = ((green1 + green2) as f64 / 2.0).round() as u32;
    let blue = ((blue1 + blue2) as f64 / 2.0).round() as u32;
    255 << 24 | (red & 255) << 16 | (green & 255) << 8 | (blue & 255)
  }
}

/// Key color is a color that represents the hue and chroma of a tonal palette.
struct KeyColor {
  hue: f64,
  requested_chroma: f64,
  /// Cache that maps tone to max chroma to avoid duplicated HCT calculation.
  chroma_cache: HashMap<u8, f64>,
}

impl KeyColor {
  const MAX_CHROMA_VALUE: f64 = 200.0;

  /// Key color is a color that represents the hue and chroma of a tonal palette
  pub fn new(hue: f64, requested_chroma: f64) -> Self {
    Self {
      hue,
      requested_chroma,
      chroma_cache: HashMap::default(),
    }
  }

  /// Creates a key color from a [hue] and a [chroma]. The key color is the first tone, starting
  /// from T50, matching the given hue and chroma.
  pub fn create(&mut self) -> Hct {
    // Pivot around T50 because T50 has the most chroma available, on
    // average. Thus it is most likely to have a direct answer.
    let pivot_tone = 50;
    let tone_step_size = 1;
    // Epsilon to accept values slightly higher than the requested chroma.
    let epsilon = 0.01;

    // Binary search to find the tone that can provide a chroma that is closest
    // to the requested chroma.
    let mut lower_tone: u8 = 0;
    let mut upper_tone: u8 = 100;
    while lower_tone < upper_tone {
      let mid_tone = (lower_tone + upper_tone) / 2;
      let is_ascending = self.max_chroma(mid_tone) < self.max_chroma(mid_tone + tone_step_size);
      let sufficient_chroma = self.max_chroma(mid_tone) >= self.requested_chroma - epsilon;

      if sufficient_chroma {
        // Either range [lowerTone, midTone] or [midTone, upperTone] has
        // the answer, so search in the range that is closer the pivot tone.
        if u8::abs_diff(lower_tone, pivot_tone) < u8::abs_diff(upper_tone, pivot_tone) {
          upper_tone = mid_tone;
        } else {
          if lower_tone == mid_tone {
            return Hct::from(self.hue, self.requested_chroma, lower_tone as f64);
          }
          lower_tone = mid_tone;
        }
      } else {
        // As there is no sufficient chroma in the midTone, follow the direction to the chroma
        // peak.
        if is_ascending {
          lower_tone = mid_tone + tone_step_size;
        } else {
          // Keep midTone for potential chroma peak.
          upper_tone = mid_tone;
        }
      }
    }

    Hct::from(self.hue, self.requested_chroma, lower_tone as f64)
  }

  /// Find the maximum chroma for a given tone
  fn max_chroma(&mut self, tone: u8) -> f64 {
    *self
      .chroma_cache
      .entry(tone)
      .or_insert_with_key(|tone| Hct::from(self.hue, Self::MAX_CHROMA_VALUE, *tone as f64).chroma())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tonal_palette_of_blue() {
    let blue = TonalPalette::from_int(0xff0000ff);

    assert_eq!(blue.tone(100), 0xffffffff);
    assert_eq!(blue.tone(95), 0xfff1efff);
    assert_eq!(blue.tone(90), 0xffe0e0ff);
    assert_eq!(blue.tone(80), 0xffbec2ff);
    assert_eq!(blue.tone(70), 0xff9da3ff);
    assert_eq!(blue.tone(60), 0xff7c84ff);
    assert_eq!(blue.tone(50), 0xff5a64ff);
    assert_eq!(blue.tone(40), 0xff343dff);
    assert_eq!(blue.tone(30), 0xff0000ef);
    assert_eq!(blue.tone(20), 0xff0001ac);
    assert_eq!(blue.tone(10), 0xff00006e);
    assert_eq!(blue.tone(0), 0xff000000);
  }

  #[test]
  fn key_color_with_exact_chroma() {
    // Requested chroma is exactly achievable at a certain tone.
    let palette = TonalPalette::from_hue_and_chroma(50.0, 60.0);
    let result = palette.key_color();

    let hue_difference = (result.hue() - 50.0).abs();
    assert!(hue_difference < 10.0);
    let chroma_difference = (result.chroma() - 60.0).abs();
    assert!(chroma_difference < 0.5);
    // Tone might vary, but should be within the range from 0 to 100.
    assert!(result.tone() > 0.0);
    assert!(result.tone() < 100.0);
  }

  #[test]
  fn key_color_with_unusually_high_chroma() {
    // Requested chroma is above what is achievable. For Hue 149, chroma peak
    // is 89.6 at Tone 87.9. The result key color's chroma should be close to
    // the chroma peak.
    let palette = TonalPalette::from_hue_and_chroma(149.0, 200.0);
    let result = palette.key_color();

    let hue_difference = (result.hue() - 149.0).abs();
    assert!(hue_difference < 10.0);
    assert!(result.chroma() > 89.0);
    // Tone might vary, but should be within the range from 0 to 100.
    assert!(result.tone() > 0.0);
    assert!(result.tone() < 100.0);
  }

  #[test]
  fn key_color_with_unusually_low_chroma() {
    // By definition, the key color should be the first tone, starting from
    // Tone 50, matching the given hue and chroma. When requesting a very low
    // chroma, the result should be close to Tone 50, since most tones can
    // produce a low chroma.
    let palette = TonalPalette::from_hue_and_chroma(50.0, 3.0);
    let result = palette.key_color();

    let hue_difference = (result.hue() - 50.0).abs();
    assert!(hue_difference < 10.0);
    let chroma_difference = (result.chroma() - 3.0).abs();
    assert!(chroma_difference < 0.5);
    let tone_difference = (result.tone() - 50.0).abs();
    assert!(tone_difference < 0.5);
  }
}
