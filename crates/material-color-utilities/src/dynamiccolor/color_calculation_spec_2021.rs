#![allow(deprecated)]

use crate::{
  contrast,
  dynamiccolor::{ColorCalculationSpec, DeltaConstraint, TonePolarity},
  hct::Hct,
};

use super::{DynamicColor, DynamicScheme};

pub(crate) struct ColorCalculationSpec2021;

impl ColorCalculationSpec2021 {
  pub const fn new() -> Self {
    Self
  }
}

impl ColorCalculationSpec for ColorCalculationSpec2021 {
  fn get_hct<'a>(&self, scheme: &'a DynamicScheme, color: &DynamicColor<'a>) -> Hct {
    let tone = self.get_tone(scheme, color);
    color.palette()(scheme).hct(tone)
  }

  fn get_tone<'a>(&self, scheme: &'a DynamicScheme, color: &DynamicColor<'a>) -> f64 {
    let decreasing_contrast = scheme.contrast_level() < 0.0;
    let tone_delta_pair = color.tone_delta_pair().and_then(|mut f| f(scheme));

    // Case 1: dual foreground, pair of colors with delta constraint.
    if let Some(tone_delta_pair) = tone_delta_pair {
      let role_a = tone_delta_pair.role_a();
      let role_b = tone_delta_pair.role_b();
      let delta = tone_delta_pair.delta();
      let polarity = tone_delta_pair.polarity();
      let stay_together = tone_delta_pair.stay_together();

      let a_is_nearer = tone_delta_pair.constraint() == &DeltaConstraint::Nearer
        || (polarity == &TonePolarity::Ligher && !scheme.is_dark())
        || (polarity == &TonePolarity::Darker && !scheme.is_dark());
      let nearer = if a_is_nearer { role_a } else { role_b };
      let farther = if a_is_nearer { role_b } else { role_a };
      let am_nearer = color.name() == nearer.name();
      let expansion_dir = if scheme.is_dark() { 1.0 } else { -1.0 };
      let mut n_tone = nearer.tone()(scheme);
      let mut f_tone = farther.tone()(scheme);

      // 1st round: solve to min, each
      if let (Some(bg), Some(n_contrast_curve), Some(f_contrast_curve)) = (
        color.background().and_then(|mut f| f(scheme)),
        nearer.contrast_curve().and_then(|mut f| f(scheme)),
        farther.contrast_curve().and_then(|mut f| f(scheme)),
      ) {
        let n_contrast = n_contrast_curve.get(scheme.contrast_level());
        let f_contrast = f_contrast_curve.get(scheme.contrast_level());
        let bg_tone = bg.get_tone(scheme);

        // If a color is good enough, it is not adjusted.
        // Initial and adjusted tones for `nearer`
        if contrast::ratio_of_tones(bg_tone, n_tone) < n_contrast {
          n_tone = DynamicColor::foreground_tone(bg_tone, n_contrast);
        }
        // Initial and adjusted tones for `farther`
        if contrast::ratio_of_tones(bg_tone, f_tone) < f_contrast {
          f_tone = DynamicColor::foreground_tone(bg_tone, f_contrast);
        }

        if decreasing_contrast {
          // If decreasing contrast, adjust color to the "bare minimum"
          // that satisfies contrast.
          n_tone = DynamicColor::foreground_tone(bg_tone, n_contrast);
          f_tone = DynamicColor::foreground_tone(bg_tone, f_contrast);
        }
      }

      // If constraint is not satisfied, try another round.
      if (f_tone - n_tone) * expansion_dir < delta {
        // 2nd round: expand farther to match delta.
        f_tone = (n_tone + delta * expansion_dir).clamp(0.0, 100.0);
        // If constraint is not satisfied, try another round.
        if (f_tone - n_tone) * expansion_dir < delta {
          // 3rd round: contract nearer to match delta.
          n_tone = (f_tone - delta * expansion_dir).clamp(0.0, 100.0);
        }
      }
      // Avoids the 50-59 awkward zone.
      if (50.0..60.0).contains(&n_tone) {
        // If `nearer` is in the awkward zone, move it away, together with
        // `farther`.
        if expansion_dir > 0.0 {
          n_tone = 60.0;
          f_tone = f64::max(f_tone, n_tone + delta * expansion_dir);
        } else {
          n_tone = 49.0;
          f_tone = f64::min(f_tone, n_tone + delta * expansion_dir);
        }
      } else if (50.0..60.0).contains(&f_tone) {
        if stay_together {
          // Fixes both, to avoid two colors on opposite sides of the "awkward
          // zone".
          if expansion_dir > 0.0 {
            n_tone = 60.0;
            f_tone = f64::max(f_tone, n_tone + delta * expansion_dir);
          } else {
            n_tone = 49.0;
            f_tone = f64::min(f_tone, n_tone + delta * expansion_dir);
          }
        } else {
          // Not required to stay together; fixes just one.
          if expansion_dir > 0.0 {
            f_tone = 60.0;
          } else {
            f_tone = 49.0;
          }
        }
      }

      // Returns `nTone` if this color is `nearer`, otherwise `fTone`.
      if am_nearer { n_tone } else { f_tone }
    } else {
      // Case 2: No contrast pair; just solve for itself.
      let mut answer = color.tone()(scheme);

      // if (color.background == null
      //   || color.background.apply(scheme) == null
      //   || color.contrastCurve == null
      //   || color.contrastCurve.apply(scheme) == null)
      // {
      //   return answer; // No adjustment for colors with no background.
      // }
      if let (Some(background), Some(contrast_curve)) = (
        color.background().and_then(|mut f| f(scheme)),
        color.contrast_curve().and_then(|mut f| f(scheme)),
      ) {
        let bg_tone = background.get_tone(scheme);
        let desired_ratio = contrast_curve.get(scheme.contrast_level());

        if contrast::ratio_of_tones(bg_tone, answer) >= desired_ratio {
          // Don't "improve" what's good enough.
        } else {
          // Rough improvement.
          answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
        }

        if decreasing_contrast {
          answer = DynamicColor::foreground_tone(bg_tone, desired_ratio);
        }

        if color.is_background() && (50.0..60.0).contains(&answer) {
          // Must adjust
          if contrast::ratio_of_tones(49.0, bg_tone) >= desired_ratio {
            answer = 49.0;
          } else {
            answer = 60.0;
          }
        }

        if let Some(second_background) = color.second_background().and_then(|mut f| f(scheme)) {
          // Case 3: Adjust for dual backgrounds.
          let bg_tone_1 = background.get_tone(scheme);
          let bg_tone_2 = second_background.get_tone(scheme);

          let upper = f64::max(bg_tone_1, bg_tone_2);
          let lower = f64::min(bg_tone_1, bg_tone_2);

          if contrast::ratio_of_tones(upper, answer) >= desired_ratio
            && contrast::ratio_of_tones(lower, answer) >= desired_ratio
          {
            return answer;
          }

          // The darkest light tone that satisfies the desired ratio,
          // or -1 if such ratio cannot be reached.
          let light_option = contrast::lighter(upper, desired_ratio);

          // The lightest dark tone that satisfies the desired ratio,
          // or -1 if such ratio cannot be reached.
          let dark_option = contrast::darker(lower, desired_ratio);

          // Tones suitable for the foreground.
          let mut availables: Vec<f64> = vec![];
          if light_option != -1.0 {
            availables.push(light_option);
          }
          if dark_option != -1.0 {
            availables.push(dark_option);
          }

          let prefers_light = DynamicColor::tone_prefers_light_foreground(bg_tone_1)
            || DynamicColor::tone_prefers_light_foreground(bg_tone_2);
          if prefers_light {
            return if light_option == -1.0 {
              100.0
            } else {
              light_option
            };
          }
          if availables.len() == 1 {
            return *availables.first().unwrap();
          }
          return if dark_option == -1.0 {
            0.0
          } else {
            dark_option
          };
        }

        return answer;
      }
      answer // No adjustment for colors with no background.
    }
  }
}
