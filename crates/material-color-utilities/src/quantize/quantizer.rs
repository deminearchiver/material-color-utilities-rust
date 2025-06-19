use crate::quantize::quantizer_result::QuantizerResult;

pub trait Quantizer {
  fn quantize(self, pixels: &[u32], max_colors: usize) -> QuantizerResult;
}
