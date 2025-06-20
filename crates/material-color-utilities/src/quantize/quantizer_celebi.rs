use crate::quantize::{Quantizer, QuantizerResult, QuantizerWsmeans, QuantizerWu};

/// An image quantizer that improves on the quality of a standard K-Means
/// algorithm by setting the K-Means initial state to the output of a Wu
/// quantizer, instead of random centroids. Improves on speed by several
/// optimizations, as implemented in Wsmeans, or Weighted Square Means, K-Means
/// with those optimizations.
///
/// This algorithm was designed by M. Emre Celebi, and was found in their 2011
/// paper, Improving the Performance of K-Means for Color Quantization.
/// https://arxiv.org/abs/1101.0395
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuantizerCelebi;

impl Quantizer for QuantizerCelebi {
  fn quantize(self, pixels: &[u32], max_colors: usize) -> QuantizerResult {
    let wu = QuantizerWu::default();
    let wu_result = wu.quantize(pixels, max_colors);
    let wu_clusters: Vec<_> = wu_result.color_to_count.into_keys().collect();
    let wsmeans = QuantizerWsmeans::default();
    wsmeans.quantize_with_starting_clusters(pixels, max_colors, &wu_clusters)
  }
}
