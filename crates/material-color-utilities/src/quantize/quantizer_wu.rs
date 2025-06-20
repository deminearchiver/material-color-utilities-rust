use crate::quantize::{PointProviderLab, Quantizer, QuantizerResult};

// A histogram of all the input colors is constructed. It has the shape of a
// cube. The cube would be too large if it contained all 16 million colors:
// historical best practice is to use 5 bits  of the 8 in each channel,
// reducing the histogram to a volume of ~32,000.
// const INDEX_BITS: usize = 5;
// const INDEX_COUNT: usize = (1 << INDEX_BITS) + 1;
// const TOTAL_SIZE: usize = INDEX_COUNT * INDEX_COUNT * INDEX_COUNT;

/// An image quantizer that divides the image's pixels into clusters by
/// recursively cutting an RGB cube, based on the weight of pixels in each area
/// of the cube.
///
/// The algorithm was described by Xiaolin Wu in Graphic Gems II, published in
/// 1991.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct QuantizerWu {}

impl QuantizerWu {}

impl Quantizer for QuantizerWu {
  fn quantize(mut self, pixels: &[u32], colorCount: usize) -> QuantizerResult {
    todo!()
  }
}
