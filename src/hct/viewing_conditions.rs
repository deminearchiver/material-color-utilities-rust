use std::f64;

use crate::{hct::XYZ_TO_CAM16RGB, utils};

/// In traditional color spaces, a color can be identified solely by the observer's measurement of
/// the color. Color appearance models such as CAM16 also use information about the environment where
/// the color was observed, known as the viewing conditions.
///
/// For example, white under the traditional assumption of a midday sun white point is accurately
/// measured as a slightly chromatic blue by CAM16. (roughly, hue 203, chroma 3, lightness 100)
///
/// This class caches intermediate values of the CAM16 conversion process that depend only on
/// viewing conditions, enabling speed ups.
#[derive(Debug, Clone, PartialEq)]
pub struct ViewingConditions {
  aw: f64,
  nbb: f64,
  ncb: f64,
  c: f64,
  nc: f64,
  n: f64,
  rgb_d: [f64; 3],
  fl: f64,
  fl_root: f64,
  z: f64,
}

impl ViewingConditions {
  /// Parameters are intermediate values of the CAM16 conversion process. Their names are shorthand
  /// for technical color science terminology, this class would not benefit from documenting them
  /// individually. A brief overview is available in the CAM16 specification, and a complete overview
  /// requires a color science textbook, such as Fairchild's Color Appearance Models.
  #[allow(clippy::too_many_arguments)]
  fn new(
    aw: f64,
    nbb: f64,
    ncb: f64,
    c: f64,
    nc: f64,
    n: f64,
    rgb_d: [f64; 3],
    fl: f64,
    fl_root: f64,
    z: f64,
  ) -> Self {
    Self {
      aw,
      nbb,
      ncb,
      c,
      nc,
      n,
      rgb_d,
      fl,
      fl_root,
      z,
    }
  }

  /// Create ViewingConditions from a simple, physically relevant, set of parameters.
  pub fn make(
    white_point: [f64; 3],
    adapting_luminance: f64,
    mut background_lstar: f64,
    surround: f64,
    discounting_illuminant: bool,
  ) -> Self {
    // A background of pure black is non-physical and leads to infinities that represent the idea
    // that any color viewed in pure black can't be seen.
    background_lstar = f64::max(0.1, background_lstar);
    // Transform white point XYZ to 'cone'/'rgb' responses
    let matrix = XYZ_TO_CAM16RGB;
    let xyz = white_point;
    let r_w = (xyz[0] * matrix[0][0])
      + (xyz[1] * matrix[0][1])
      + (xyz[2] * matrix[0][2]);
    let g_w = (xyz[0] * matrix[1][0])
      + (xyz[1] * matrix[1][1])
      + (xyz[2] * matrix[1][2]);
    let b_w = (xyz[0] * matrix[2][0])
      + (xyz[1] * matrix[2][1])
      + (xyz[2] * matrix[2][2]);
    let f = 0.8 + (surround / 10.0);
    let c = if f >= 0.9 {
      utils::math::lerp(0.59, 0.69, (f - 0.9) * 10.0)
    } else {
      utils::math::lerp(0.525, 0.59, (f - 0.8) * 10.0)
    };
    let d = if discounting_illuminant {
      1.0
    } else {
      f * (1.0 - ((1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp()))
    }
    .clamp(0.0, 1.0);
    let nc = f;
    let rgb_d = [
      d * (100.0 / r_w) + 1.0 - d,
      d * (100.0 / g_w) + 1.0 - d,
      d * (100.0 / b_w) + 1.0 - d,
    ];
    let k = 1.0 / (5.0 * adapting_luminance + 1.0);
    let k4 = k * k * k * k;
    let k4_f = 1.0 - k4;
    let fl = (k4 * adapting_luminance)
      + (0.1 * k4_f * k4_f * f64::cbrt(5.0 * adapting_luminance));
    let n = utils::color::y_from_lstar(background_lstar) / white_point[1];
    let z = 1.48 + f64::sqrt(n);
    let nbb = 0.725 / n.powf(0.2);
    let ncb = nbb;
    let rgb_a_factors = [
      (fl * rgb_d[0] * r_w / 100.0).powf(0.42),
      (fl * rgb_d[1] * g_w / 100.0).powf(0.42),
      (fl * rgb_d[2] * b_w / 100.0).powf(0.42),
    ];

    let rgb_a = [
      (400.0 * rgb_a_factors[0]) / (rgb_a_factors[0] + 27.13),
      (400.0 * rgb_a_factors[1]) / (rgb_a_factors[1] + 27.13),
      (400.0 * rgb_a_factors[2]) / (rgb_a_factors[2] + 27.13),
    ];

    let aw = ((2.0 * rgb_a[0]) + rgb_a[1] + (0.05 * rgb_a[2])) * nbb;
    Self::new(aw, nbb, ncb, c, nc, n, rgb_d, fl, fl.powf(0.25), z)
  }

  /// Create sRGB-like viewing conditions with a custom background lstar.
  pub fn default_with_background_lstar(lstar: f64) -> Self {
    Self::make(
      utils::color::white_point_d65(),
      200.0 / f64::consts::PI * utils::color::y_from_lstar(50.0) / 100.0,
      lstar,
      2.0,
      false,
    )
  }

  pub fn aw(&self) -> f64 {
    self.aw
  }

  pub fn nbb(&self) -> f64 {
    self.nbb
  }

  pub fn ncb(&self) -> f64 {
    self.ncb
  }

  pub fn c(&self) -> f64 {
    self.c
  }

  pub fn nc(&self) -> f64 {
    self.nc
  }

  pub fn n(&self) -> f64 {
    self.n
  }

  pub fn rgb_d(&self) -> &[f64; 3] {
    &self.rgb_d
  }

  pub fn fl(&self) -> f64 {
    self.fl
  }

  pub fn fl_root(&self) -> f64 {
    self.fl_root
  }

  pub fn z(&self) -> f64 {
    self.z
  }
}

impl Default for ViewingConditions {
  /// sRGB-like viewing conditions.
  fn default() -> Self {
    Self::default_with_background_lstar(50.0)
  }
}
