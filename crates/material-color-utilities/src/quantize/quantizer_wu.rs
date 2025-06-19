use std::collections::HashMap;

use crate::{
  quantize::{Quantizer, QuantizerResult, quantizer_map::QuantizerMap},
  utils,
};

// A histogram of all the input colors is constructed. It has the shape of a
// cube. The cube would be too large if it contained all 16 million colors:
// historical best practice is to use 5 bits  of the 8 in each channel,
// reducing the histogram to a volume of ~32,000.
const INDEX_BITS: usize = 5;
const INDEX_COUNT: usize = (1 << INDEX_BITS) + 1;
const TOTAL_SIZE: usize = INDEX_COUNT as usize * INDEX_COUNT as usize * INDEX_COUNT as usize;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct QuantizerWu {
  weights: Vec<usize>,
  moments_r: Vec<usize>,
  moments_g: Vec<usize>,
  moments_b: Vec<usize>,
  moments: Vec<f64>,
  cubes: Vec<Cube>,
}

impl QuantizerWu {
  fn get_index(r: usize, g: usize, b: usize) -> usize {
    ((r as usize) << (INDEX_BITS * 2))
      + ((r as usize) << (INDEX_BITS + 1))
      + (r as usize)
      + ((g as usize) << INDEX_BITS)
      + (g as usize)
      + (b as usize)
  }
  fn volume(cube: &Cube, moment: &[usize]) -> usize {
    moment[Self::get_index(cube.r1, cube.g1, cube.b1)]
      - moment[Self::get_index(cube.r1, cube.g1, cube.b0)]
      - moment[Self::get_index(cube.r1, cube.g0, cube.b1)]
      + moment[Self::get_index(cube.r1, cube.g0, cube.b0)]
      - moment[Self::get_index(cube.r0, cube.g1, cube.b1)]
      + moment[Self::get_index(cube.r0, cube.g1, cube.b0)]
      + moment[Self::get_index(cube.r0, cube.g0, cube.b1)]
      - moment[Self::get_index(cube.r0, cube.g0, cube.b0)]
  }

  fn construct_histogram(&mut self, pixels: HashMap<u32, usize>) {
    self.weights = vec![0; TOTAL_SIZE];
    self.moments_r = vec![0; TOTAL_SIZE];
    self.moments_g = vec![0; TOTAL_SIZE];
    self.moments_b = vec![0; TOTAL_SIZE];
    self.moments = vec![0.0; TOTAL_SIZE];

    for (&pixel, &count) in pixels.iter() {
      let red = utils::color::red_from_argb(pixel) as usize;
      let green = utils::color::green_from_argb(pixel) as usize;
      let blue = utils::color::blue_from_argb(pixel) as usize;

      let bits_to_remove = 8 - INDEX_BITS;
      let i_r = (red >> bits_to_remove) + 1;
      let i_g = (green >> bits_to_remove) + 1;
      let i_b = (blue >> bits_to_remove) + 1;
      let index = Self::get_index(i_r, i_g, i_b);

      self.weights[index] += count;
      self.moments_r[index] += red * count;
      self.moments_g[index] += green * count;
      self.moments_b[index] += blue * count;
      self.moments[index] += count as f64
        * ((red as f64 * red as f64) + (green as f64 * green as f64) + (blue as f64 * blue as f64));
    }
  }

  fn create_moments(&mut self) {
    for r in 1..INDEX_COUNT {
      let mut area = vec![0; INDEX_COUNT];
      let mut area_r = vec![0; INDEX_COUNT];
      let mut area_g = vec![0; INDEX_COUNT];
      let mut area_b = vec![0; INDEX_COUNT];
      let mut area2: Vec<f64> = vec![0.0; INDEX_COUNT];

      for g in 1..INDEX_COUNT {
        let mut line = 0;
        let mut line_r = 0;
        let mut line_g = 0;
        let mut line_b = 0;
        let mut line2 = 0.0;
        for b in 1..INDEX_COUNT {
          let index = Self::get_index(r, g, b);
          line += self.weights[index];
          line_r += self.moments_r[index];
          line_g += self.moments_g[index];
          line_b += self.moments_b[index];
          line2 += self.moments[index];

          area[b] += line;
          area_r[b] += line_r;
          area_g[b] += line_g;
          area_b[b] += line_b;
          area2[b] += line2;

          let previous_index = Self::get_index(r - 1, g, b);
          self.weights[index] = self.weights[previous_index] + area[b];
          self.moments_r[index] = self.moments_r[previous_index] + area_r[b];
          self.moments_g[index] = self.moments_g[previous_index] + area_g[b];
          self.moments_b[index] = self.moments_b[previous_index] + area_b[b];
          self.moments[index] = self.moments[previous_index] + area2[b];
        }
      }
    }
  }

  fn create_boxes(&mut self, maxColorCount: usize) -> CreateBoxesResult {}
  fn create_result(&self, colorCount: usize) -> Vec<u32> {
    let colors: Vec<u32> = vec![];
    for i in 0..colorCount {
      let cube = &self.cubes[i];
      let weight = Self::volume(&cube, &self.weights);
      if weight > 0 {
        let r = Self::volume(&cube, &self.moments_r) / weight;
        let g = Self::volume(cube, &self.moments_g) / weight;
        let b = Self::volume(cube, &self.moments_b) / weight;
        let color = utils::color::argb_from_rgb(r, g, b);
        colors.add(color);
      }
    }
    colors
    //     List<Integer> colors = new ArrayList<>();
    // for (int i = 0; i < colorCount; ++i) {
    //   Box cube = cubes[i];
    //   int weight = volume(cube, weights);
    //   if (weight > 0) {
    //     int r = volume(cube, momentsR) / weight;
    //     int g = volume(cube, momentsG) / weight;
    //     int b = volume(cube, momentsB) / weight;
    //     int color = (255 << 24) | ((r & 0x0ff) << 16) | ((g & 0x0ff) << 8) | (b & 0x0ff);
    //     colors.add(color);
    //   }
    // }
    // return colors;
  }
}

impl Quantizer for QuantizerWu {
  fn quantize(mut self, pixels: &[u32], color_count: usize) -> QuantizerResult {
    let map_result = QuantizerMap.quantize(pixels, color_count);
    self.construct_histogram(map_result.color_to_count);
    self.create_moments();
    let create_boxes_result = self.create_boxes(color_count);
    let colors = self.create_result(create_boxes_result.result_count);
    todo!()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
  Red,
  Green,
  Blue,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
struct MaximizeResult {
  cut_location: isize,
  maximum: f64,
}

impl MaximizeResult {
  pub fn new(cut: isize, max: f64) -> Self {
    Self {
      cut_location: cut,
      maximum: max,
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CreateBoxesResult {
  requested_count: usize,
  result_count: usize,
}

impl CreateBoxesResult {
  pub fn new(requested_count: usize, result_count: usize) -> Self {
    Self {
      requested_count,
      result_count,
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
  r0: u8,
  r1: u8,
  g0: u8,
  g1: u8,
  b0: u8,
  b1: u8,
  vol: u32,
}
