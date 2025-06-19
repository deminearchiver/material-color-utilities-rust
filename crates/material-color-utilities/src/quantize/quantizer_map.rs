use std::collections::HashMap;

use crate::{
  quantize::{Quantizer, quantizer_result::QuantizerResult},
  utils,
};

pub fn quantize(pixels: &[u32]) -> HashMap<u32, usize> {
  let mut count_by_color = HashMap::default();
  for &pixel in pixels.iter() {
    let alpha = utils::color::alpha_from_argb(pixel);
    if alpha < 255 {
      continue;
    }
    count_by_color
      .entry(pixel)
      .and_modify(|count| *count += 1)
      .or_insert(1);
  }
  count_by_color
}

pub struct QuantizerMap;

impl Quantizer for QuantizerMap {
  fn quantize(
    self,
    pixels: &[u32],
    _max_colors: usize,
  ) -> super::quantizer_result::QuantizerResult {
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
