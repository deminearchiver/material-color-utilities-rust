use std::collections::HashMap;

use rand::{SeedableRng, rngs::StdRng};

use crate::quantize::{PointProvider, PointProviderLab, Quantizer, QuantizerResult};

/// An image quantizer that improves on the speed of a standard K-Means algorithm
/// by implementing several optimizations, including deduping identical pixels
/// and a triangle inequality rule that reduces the number of comparisons needed
/// to identify which cluster a point should be moved to.
///
/// Wsmeans stands for Weighted Square Means.
///
/// This algorithm was designed by M. Emre Celebi, and was found in their 2011
/// paper, Improving the Performance of K-Means for Color Quantization.
/// https://arxiv.org/abs/1101.0395
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuantizerWsmeans {}

impl QuantizerWsmeans {
  const MAX_ITERATIONS: usize = 10;
  const MIN_MOVEMENT_DISTANCE: f64 = 3.0;

  /// Reduce the number of colors needed to represented the input, minimizing the difference between
  /// the original image and the recolored image.
  pub fn quantize_with_starting_clusters(
    self,
    inputPixels: &[u32],      // TODO: rename to `pixels`
    maxColors: usize,         // TODO: rename to `max_colors`
    startingClusters: &[u32], // TODO: rename to `starting_clusters`
  ) -> QuantizerResult {
    // Uses a seeded random number generator to ensure consistent results.
    let mut rng = StdRng::seed_from_u64(0x42688);

    let pointProvider = PointProviderLab;

    let mut pixelToCount: HashMap<u32, usize> = HashMap::new();
    let mut points: Vec<[f64; 3]> = Vec::with_capacity(inputPixels.len());
    let mut pixels: Vec<u32> = Vec::with_capacity(inputPixels.len());
    let mut pointCount = 0;

    for &input_pixel in inputPixels {
      pixelToCount
        .entry(input_pixel)
        .and_modify(|pixel_count| *pixel_count += 1)
        .or_insert_with(|| {
          points[pointCount] = pointProvider.from_int(input_pixel);
          pixels[pointCount] = input_pixel;

          pointCount += 1;
          1
        });
    }

    let mut counts = vec![0; pointCount];
    for i in 0..pointCount {
      let pixel = pixels[i];
      let count = pixelToCount[&pixel];
      counts[i] = count;
    }

    let mut clusterCount = usize::min(maxColors, pointCount);
    if !startingClusters.is_empty() {
      clusterCount = usize::min(clusterCount, startingClusters.len());
    }

    let mut clusters: Vec<[f64; 3]> = startingClusters
      .iter()
      .map(|starting_cluster| pointProvider.from_int(*starting_cluster))
      .collect();

    todo!()
  }
}

impl Quantizer for QuantizerWsmeans {
  /// Reduce the number of colors needed to represented the input, minimizing the difference between
  /// the original image and the recolored image.
  fn quantize(self, pixels: &[u32], max_colors: usize) -> QuantizerResult {
    self.quantize_with_starting_clusters(pixels, max_colors, &[])
  }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct DistanceAndIndex {
  distance: f64,
  index: isize,
}

impl Default for DistanceAndIndex {
  fn default() -> Self {
    Self {
      distance: -1.0,
      index: -1,
    }
  }
}
