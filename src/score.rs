use std::collections::HashMap;

use crate::{hct::Hct, utils};

const TARGET_CHROMA: f64 = 48.0; // A1 Chroma
const WEIGHT_PROPORTION: f64 = 0.7;
const WEIGHT_CHROMA_ABOVE: f64 = 0.3;
const WEIGHT_CHROMA_BELOW: f64 = 0.1;
const CUTOFF_CHROMA: f64 = 5.0;
const CUTOFF_EXCITED_PROPORTION: f64 = 0.01;

/// Given a map with keys of colors and values of how often the color appears, rank the colors
/// based on suitability for being used for a UI theme.
///
/// Returns colors sorted by suitability for a UI theme. The most suitable color is the first item,
/// the least suitable is the last. There will always be at least one color returned. If all
/// the input colors were not suitable for a theme, a default fallback color will be provided,
/// Google Blue.
pub fn score_with(
  colors_to_population: HashMap<u32, u16>,
  desired: Option<usize>,
  fallback_color_argb: Option<u32>,
  filter: Option<bool>,
) -> Vec<u32> {
  let desired = desired.unwrap_or(4);
  // Fallback color is Google Blue.
  let fallback_color_argb = fallback_color_argb.unwrap_or(0xff4285f4);
  let filter = filter.unwrap_or(true);

  // Get the HCT color for each Argb value, while finding the per hue count and
  // total count.
  let mut colors_hct: Vec<Hct> = vec![];
  let mut hue_population: [u16; 360] = [0; 360];
  let mut population_sum = 0.0;
  for (key, value) in &colors_to_population {
    let hct = Hct::from_int(*key);
    let hue = hct.hue().floor() as u16;
    colors_hct.push(hct);
    hue_population[hue as usize] += value;
    population_sum += *value as f64;
  }

  // Hues with more usage in neighboring 30 degree slice get a larger number.
  let mut hue_excited_proportions = [0.0; 360];
  for (hue, population) in hue_population.iter().enumerate() {
    let proportion = *population as f64 / population_sum;
    for i in hue as isize - 14_isize..hue as isize + 16 {
      let neighbor_hue = utils::math::sanitize_degrees(i);
      hue_excited_proportions[neighbor_hue as usize] += proportion;
    }
  }

  // Scores each HCT color based on usage and chroma, while optionally
  // filtering out values that do not have enough chroma or usage.
  let mut scored_hcts: Vec<ScoredHct> = vec![];
  for hct in colors_hct {
    let hue = utils::math::sanitize_degrees(hct.hue().round() as usize);
    let proportion = hue_excited_proportions[hue];
    if filter && (hct.chroma() < CUTOFF_CHROMA || proportion <= CUTOFF_EXCITED_PROPORTION) {
      continue;
    }

    let proportion_score = proportion * 100.0 * WEIGHT_PROPORTION;
    let chroma_weight = if hct.chroma() < TARGET_CHROMA {
      WEIGHT_CHROMA_BELOW
    } else {
      WEIGHT_CHROMA_ABOVE
    };
    let chroma_score = (hct.chroma() - TARGET_CHROMA) * chroma_weight;
    let score = proportion_score + chroma_score;
    scored_hcts.push(ScoredHct::new(hct, score));
  }
  // Sorted so that colors with higher scores come first.
  scored_hcts.sort_by(|a, b| a.score.total_cmp(&b.score));

  // Iterates through potential hue differences in degrees in order to select
  // the colors with the largest distribution of hues possible. Starting at
  // 90 degrees(maximum difference for 4 colors) then decreasing down to a
  // 15 degree minimum.
  let mut chosen_colors: Vec<Hct> = vec![];
  // for (int differenceDegrees = 90; differenceDegrees >= 15; differenceDegrees--) {
  for difference_degrees in (15..=90u8).rev() {
    chosen_colors.clear();
    for entry in scored_hcts.iter() {
      let hct = &entry.hct;
      let mut has_duplicate_hue = false;
      for chosen_hct in chosen_colors.iter() {
        if utils::math::difference_degrees(hct.hue(), chosen_hct.hue()) < difference_degrees as f64
        {
          has_duplicate_hue = true;
          break;
        }
      }
      if !has_duplicate_hue {
        chosen_colors.push(hct.clone());
      }
      if chosen_colors.len() >= desired {
        break;
      }
    }
    if chosen_colors.len() >= desired {
      break;
    }
  }
  if chosen_colors.is_empty() {
    vec![fallback_color_argb]
  } else {
    chosen_colors
      .into_iter()
      .map(|chosen_hct| chosen_hct.to_int())
      .collect()
  }
}

#[derive(Debug, Default, PartialEq)]
struct ScoredHct {
  pub hct: Hct,
  pub score: f64,
}

impl ScoredHct {
  pub fn new(hct: Hct, score: f64) -> Self {
    Self { hct, score }
  }
}

// TODO: investigate test failures and implement remaining tests
// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn scoring_prioritizes_chroma() {
//     let mut colors_to_population = HashMap::new();
//     colors_to_population.insert(0xff000000, 1);
//     colors_to_population.insert(0xffffffff, 1);
//     colors_to_population.insert(0xff0000ff, 1);

//     let ranked = score_with(colors_to_population, Some(4), None, None);

//     assert_eq!(ranked.len(), 1);
//     assert_eq!(ranked[0], 0xff0000ff);
//   }

//   #[test]
//   fn scoring_prioritizes_chroma_when_proportions_equal() {
//     let mut colors_to_population = HashMap::new();
//     colors_to_population.insert(0xffff0000, 1);
//     colors_to_population.insert(0xff00ff00, 1);
//     colors_to_population.insert(0xff0000ff, 1);

//     let ranked = score_with(colors_to_population, Some(4), None, None);

//     assert_eq!(ranked.len(), 3);
//     assert_eq!(ranked[0], 0xffff0000);
//     assert_eq!(ranked[1], 0xff00ff00);
//     assert_eq!(ranked[2], 0xff0000ff);
//   }
// }
