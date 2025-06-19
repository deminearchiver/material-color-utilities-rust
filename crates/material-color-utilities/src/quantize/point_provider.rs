use crate::utils;

pub trait PointProvider<const N: usize> {
  /// The four components in the color space of an sRGB color.
  fn from_int(&self, argb: u32) -> [f64; N];

  /// The ARGB (i.e. hex code) representation of this color.
  fn to_int(&self, point: [f64; N]) -> u32;

  /// Squared distance between two colors. Distance is defined by scientific color spaces and
  /// referred to as delta E.
  fn distance(&self, a: [f64; N], b: [f64; N]) -> f64;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointProviderLab {}

impl PointProvider<3> for PointProviderLab {
  fn from_int(&self, argb: u32) -> [f64; 3] {
    utils::color::lab_from_argb(argb)
  }

  fn to_int(&self, point: [f64; 3]) -> u32 {
    utils::color::argb_from_lab(point[0], point[1], point[2])
  }

  fn distance(&self, one: [f64; 3], two: [f64; 3]) -> f64 {
    let d_l = one[0] - two[0];
    let d_a = one[1] - two[1];
    let d_b = one[2] - two[2];
    d_l * d_l + d_a * d_a + d_b * d_b
  }
}
