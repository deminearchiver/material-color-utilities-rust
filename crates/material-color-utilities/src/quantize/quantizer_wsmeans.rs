use std::collections::HashMap;

use rand::prelude::*;
use rand::rngs::StdRng;

use crate::quantize::{PointProvider, PointProviderLab, Quantizer, QuantizerResult};

const MAX_ITERATIONS: usize = 10;
const MIN_MOVEMENT_DISTANCE: f64 = 3.0;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QuantizerWsmeans {}

impl QuantizerWsmeans {
  /// Reduce the number of colors needed to represented the input, minimizing the difference between
  /// the original image and the recolored image.
  pub fn quantize_with_starting_clusters(
    self,
    inputPixels: &[u32],
    startingClusters: &[u32],
    maxColors: usize,
  ) -> QuantizerResult {
    // TODO: implement the algorithm using idiomatic Rust

    let mut rng = rand::rng();

    let mut pixelToCount: HashMap<u32, usize> = HashMap::default();
    let mut points: Vec<[f64; 3]> = Vec::with_capacity(inputPixels.len());
    let mut pixels: Vec<u32> = Vec::with_capacity(inputPixels.len());
    let pointProvider = PointProviderLab::default();

    let mut pointCount = 0;
    for &input_pixel in inputPixels.iter() {
      pixelToCount
        .entry(input_pixel)
        .and_modify(|count| *count += 1)
        .or_insert_with(|| {
          points[pointCount] = pointProvider.from_int(input_pixel);
          pixels[pointCount] = input_pixel;
          pointCount += 1;
          1
        });
    }

    let mut counts: Vec<usize> = Vec::with_capacity(pointCount);
    for i in 0..pointCount {
      let pixel = pixels[i];
      let count = pixelToCount[&pixel];
      counts[i] = count;
    }

    let mut clusterCount = usize::min(maxColors, pointCount);
    if !startingClusters.is_empty() {
      clusterCount = usize::min(clusterCount, startingClusters.len());
    }
    // let mut clusters: Vec<[f64; 3]> = Vec::with_capacity(clusterCount);
    // for i in 0..startingClusters.len() {
    //   clusters[i] = pointProvider.from_int(startingClusters[i]);
    // }
    let clusters: Vec<_> = startingClusters
      .iter()
      .map(|&value| pointProvider.from_int(value))
      .collect();

    let additional_clusters_needed = clusterCount - clusters.len();
    if startingClusters.is_empty() && additional_clusters_needed > 0 {
      for i in 0..additional_clusters_needed {
        // let l = rng.random::<f64>() * 100.0
      }
    }

    // TODO: replace once the algorithm is implemented
    QuantizerResult::default()
  }
}

impl Quantizer for QuantizerWsmeans {
  /// Reduce the number of colors needed to represented the input, minimizing the difference between
  /// the original image and the recolored image.
  fn quantize(self, pixels: &[u32], max_colors: usize) -> QuantizerResult {
    self.quantize_with_starting_clusters(pixels, &[], max_colors)
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Distance {
  index: usize,
  distance: f64,
}

impl Distance {
  pub fn new(index: usize, distance: f64) -> Self {
    Self { index, distance }
  }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.distance.partial_cmp(&other.distance)
  }
}

impl Ord for Distance {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.distance.total_cmp(&other.distance)
  }
}
