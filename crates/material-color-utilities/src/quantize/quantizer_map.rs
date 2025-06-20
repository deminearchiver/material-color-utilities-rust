use crate::quantize::{Quantizer, QuantizerResult};

/// Quantizes an image into a map, with keys of ARGB colors, and values of the
/// number of times that color appears in the image.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuantizerMap;

impl Quantizer for QuantizerMap {
  fn quantize(self, pixels: &[u32], _max_colors: usize) -> QuantizerResult {
    let mut result = QuantizerResult::default();
    for &pixel in pixels.iter() {
      result
        .color_to_count
        .entry(pixel)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    }
    result
  }
}
