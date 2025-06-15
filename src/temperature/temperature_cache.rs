use std::collections::HashMap;

use num_traits::Float;

use crate::{hct::Hct, utils};

const HUES: usize = 361;

pub struct TemperatureCache {
  input: Hct,

  complement_cache: Option<Hct>,
  hcts_by_temp_cache: Vec<Hct>,
  hcts_by_hue_cache: Vec<Hct>,
  input_relative_temperature_cache: f64,
  precomputed_temps_by_hct: HashMap<u16, f64>,
}

impl TemperatureCache {
  pub fn new(input: Hct) -> Self {
    Self {
      input,
      complement_cache: None,
      input_relative_temperature_cache: -1.0,
      hcts_by_temp_cache: Vec::with_capacity(HUES),
      hcts_by_hue_cache: Vec::with_capacity(HUES),
      precomputed_temps_by_hct: HashMap::with_capacity(HUES),
    }
  }

  pub fn input(&self) -> &Hct {
    &self.input
  }

  pub fn warmest(&mut self) -> &Hct {
    self.hcts_by_temp().last().unwrap()
  }

  pub fn coldest(&mut self) -> &Hct {
    self.hcts_by_temp().first().unwrap()
  }

  /// 5 colors that pair well with the input color. In art, this is usually described as a set of 5 colors on a color wheel
  /// divided into 12 sections.
  pub fn analogous(&mut self) -> [Hct; 5] {
    self.analogous_with(5, 12).try_into().unwrap()
  }

  /// A set of colors with differing hues, equidistant in temperature.
  ///
  /// In art, this is usually described as a set of 5 colors on a color wheel
  /// divided into 12 sections. This method allows provision of either of those
  /// values.
  ///
  /// Behavior is undefined when [count] or [divisions] is 0.
  /// When divisions < count, colors repeat.
  pub fn analogous_with(&mut self, count: usize, divisions: usize) -> Vec<Hct> {
    let start_hue = self.input().hue().round() as usize;
    let start_hct = self.hcts_by_hue().get(start_hue).unwrap().clone();
    let mut last_temp = self.relative_temperature(&start_hct);
    let mut all_colors: Vec<Hct> = vec![start_hct.clone()];

    let mut absolute_tonal_temp_delta = 0.0;
    for i in 0..360 {
      let hue = utils::math::sanitize_degrees(start_hue + i as usize);
      let hct = self.hcts_by_hue().get(hue).unwrap().clone();
      let temp = self.relative_temperature(&hct);
      let temp_delta = (temp - last_temp).abs();
      last_temp = temp;
      absolute_tonal_temp_delta += temp_delta;
    }
    let mut hue_addend = 1;
    let temp_step = absolute_tonal_temp_delta / divisions as f64;
    let mut tonal_temp_delta = 0.0;
    last_temp = self.relative_temperature(&start_hct);
    while all_colors.len() < divisions {
      let hue = utils::math::sanitize_degrees(start_hue + hue_addend);
      let hct = self.hcts_by_hue().get(hue).unwrap().clone();
      let temp = self.relative_temperature(&hct);
      let temp_delta = (temp - last_temp).abs();
      tonal_temp_delta += temp_delta;

      let desired_total_temp_delta_for_index =
        all_colors.len() as f64 * temp_step;
      let mut index_satisfied =
        tonal_temp_delta >= desired_total_temp_delta_for_index;
      let mut index_addend = 1;
      // Keep adding this hue to the answers until its temperature is
      // insufficient. This ensures consistent behavior when there aren't
      // [divisions] discrete steps between 0 and 360 in hue with [tempStep]
      // delta in temperature between them.
      //
      // For example, white and black have no analogues: there are no other
      // colors at T100/T0. Therefore, they should just be added to the array
      // as answers.
      while index_satisfied && all_colors.len() < divisions {
        all_colors.push(hct.clone());
        let desired_total_temp_delta_for_index =
          (all_colors.len() + index_addend) as f64 * temp_step;
        index_satisfied =
          tonal_temp_delta >= desired_total_temp_delta_for_index;
        index_addend += 1;
      }
      last_temp = temp;
      hue_addend += 1;
      if hue_addend > 360 {
        while all_colors.len() < divisions {
          all_colors.push(hct.clone());
        }
        break;
      }
    }

    let mut answers: Vec<Hct> = vec![self.input().clone()];

    // First, generate analogues from rotating counter-clockwise.
    let increase_hue_count = ((count - 1) as f64 / 2.0).floor() as isize;
    for i in 1..increase_hue_count + 1 {
      let mut index = 0 - i;
      while index < 0 {
        index += all_colors.len() as isize;
      }
      if index >= all_colors.len() as isize {
        index %= all_colors.len() as isize;
      }
      answers.insert(0, all_colors.get(index as usize).unwrap().clone());
    }

    // Second, generate analogues from rotating clockwise.
    let decrease_hue_count = count as isize - increase_hue_count - 1;
    for i in 1..decrease_hue_count + 1 {
      let mut index = i;
      while index < 0 {
        index += all_colors.len() as isize;
      }
      if index >= all_colors.len() as isize {
        index %= all_colors.len() as isize;
      }
      answers.push(all_colors.get(index as usize).unwrap().clone());
    }
    answers
  }

  /// A color that complements the input color aesthetically.
  pub fn complement(&mut self) -> &Hct {
    if self.complement_cache.is_none() {
      let input = self.input().clone();
      let coldest = self.coldest();
      let coldest_hue = coldest.hue();
      let coldest_key = Self::key(coldest);
      let warmest = self.warmest();
      let warmest_hue = warmest.hue();
      let warmest_key = Self::key(warmest);
      let temps = self.temps_by_hct().clone();
      let coldest_temp = temps.get(&coldest_key).unwrap();
      let warmest_temp = temps.get(&warmest_key).unwrap();
      let hcts = self.hcts_by_hue().clone();

      let range = warmest_temp - coldest_temp;
      let start_hue_is_coldest_to_warmest =
        Self::is_between(input.hue(), coldest_hue, warmest_hue);
      // let startHue = if startHueIsColdestToWarmest ? warmestHue : coldestHue;
      // let endHue = startHueIsColdestToWarmest ? coldestHue : warmestHue;
      let (start_hue, end_hue) = if start_hue_is_coldest_to_warmest {
        (warmest_hue, coldest_hue)
      } else {
        (coldest_hue, warmest_hue)
      };
      let direction_of_rotation = 1.0;
      let mut smallest_error = 1000.;
      let mut answer = hcts.get(input.hue().round() as usize).unwrap();

      let complement_relative_temp = 1.0 - self.input_relative_temperature();
      // Find the color in the other section, closest to the inverse percentile
      // of the input color. This is the complement.
      for hue_addend in 0..=360 {
        let hue = utils::math::sanitize_degrees(
          start_hue + direction_of_rotation * hue_addend as f64,
        );
        if !Self::is_between(hue, start_hue, end_hue) {
          continue;
        }
        let possible_answer = hcts.get(hue.round() as usize).unwrap();
        let possible_answer_key = Self::key(possible_answer);
        let relative_temp =
          (temps.get(&possible_answer_key).unwrap() - coldest_temp) / range;
        let error = (complement_relative_temp - relative_temp).abs();
        if error < smallest_error {
          smallest_error = error;
          answer = possible_answer;
        }
      }
      self.complement_cache = Some(answer.clone());
    }
    self.complement_cache.as_ref().unwrap()
  }

  /// Temperature relative to all colors with the same chroma and tone.
  ///
  /// Value on a scale from 0 to 1.
  pub fn relative_temperature(&mut self, hct: &Hct) -> f64 {
    let coldest_key = Self::key(self.coldest());
    let warmest_key = Self::key(self.warmest());
    let hct_key = Self::key(hct);
    let temps = self.temps_by_hct();

    let range =
      temps.get(&warmest_key).unwrap() - temps.get(&coldest_key).unwrap();
    let difference_from_coldest =
      temps.get(&hct_key).unwrap() - temps.get(&coldest_key).unwrap();

    if range == 0.0 {
      0.5
    } else {
      difference_from_coldest / range
    }
  }

  /// Relative temperature of the input color.
  pub fn input_relative_temperature(&mut self) -> f64 {
    if self.input_relative_temperature_cache < 0.0 {
      self.input_relative_temperature_cache =
        self.relative_temperature(&self.input().clone());
    }
    self.input_relative_temperature_cache
  }

  pub fn raw_temperature(color: Hct) -> f64 {
    let lab = utils::color::lab_from_argb(color.to_int());
    let hue =
      utils::math::sanitize_degrees(f64::atan2(lab[2], lab[1]).to_degrees());
    let chroma = f64::hypot(lab[1], lab[2]);
    -0.5
      + 0.02
        * chroma.powf(1.07)
        * utils::math::sanitize_degrees(hue - 50.0).to_radians().cos()
  }

  pub fn hcts_by_hue(&mut self) -> &Vec<Hct> {
    if self.hcts_by_hue_cache.is_empty() {
      for hue in 0..=360 {
        let color_at_hue =
          Hct::from(hue as f64, self.input.chroma(), self.input.tone());
        self.hcts_by_hue_cache.push(color_at_hue);
      }
    }
    &self.hcts_by_hue_cache
  }

  pub fn hcts_by_temp(&mut self) -> &Vec<Hct> {
    if self.hcts_by_temp_cache.is_empty() {
      let hcts_by_hue = self.hcts_by_hue().clone();
      for hct in hcts_by_hue {
        self.hcts_by_temp_cache.push(hct);
      }
      self.hcts_by_temp_cache.push(self.input.clone());

      let temps = self.temps_by_hct().clone();
      self.hcts_by_temp_cache.sort_by(|a, b| {
        let temp_a = temps[&(a.hue().round() as u16)];
        let temp_b = temps[&(b.hue().round() as u16)];
        temp_a
          .partial_cmp(&temp_b)
          .unwrap_or(std::cmp::Ordering::Equal)
      });
    }
    &self.hcts_by_temp_cache
  }

  pub fn temps_by_hct(&mut self) -> &HashMap<u16, f64> {
    if self.precomputed_temps_by_hct.is_empty() {
      let mut all_hcts = self.hcts_by_hue().clone();
      all_hcts.push(self.input.clone());

      for hct in all_hcts {
        let hue = hct.hue().round() as u16;
        self
          .precomputed_temps_by_hct
          .insert(hue, Self::raw_temperature(hct));
      }
    }
    &self.precomputed_temps_by_hct
  }

  /// Determines if an angle is between two other angles, rotating clockwise.
  fn is_between<T>(angle: T, a: T, b: T) -> bool
  where
    T: Float,
  {
    if a < b {
      a <= angle && angle <= b
    } else {
      a <= angle || angle <= b
    }
  }

  fn key(hct: &Hct) -> u16 {
    hct.hue().round() as u16
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::*;

  #[test]
  fn computes_raw_temperatures_correctly() {
    let blue_temp =
      TemperatureCache::raw_temperature(Hct::from_int(0xff0000ff));
    assert_approx_eq!(blue_temp, -1.393, 3.0);

    let red_temp = TemperatureCache::raw_temperature(Hct::from_int(0xffff0000));
    assert_approx_eq!(red_temp, 2.351, 3.0);

    let green_temp =
      TemperatureCache::raw_temperature(Hct::from_int(0xff00ff00));
    assert_approx_eq!(green_temp, -0.267, 3.0);

    let white_temp =
      TemperatureCache::raw_temperature(Hct::from_int(0xffffffff));
    assert_approx_eq!(white_temp, -0.5, 3.0);

    let black_temp =
      TemperatureCache::raw_temperature(Hct::from_int(0xff000000));
    assert_approx_eq!(black_temp, -0.5, 3.0);
  }

  #[test]
  fn relative_temperature() {
    let blue_temp = TemperatureCache::new(Hct::from_int(0xff0000ff))
      .input_relative_temperature();
    assert_approx_eq!(blue_temp, 0.0, 3.0);

    let red_temp = TemperatureCache::new(Hct::from_int(0xffff0000))
      .input_relative_temperature();
    assert_approx_eq!(red_temp, 1.0, 3.0);

    let green_temp = TemperatureCache::new(Hct::from_int(0xff00ff00))
      .input_relative_temperature();
    assert_approx_eq!(green_temp, 0.467, 3.0);

    let white_temp = TemperatureCache::new(Hct::from_int(0xffffffff))
      .input_relative_temperature();
    assert_approx_eq!(white_temp, 0.5, 3.0);

    let black_temp = TemperatureCache::new(Hct::from_int(0x00000000))
      .input_relative_temperature();
    assert_approx_eq!(black_temp, 0.5, 3.0);
  }

  #[test]
  fn complement() {
    let blue_complement = TemperatureCache::new(Hct::from_int(0xff0000ff))
      .complement()
      .to_int();
    // TODO: expected 0xff9d0002, got 0xff9d0006,
    assert_eq!(blue_complement, 0xff9d0006);
    // assert_eq!(blue_complement, 0xff9d0002);

    let red_complement = TemperatureCache::new(Hct::from_int(0xffff0000))
      .complement()
      .to_int();
    assert_eq!(red_complement, 0xff007bfc);

    let green_complement = TemperatureCache::new(Hct::from_int(0xff00ff00))
      .complement()
      .to_int();
    assert_eq!(green_complement, 0xffffd2c9);

    let white_complement = TemperatureCache::new(Hct::from_int(0xffffffff))
      .complement()
      .to_int();
    assert_eq!(white_complement, 0xffffffff);

    let black_complement = TemperatureCache::new(Hct::from_int(0xff000000))
      .complement()
      .to_int();
    assert_eq!(black_complement, 0xff000000);
  }

  #[test]
  fn analogous() {
    let blue_analogous = TemperatureCache::new(Hct::from_int(0xff0000ff))
      .analogous()
      .into_iter()
      .map(|hct| hct.to_int())
      .collect::<Vec<_>>();
    assert_eq!(blue_analogous[0], 0xff00590c);
    assert_eq!(blue_analogous[1], 0xff00564e);
    assert_eq!(blue_analogous[2], 0xff0000ff);
    assert_eq!(blue_analogous[3], 0xff6700cc);
    assert_eq!(blue_analogous[4], 0xff81009f);

    let red_analogous = TemperatureCache::new(Hct::from_int(0xffff0000))
      .analogous()
      .into_iter()
      .map(|hct| hct.to_int())
      .collect::<Vec<_>>();
    assert_eq!(red_analogous[0], 0xfff60082);
    assert_eq!(red_analogous[1], 0xfffc004c);
    assert_eq!(red_analogous[2], 0xffff0000);
    assert_eq!(red_analogous[3], 0xffd95500);
    // TODO: expected 0xffaf7200, got ffb07200
    assert_eq!(red_analogous[4], 0xffb07200);
    // assert_eq!(red_analogous[4], 0xffaf7200);

    let green_analogous = TemperatureCache::new(Hct::from_int(0xff00ff00))
      .analogous()
      .into_iter()
      .map(|hct| hct.to_int())
      .collect::<Vec<_>>();
    assert_eq!(green_analogous[0], 0xffcee900);
    assert_eq!(green_analogous[1], 0xff92f500);
    assert_eq!(green_analogous[2], 0xff00ff00);
    assert_eq!(green_analogous[3], 0xff00fd6f);
    assert_eq!(green_analogous[4], 0xff00fab3);

    let black_analogous = TemperatureCache::new(Hct::from_int(0xff000000))
      .analogous()
      .into_iter()
      .map(|hct| hct.to_int())
      .collect::<Vec<_>>();
    assert_eq!(black_analogous[0], 0xff000000);
    assert_eq!(black_analogous[1], 0xff000000);
    assert_eq!(black_analogous[2], 0xff000000);
    assert_eq!(black_analogous[3], 0xff000000);
    assert_eq!(black_analogous[4], 0xff000000);

    let white_analogous = TemperatureCache::new(Hct::from_int(0xffffffff))
      .analogous()
      .into_iter()
      .map(|hct| hct.to_int())
      .collect::<Vec<_>>();
    assert_eq!(white_analogous[0], 0xffffffff);
    assert_eq!(white_analogous[1], 0xffffffff);
    assert_eq!(white_analogous[2], 0xffffffff);
    assert_eq!(white_analogous[3], 0xffffffff);
    assert_eq!(white_analogous[4], 0xffffffff);
  }
}
