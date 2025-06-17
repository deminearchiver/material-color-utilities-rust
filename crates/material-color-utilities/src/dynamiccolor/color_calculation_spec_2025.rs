use crate::{
  contrast,
  dynamiccolor::{
    ColorCalculationSpec, DeltaConstraint, DynamicColor, DynamicScheme, TonePolarity,
  },
  hct::Hct,
};

pub(crate) struct ColorCalculationSpec2025;

impl ColorCalculationSpec2025 {
  pub const fn new() -> Self {
    Self
  }
}

impl ColorCalculationSpec for ColorCalculationSpec2025 {
  fn get_hct<'a>(&self, scheme: &'a DynamicScheme, color: &DynamicColor<'a>) -> Hct {
    let palette = color.palette()(scheme);
    let tone = self.get_tone(scheme, color);
    let hue = palette.hue();
    let chroma_multiplier = color
      .chroma_multiplier()
      .map(|mut f| f(scheme))
      .unwrap_or(1.0);
    let chroma = palette.chroma() * chroma_multiplier;
    Hct::from(hue, chroma, tone)
  }

  fn get_tone<'a>(&self, scheme: &'a DynamicScheme, color: &DynamicColor<'a>) -> f64 {
    let tone_delta_pair = color.tone_delta_pair().and_then(|mut f| f(scheme));

    // Case 0: tone delta pair.
    if let Some(tone_delta_pair) = tone_delta_pair {
      let role_a = tone_delta_pair.role_a();
      let role_b = tone_delta_pair.role_b();
      let polarity = tone_delta_pair.polarity();
      let constraint = tone_delta_pair.constraint();
      let absolute_delta = if polarity == &TonePolarity::Darker
        || (polarity == &TonePolarity::RelativeLighter && scheme.is_dark())
        || (polarity == &TonePolarity::RelativeDarker && !scheme.is_dark())
      {
        -tone_delta_pair.delta()
      } else {
        tone_delta_pair.delta()
      };

      let am_role_a = color.name() == role_a.name();
      let self_role = if am_role_a { role_a } else { role_b };
      let reference_role = if am_role_a { role_b } else { role_a };
      let mut self_tone = self_role.tone()(scheme);
      let reference_tone = reference_role.get_tone(scheme);
      let relative_delta = absolute_delta * (if am_role_a { 1.0 } else { -1.0 });

      self_tone = match *constraint {
        DeltaConstraint::Exact => (reference_tone + relative_delta).clamp(0.0, 100.0),
        DeltaConstraint::Nearer => {
          if relative_delta > 0.0 {
            self_tone
              .clamp(reference_tone, reference_tone + relative_delta)
              .clamp(0.0, 100.0)
          } else {
            self_tone
              .clamp(reference_tone + relative_delta, reference_tone)
              .clamp(0.0, 100.0)
          }
        }
        DeltaConstraint::Farther => {
          if relative_delta > 0.0 {
            self_tone.clamp(reference_tone + relative_delta, 100.0)
          } else {
            self_tone.clamp(0.0, reference_tone + relative_delta)
          }
        }
      };

      if let (Some(background), Some(contrast_curve)) = (
        color.background().and_then(|mut f| f(scheme)),
        color.contrast_curve().and_then(|mut f| f(scheme)),
      ) {
        let bg_tone = background.get_tone(scheme);
        let self_contrast = contrast_curve.get(scheme.contrast_level());
        self_tone = if contrast::ratio_of_tones(bg_tone, self_tone) >= self_contrast
          && scheme.contrast_level() >= 0.0
        {
          self_tone
        } else {
          DynamicColor::foreground_tone(bg_tone, self_contrast)
        };
      }

      // This can avoid the awkward tones for background colors including the access fixed colors.
      // Accent fixed dim colors should not be adjusted.
      if color.is_background() && !color.name().ends_with("_fixed_dim") {
        self_tone = if self_tone >= 57.0 {
          self_tone.clamp(65.0, 100.0)
        } else {
          self_tone.clamp(0.0, 49.0)
        };
      }

      self_tone
    } else {
      // Case 1: No tone delta pair; just solve for itself.
      let mut answer = color.tone()(scheme);

      if let (Some(background), Some(contrast_curve)) = (
        color.background().and_then(|mut f| f(scheme)),
        color.contrast_curve().and_then(|mut f| f(scheme)),
      ) {
        let bg_tone = background.get_tone(scheme);
        let desired_ratio = contrast_curve.get(scheme.contrast_level());

        // Recalculate the tone from desired contrast ratio if the current
        // contrast ratio is not enough or desired contrast level is decreasing
        // (<0).
        answer = if contrast::ratio_of_tones(bg_tone, answer) >= desired_ratio
          && scheme.contrast_level() >= 0.0
        {
          answer
        } else {
          DynamicColor::foreground_tone(bg_tone, desired_ratio)
        };

        // This can avoid the awkward tones for background colors including the access fixed colors.
        // Accent fixed dim colors should not be adjusted.
        if color.is_background() && !color.name().ends_with("_fixed_dim") {
          answer = if answer >= 57.0 {
            answer.clamp(65.0, 100.0)
          } else {
            answer.clamp(0.0, 49.0)
          }
        }

        if let Some(second_background) = color.second_background().and_then(|mut f| f(scheme)) {
          // Case 2: Adjust for dual backgrounds.
          let bg_tone1 = background.get_tone(scheme);
          let bg_tone2 = second_background.get_tone(scheme);
          let upper = f64::max(bg_tone1, bg_tone2);
          let lower = f64::min(bg_tone1, bg_tone2);

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

          let prefers_light = DynamicColor::tone_prefers_light_foreground(bg_tone1)
            || DynamicColor::tone_prefers_light_foreground(bg_tone2);
          if prefers_light {
            return if light_option < 0.0 {
              100.0
            } else {
              light_option
            };
          }
          if availables.len() == 1 {
            return *availables.first().unwrap();
          }
          return if dark_option < 0.0 { 0.0 } else { dark_option };
        }
      }

      answer // No adjustment for colors with no background.
    }
  }
}
