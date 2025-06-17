use crate::utils;

/// The minimum contrast ratio of two colors.
///
/// Contrast ratio equation = lighter + 5 / darker + 5, if lighter == darker, ratio == 1.
pub const RATIO_MIN: f64 = 1.0;

/// The maximum contrast ratio of two colors.
///
/// Contrast ratio equation = lighter + 5 / darker + 5. Lighter and darker scale from 0 to 100.
/// If lighter == 100, darker = 0, ratio == 21.
pub const RATIO_MAX: f64 = 21.0;

pub const RATIO_30: f64 = 3.0;
pub const RATIO_45: f64 = 4.5;
pub const RATIO_70: f64 = 7.0;

/// Given a color and a contrast ratio to reach, the luminance of a color that reaches that ratio
/// with the color can be calculated. However, that luminance may not contrast as desired, i.e. the
/// contrast ratio of the input color and the returned luminance may not reach the contrast ratio
/// asked for.
///
/// When the desired contrast ratio and the result contrast ratio differ by more than this amount,
/// an error value should be returned, or the method should be documented as 'unsafe', meaning,
/// it will return a valid luminance but that luminance may not meet the requested contrast ratio.
///
/// 0.04 selected because it ensures the resulting ratio rounds to the same tenth.
const CONTRAST_RATIO_EPSILON: f64 = 0.04;

/// Color spaces that measure luminance, such as Y in XYZ, L* in L*a*b*, or T in HCT, are known as
/// perceptually accurate color spaces.
///
/// To be displayed, they must gamut map to a "display space", one that has a defined limit on the
/// number of colors. Display spaces include sRGB, more commonly understood  as RGB/HSL/HSV/HSB.
/// Gamut mapping is undefined and not defined by the color space. Any gamut mapping algorithm must
/// choose how to sacrifice accuracy in hue, saturation, and/or lightness.
///
/// A principled solution is to maintain lightness, thus maintaining contrast/a11y, maintain hue,
/// thus maintaining aesthetic intent, and reduce chroma until the color is in gamut.
///
/// HCT chooses this solution, but, that doesn't mean it will _exactly_ matched desired lightness,
/// if only because RGB is quantized: RGB is expressed as a set of integers: there may be an RGB
/// color with, for example, 47.892 lightness, but not 47.891.
///
/// To allow for this inherent incompatibility between perceptually accurate color spaces and
/// display color spaces, methods that take a contrast ratio and luminance, and return a luminance
/// that reaches that contrast ratio for the input luminance, purposefully darken/lighten their
/// result such that the desired contrast ratio will be reached even if inaccuracy is introduced.
///
/// 0.4 is generous, ex. HCT requires much less delta. It was chosen because it provides a rough
/// guarantee that as long as a perceptual color space gamut maps lightness such that the resulting
/// lightness rounds to the same as the requested, the desired contrast ratio will be reached.
const LUMINANCE_GAMUT_MAP_TOLERANCE: f64 = 0.4;

/// Contrast ratio is a measure of legibility, its used to compare the lightness of two colors.
/// This method is used commonly in industry due to its use by WCAG.
///
/// To compare lightness, the colors are expressed in the XYZ color space, where Y is lightness,
/// also known as relative luminance.
///
/// The equation is ratio = lighter Y + 5 / darker Y + 5.
pub fn ratio_of_ys(y1: f64, y2: f64) -> f64 {
  let lighter = f64::max(y1, y2);
  let darker = if lighter == y2 { y1 } else { y2 };
  (lighter + 5.0) / (darker + 5.0)
}

/// Contrast ratio of two tones. T in HCT, L* in L*a*b*. Also known as luminance or perpectual
/// luminance.
///
/// Contrast ratio is defined using Y in XYZ, relative luminance. However, relative luminance is
/// linear to number of photons, not to perception of lightness. Perceptual luminance, L* in
/// L*a*b*, T in HCT, is. Designers prefer color spaces with perceptual luminance since they're
/// accurate to the eye.
///
/// Y and L* are pure functions of each other, so it possible to use perceptually accurate color
/// spaces, and measure contrast, and measure contrast in a much more understandable way: instead
/// of a ratio, a linear difference. This allows a designer to determine what they need to adjust a
/// color's lightness to in order to reach their desired contrast, instead of guessing & checking
/// with hex codes.
pub fn ratio_of_tones(t1: f64, t2: f64) -> f64 {
  let t1 = t1.clamp(0.0, 100.0);
  let t2 = t2.clamp(0.0, 100.0);
  ratio_of_ys(
    utils::color::y_from_lstar(t1),
    utils::color::y_from_lstar(t2),
  )
}

/// Returns T in HCT, L* in L*a*b* >= tone parameter that ensures ratio with input T/L*. Returns -1
/// if ratio cannot be achieved.
pub fn lighter(tone: f64, ratio: f64) -> f64 {
  if !(0.0..=100.0).contains(&tone) {
    return -1.0;
  }
  // Invert the contrast ratio equation to determine lighter Y given a ratio and darker Y.
  let dark_y = utils::color::y_from_lstar(tone);
  let light_y = ratio * (dark_y + 5.0) - 5.0;
  if !(0.0..=100.0).contains(&light_y) {
    return -1.0;
  }
  let real_contrast = ratio_of_ys(light_y, dark_y);
  let delta = (real_contrast - ratio).abs();
  if real_contrast < ratio && delta > CONTRAST_RATIO_EPSILON {
    return -1.0;
  }

  let return_value = utils::color::lstar_from_y(light_y) + LUMINANCE_GAMUT_MAP_TOLERANCE;
  // NOMUTANTS--important validation step; functions it is calling may change implementation.
  if !(0.0..=100.0).contains(&return_value) {
    return -1.0;
  }
  return_value
}

/// Tone >= tone parameter that ensures ratio. 100 if ratio cannot be achieved.
///
/// This method is unsafe because the returned value is guaranteed to be in bounds, but, the in
/// bounds return value may not reach the desired ratio.
pub fn lighter_unsafe(tone: f64, ratio: f64) -> f64 {
  let lighter_safe = lighter(tone, ratio);
  if lighter_safe < 0.0 {
    100.0
  } else {
    lighter_safe
  }
}

/// Returns T in HCT, L* in L*a*b* <= tone parameter that ensures ratio with input T/L*. Returns -1
/// if ratio cannot be achieved.
pub fn darker(tone: f64, ratio: f64) -> f64 {
  if !(0.0..=100.0).contains(&tone) {
    return -1.0;
  }
  // Invert the contrast ratio equation to determine darker Y given a ratio and lighter Y.
  let light_y = utils::color::y_from_lstar(tone);
  let dark_y = ((light_y + 5.0) / ratio) - 5.0;
  if !(0.0..=100.0).contains(&dark_y) {
    return -1.0;
  }
  let real_contrast = ratio_of_ys(light_y, dark_y);
  let delta = (real_contrast - ratio).abs();
  if real_contrast < ratio && delta > CONTRAST_RATIO_EPSILON {
    return -1.0;
  }

  // For information on 0.4 constant, see comment in lighter(tone, ratio).
  let return_value = utils::color::lstar_from_y(dark_y) - LUMINANCE_GAMUT_MAP_TOLERANCE;
  // NOMUTANTS--important validation step; functions it is calling may change implementation.
  if !(0.0..=100.0).contains(&return_value) {
    return -1.0;
  }
  return_value
}

/// Tone <= tone parameter that ensures ratio. 0 if ratio cannot be achieved.
///
/// This method is unsafe because the returned value is guaranteed to be in bounds, but, the in
/// bounds return value may not reach the desired ratio.
pub fn darker_unsafe(tone: f64, ratio: f64) -> f64 {
  let darker_safe = darker(tone, ratio);
  f64::max(0.0, darker_safe)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::*;

  #[test]
  fn ratio_of_tones_out_of_bounds_input() {
    assert_approx_eq!(21.0, ratio_of_tones(-10.0, 110.0), 0.001)
  }

  #[test]
  fn lighter_impossible_ratio_errors() {
    assert_approx_eq!(-1.0, lighter(90.0, 10.0), 0.001)
  }

  #[test]
  fn lighter_out_of_bounds_input_above_errors() {
    assert_approx_eq!(-1.0, lighter(110.0, 2.0), 0.001)
  }

  #[test]
  fn lighter_out_of_bounds_input_below_errors() {
    assert_approx_eq!(-1.0, lighter(-10.0, 2.0), 0.001)
  }

  #[test]
  fn lighter_unsafe_returns_max_tone() {
    assert_approx_eq!(100.0, lighter_unsafe(100.0, 2.0), 0.001)
  }

  #[test]
  fn darker_impossible_ratio_errors() {
    assert_approx_eq!(-1.0, darker(10.0, 20.0), 0.001)
  }

  #[test]
  fn darker_out_of_bounds_input_above_errors() {
    assert_approx_eq!(-1.0, darker(110.0, 20.0), 0.001)
  }

  #[test]
  fn darker_out_of_bounds_input_below_errors() {
    assert_approx_eq!(-1.0, darker(-10.0, 20.0), 0.001)
  }

  #[test]
  fn darker_unsafe_returns_min_tone() {
    assert_approx_eq!(0.0, darker_unsafe(0.0, 2.0), 0.001)
  }
}
