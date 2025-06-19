use std::collections::HashSet;

use crate::quantize::{
  Quantizer, QuantizerResult, QuantizerWu, quantizer_wsmeans::QuantizerWsmeans,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuantizerCelebi;

impl Quantizer for QuantizerCelebi {
  fn quantize(self, pixels: &[u32], max_colors: usize) -> QuantizerResult {
    let wu = QuantizerWu::default();
    let mut wu_result = wu.quantize(pixels, max_colors);

    let wu_clusters: Vec<_> = wu_result.color_to_count.into_keys().collect();
    let wsmeans = QuantizerWsmeans::default();
    wsmeans.quantize_with_starting_clusters(pixels, &wu_clusters, max_colors)
  }
}
