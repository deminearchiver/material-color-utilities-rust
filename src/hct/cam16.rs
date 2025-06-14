use std::f64;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{hct::ViewingConditions, utils};

/// Transforms XYZ color space coordinates to 'cone'/'RGB' responses in CAM16.
pub(crate) const XYZ_TO_CAM16RGB: [[f64; 3]; 3] = [
  [0.401288, 0.650173, -0.051461],
  [-0.250268, 1.204414, 0.045854],
  [-0.002079, 0.048952, 0.953127],
];

/// Transforms 'cone'/'RGB' responses in CAM16 to XYZ color space coordinates.
pub(crate) const CAM16RGB_TO_XYZ: [[f64; 3]; 3] = [
  [1.8620678, -1.0112547, 0.14918678],
  [0.38752654, 0.62144744, -0.00897398],
  [-0.01584150, -0.03412294, 1.0499644],
];

/// CAM16, a color appearance model. Colors are not just defined by their hex code, but rather, a hex
/// code and viewing conditions.
///
/// CAM16 instances also have coordinates in the CAM16-UCS space, called J*, a*, b*, or jstar,
/// astar, bstar in code. CAM16-UCS is included in the CAM16 specification, and should be used when
/// measuring distances between colors.
///
/// In traditional color spaces, a color can be identified solely by the observer's measurement of
/// the color. Color appearance models such as CAM16 also use information about the environment where
/// the color was observed, known as the viewing conditions.
///
/// For example, white under the traditional assumption of a midday sun white point is accurately
/// measured as a slightly chromatic blue by CAM16. (roughly, hue 203, chroma 3, lightness 100)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Cam16 {
  // CAM16 color dimensions, see getters for documentation.
  hue: f64,
  chroma: f64,
  j: f64,
  q: f64,
  m: f64,
  s: f64,

  // Coordinates in UCS space. Used to determine color distance, like delta E equations in L*a*b*.
  jstar: f64,
  astar: f64,
  bstar: f64,
  // Avoid allocations during conversion by pre-allocating an array.
  // temp_array: [f64; 3],
}

impl Cam16 {
  /// All of the CAM16 dimensions can be calculated from 3 of the dimensions, in the following
  /// combinations: - {j or q} and {c, m, or s} and hue - jstar, astar, bstar Prefer using a static
  /// method that constructs from 3 of those dimensions. This constructor is intended for those
  /// methods to use to return all possible dimensions.
  #[allow(clippy::too_many_arguments)]
  fn new(
    hue: f64,
    chroma: f64,
    j: f64,
    q: f64,
    m: f64,
    s: f64,
    jstar: f64,
    astar: f64,
    bstar: f64,
  ) -> Self {
    Self {
      hue,
      chroma,
      j,
      q,
      m,
      s,
      jstar,
      astar,
      bstar,
      // temp_array: [0.0; 3],
    }
  }

  /// Create a CAM16 color from a color, assuming the color was viewed in default viewing conditions.
  pub fn from_int(argb: u32) -> Self {
    Self::from_int_in_viewing_conditions(argb, &ViewingConditions::default())
  }

  /// Create a CAM16 color from a color in defined viewing conditions.
  pub fn from_int_in_viewing_conditions(
    argb: u32,
    viewing_conditions: &ViewingConditions,
  ) -> Self {
    // Transform ARGB int to XYZ
    let red = ((argb & 0x00ff0000) >> 16) as u8;
    let green = ((argb & 0x0000ff00) >> 8) as u8;
    let blue = (argb & 0x000000ff) as u8;
    let red_l: f64 = utils::color::linearized(red);
    let green_l: f64 = utils::color::linearized(green);
    let blue_l: f64 = utils::color::linearized(blue);
    let x = 0.41233895 * red_l + 0.35762064 * green_l + 0.18051042 * blue_l;
    let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
    let z = 0.01932141 * red_l + 0.11916382 * green_l + 0.95034478 * blue_l;

    Self::from_xyz_in_viewing_conditions(x, y, z, viewing_conditions)
  }

  pub fn from_xyz_in_viewing_conditions(
    x: f64,
    y: f64,
    z: f64,
    viewing_conditions: &ViewingConditions,
  ) -> Self {
    // Transform XYZ to 'cone'/'rgb' responses
    let matrix = XYZ_TO_CAM16RGB;
    let r_t = (x * matrix[0][0]) + (y * matrix[0][1]) + (z * matrix[0][2]);
    let g_t = (x * matrix[1][0]) + (y * matrix[1][1]) + (z * matrix[1][2]);
    let b_t = (x * matrix[2][0]) + (y * matrix[2][1]) + (z * matrix[2][2]);

    // Discount illuminant
    let r_d = viewing_conditions.rgb_d()[0] * r_t;
    let g_d = viewing_conditions.rgb_d()[1] * g_t;
    let b_d = viewing_conditions.rgb_d()[2] * b_t;

    // Chromatic adaptation
    let r_a_f = (viewing_conditions.fl() * r_d.abs() / 100.0).powf(0.42);
    let g_a_f = (viewing_conditions.fl() * g_d.abs() / 100.0).powf(0.42);
    let b_a_f = (viewing_conditions.fl() * b_d.abs() / 100.0).powf(0.42);
    let r_a = r_d.signum() * 400.0 * r_a_f / (r_a_f + 27.13);
    let g_a = g_d.signum() * 400.0 * g_a_f / (g_a_f + 27.13);
    let b_a = b_d.signum() * 400.0 * b_a_f / (b_a_f + 27.13);

    // redness-greenness
    let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
    // yellowness-blueness
    let b = (r_a + g_a - 2.0 * b_a) / 9.0;

    // auxiliary components
    let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
    let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;

    // hue
    let atan2 = f64::atan2(b, a);
    let atan_degrees = atan2.to_degrees();
    let hue = if atan_degrees < 0.0 {
      atan_degrees + 360.0
    } else if atan_degrees >= 360.0 {
      atan_degrees - 360.0
    } else {
      atan_degrees
    };
    let hue_radians = hue.to_radians();

    // achromatic response to color
    let ac = p2 * viewing_conditions.nbb();

    // CAM16 lightness and brightness
    let j = 100.0
      * f64::powf(
        ac / viewing_conditions.aw(),
        viewing_conditions.c() * viewing_conditions.z(),
      );
    let q = 4.0 / viewing_conditions.c()
      * (j / 100.0).sqrt()
      * (viewing_conditions.aw() + 4.0)
      * viewing_conditions.fl_root();

    // CAM16 chroma, colorfulness, and saturation.
    let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
    let e_hue = 0.25 * ((hue_prime.to_radians() + 2.0).cos() + 3.8);
    let p1 = 50000.0 / 13.0
      * e_hue
      * viewing_conditions.nc()
      * viewing_conditions.ncb();
    let t = p1 * f64::hypot(a, b) / (u + 0.305);
    let alpha =
      (1.64 - 0.29_f64.powf(viewing_conditions.n())).powf(0.73) * t.powf(0.9);

    // CAM16 chroma, colorfulness, saturation
    let c = alpha * (j / 100.0).sqrt();
    let m = c * viewing_conditions.fl_root();
    let s = 50.0
      * ((alpha * viewing_conditions.c()) / (viewing_conditions.aw() + 4.0))
        .sqrt();
    // CAM16-UCS components
    let jstar = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
    let mstar = 1.0 / 0.0228 * (0.0228 * m).ln_1p();
    let astar = mstar * hue_radians.cos();
    let bstar = mstar * hue_radians.sin();

    Self::new(hue, c, j, q, m, s, jstar, astar, bstar)
  }

  pub fn from_jch(j: f64, c: f64, h: f64) -> Self {
    Self::from_jch_in_viewing_conditions(j, c, h, &ViewingConditions::default())
  }
  pub fn from_jch_in_viewing_conditions(
    j: f64,
    c: f64,
    h: f64,
    viewing_conditions: &ViewingConditions,
  ) -> Self {
    let q = 4.0 / viewing_conditions.c()
      * (j / 100.0).sqrt()
      * (viewing_conditions.aw() + 4.0)
      * viewing_conditions.fl_root();
    let m = c * viewing_conditions.fl_root();
    let alpha = c / (j / 100.0).sqrt();
    let s = 50.0
      * ((alpha * viewing_conditions.c()) / (viewing_conditions.aw() + 4.0))
        .sqrt();

    let hue_radians = h.to_radians();
    let jstar = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
    let mstar = 1.0 / 0.0228 * (0.0228 * m).ln_1p();
    let astar = mstar * hue_radians.cos();
    let bstar = mstar * hue_radians.sin();
    Self::new(h, c, j, q, m, s, jstar, astar, bstar)
  }

  /// Create a CAM16 color from CAM16-UCS coordinates.
  pub fn from_ucs(jstar: f64, astar: f64, bstar: f64) -> Self {
    Self::from_ucs_in_viewing_conditions(
      jstar,
      astar,
      bstar,
      &ViewingConditions::default(),
    )
  }

  /// Create a CAM16 color from CAM16-UCS coordinates in defined viewing conditions.
  pub fn from_ucs_in_viewing_conditions(
    jstar: f64,
    astar: f64,
    bstar: f64,
    viewing_conditions: &ViewingConditions,
  ) -> Self {
    let m = f64::hypot(astar, bstar);
    let m2 = (m * 0.0228).exp_m1() / 0.0228;
    let c = m2 / viewing_conditions.fl_root();
    let mut h = f64::atan2(bstar, astar) * (180.0 / f64::consts::PI);
    if h < 0.0 {
      h += 360.0;
    }
    let j = jstar / (1.0 - (jstar - 100.0) * 0.007);
    Self::from_jch_in_viewing_conditions(j, c, h, viewing_conditions)
  }

  /// Hue in CAM16
  pub fn hue(&self) -> f64 {
    self.hue
  }

  /// Chroma in CAM16
  pub fn chroma(&self) -> f64 {
    self.chroma
  }

  /// Lightness in CAM16
  pub fn j(&self) -> f64 {
    self.j
  }

  /// Brightness in CAM16.
  ///
  /// Prefer lightness, brightness is an absolute quantity. For example, a sheet of white paper is
  /// much brighter viewed in sunlight than in indoor light, but it is the lightest object under any
  /// lighting.
  pub fn q(&self) -> f64 {
    self.q
  }

  /// Colorfulness in CAM16.
  ///
  /// Prefer chroma, colorfulness is an absolute quantity. For example, a yellow toy car is much
  /// more colorful outside than inside, but it has the same chroma in both environments.
  pub fn m(&self) -> f64 {
    self.m
  }

  /// Saturation in CAM16.
  ///
  /// Colorfulness in proportion to brightness. Prefer chroma, saturation measures colorfulness
  /// relative to the color's own brightness, where chroma is colorfulness relative to white.
  pub fn s(&self) -> f64 {
    self.s
  }

  /// Lightness coordinate in CAM16-UCS
  pub fn jstar(&self) -> f64 {
    self.jstar
  }

  /// a* coordinate in CAM16-UCS
  pub fn astar(&self) -> f64 {
    self.astar
  }

  /// b* coordinate in CAM16-UCS
  pub fn bstar(&self) -> f64 {
    self.bstar
  }

  /// CAM16 instances also have coordinates in the CAM16-UCS space, called J*, a*, b*, or jstar,
  /// astar, bstar in code. CAM16-UCS is included in the CAM16 specification, and is used to measure
  /// distances between colors.
  pub fn distance(&self, other: &Cam16) -> f64 {
    let d_j = self.jstar() - other.jstar();
    let d_a = self.astar() - other.astar();
    let d_b = self.bstar() - other.bstar();
    let d_e_prime = (d_j * d_j + d_a * d_a + d_b * d_b).sqrt();
    let d_e = 1.41 * d_e_prime.powf(0.63);
    #[allow(clippy::let_and_return)]
    d_e
  }

  /// ARGB representation of the color. Assumes the color was viewed in default viewing conditions,
  /// which are near-identical to the default viewing conditions for sRGB.
  pub fn to_int(&self) -> u32 {
    self.viewed(&ViewingConditions::default())
  }

  /// ARGB representation of the color, in defined viewing conditions.
  pub fn viewed(&self, viewing_conditions: &ViewingConditions) -> u32 {
    let xyz = self.xyz_in_viewing_conditions(viewing_conditions);
    utils::color::argb_from_xyz(xyz[0], xyz[1], xyz[2])
  }

  pub fn xyz_in_viewing_conditions(
    &self,
    viewing_conditions: &ViewingConditions,
  ) -> [f64; 3] {
    let alpha = if self.chroma() == 0.0 || self.j() == 0.0 {
      0.0
    } else {
      self.chroma() / (self.j() / 100.0).sqrt()
    };

    let t = f64::powf(
      alpha / f64::powf(1.64 - f64::powf(0.29, viewing_conditions.n()), 0.73),
      1.0 / 0.9,
    );
    let h_rad = self.hue.to_radians();

    let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
    let ac = viewing_conditions.aw()
      * f64::powf(
        self.j() / 100.0,
        1.0 / viewing_conditions.c() / viewing_conditions.z(),
      );
    let p1 = e_hue
      * (50000.0 / 13.0)
      * viewing_conditions.nc()
      * viewing_conditions.ncb();
    let p2 = ac / viewing_conditions.nbb();

    let h_sin = h_rad.sin();
    let h_cos = h_rad.cos();

    let gamma = 23.0 * (p2 + 0.305) * t
      / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
    let a = gamma * h_cos;
    let b = gamma * h_sin;
    let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
    let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
    let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

    let r_c_base = f64::max(0.0, (27.13 * r_a.abs()) / (400.0 - r_a.abs()));
    let r_c = r_a.signum()
      * (100.0 / viewing_conditions.fl())
      * f64::powf(r_c_base, 1.0 / 0.42);
    let g_c_base = f64::max(0.0, (27.13 * g_a.abs()) / (400.0 - g_a.abs()));
    let g_c = g_a.signum()
      * (100.0 / viewing_conditions.fl())
      * f64::powf(g_c_base, 1.0 / 0.42);
    let b_c_base = f64::max(0.0, (27.13 * b_a.abs()) / (400.0 - b_a.abs()));
    let b_c = b_a.signum()
      * (100.0 / viewing_conditions.fl())
      * f64::powf(b_c_base, 1.0 / 0.42);
    let r_f = r_c / viewing_conditions.rgb_d()[0];
    let g_f = g_c / viewing_conditions.rgb_d()[1];
    let b_f = b_c / viewing_conditions.rgb_d()[2];

    let matrix = CAM16RGB_TO_XYZ;
    let x = (r_f * matrix[0][0]) + (g_f * matrix[0][1]) + (b_f * matrix[0][2]);
    let y = (r_f * matrix[1][0]) + (g_f * matrix[1][1]) + (b_f * matrix[1][2]);
    let z = (r_f * matrix[2][0]) + (g_f * matrix[2][1]) + (b_f * matrix[2][2]);

    [x, y, z]
  }
}
