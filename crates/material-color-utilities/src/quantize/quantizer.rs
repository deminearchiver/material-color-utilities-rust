use std::collections::HashMap;

pub trait Quantizer {
  fn quantize(self, pixels: &[u32], max_colors: usize) -> QuantizerResult;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QuantizerResult {
  pub color_to_count: HashMap<u32, usize>,
  // pub input_pixel_to_cluster_pixel: HashMap<u32, u32>,
}
