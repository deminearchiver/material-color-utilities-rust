use crate::utils;

/// An interface to allow use of different color spaces by
/// quantizers.
pub trait PointProvider {
  type Point;

  /// The four components in the color space of an sRGB color.
  fn from_int(&self, argb: u32) -> Self::Point;

  /// The ARGB (i.e. hex code) representation of this color.
  fn to_int(&self, point: Self::Point) -> u32;

  /// Squared distance between two colors. Distance is defined by scientific color spaces and
  /// referred to as delta E.
  fn distance(&self, a: &Self::Point, b: &Self::Point) -> f64;
}

/// Provides conversions needed for K-Means quantization. Converting input to
/// points, and converting the final state of the K-Means algorithm to colors.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointProviderLab;

impl PointProvider for PointProviderLab {
  type Point = [f64; 3];

  /// Convert a color represented in ARGB to a 3-element array of L*a*b*
  /// coordinates of the color.
  fn from_int(&self, argb: u32) -> Self::Point {
    utils::color::lab_from_argb(argb)
  }

  /// Convert a 3-element array to a color represented in ARGB.
  fn to_int(&self, point: Self::Point) -> u32 {
    utils::color::argb_from_lab(point[0], point[1], point[2])
  }

  /// Standard CIE 1976 delta E formula also takes the square root, unneeded
  /// here. This method is used by quantization algorithms to compare distance,
  /// and the relative ordering is the same, with or without a square root.
  ///
  /// This relatively minor optimization is helpful because this method is
  /// called at least once for each pixel in an image.
  fn distance(&self, one: &Self::Point, two: &Self::Point) -> f64 {
    let d_l = one[0] - two[0];
    let d_a = one[1] - two[1];
    let d_b = one[2] - two[2];
    d_l * d_l + d_a * d_a + d_b * d_b
  }
}
