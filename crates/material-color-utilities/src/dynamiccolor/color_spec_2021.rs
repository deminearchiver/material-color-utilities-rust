#![allow(deprecated)]

use crate::{
  dislike_analyzer,
  dynamiccolor::{
    ColorSpec, ContrastCurve, DynamicColor, DynamicColorBuilder, DynamicScheme, ToneDeltaPair,
    TonePolarity, Variant,
  },
  hct::Hct,
};

pub(crate) struct ColorSpec2021;

impl ColorSpec2021 {
  pub const fn new() -> Self {
    Self
  }

  fn is_fidelity(scheme: &DynamicScheme) -> bool {
    matches!(*scheme.variant(), Variant::Fidelity | Variant::Content)
  }

  fn is_monochrome(scheme: &DynamicScheme) -> bool {
    matches!(*scheme.variant(), Variant::Monochrome)
  }

  fn find_desired_chroma_by_tone(
    hue: f64,
    chroma: f64,
    tone: f64,
    by_decreasing_tone: bool,
  ) -> f64 {
    let mut answer = tone;
    let mut closest_to_chroma = Hct::from(hue, chroma, tone);
    if closest_to_chroma.chroma() < chroma {
      let mut chroma_peak = closest_to_chroma.chroma();
      while closest_to_chroma.chroma() < chroma {
        answer += if by_decreasing_tone { -1.0 } else { 1.0 };
        let potential_solution = Hct::from(hue, chroma, answer);
        if chroma_peak > potential_solution.chroma() {
          break;
        }
        if (potential_solution.chroma() - chroma).abs() < 0.4 {
          break;
        }
        let potential_delta = (potential_solution.chroma() - chroma).abs();
        let current_delta = (closest_to_chroma.chroma() - chroma).abs();
        let potential_solution_chroma = potential_solution.chroma();
        if potential_delta < current_delta {
          closest_to_chroma = potential_solution.clone();
        }
        chroma_peak = f64::max(chroma_peak, potential_solution_chroma);
      }
    }
    answer
  }
}

impl Default for ColorSpec2021 {
  fn default() -> Self {
    Self::new()
  }
}

impl ColorSpec for ColorSpec2021 {
  fn primary_palette_key_color(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("primary_palette_key_color")
      .palette(|s| s.primary_palette())
      .tone(|s| s.primary_palette().key_color().tone())
      .build()
      .unwrap()
  }

  fn secondary_palette_key_color(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("secondary_palette_key_color")
      .palette(|s| s.secondary_palette())
      .tone(|s| s.secondary_palette().key_color().tone())
      .build()
      .unwrap()
  }

  fn tertiary_palette_key_color(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("tertiary_palette_key_color")
      .palette(|s| s.tertiary_palette())
      .tone(|s| s.tertiary_palette().key_color().tone())
      .build()
      .unwrap()
  }

  fn neutral_palette_key_color(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("neutral_palette_key_color")
      .palette(|s| s.neutral_palette())
      .tone(|s| s.neutral_palette().key_color().tone())
      .build()
      .unwrap()
  }

  fn neutral_variant_palette_key_color(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("neutral_variant_palette_key_color")
      .palette(|s| s.neutral_variant_palette())
      .tone(|s| s.neutral_variant_palette().key_color().tone())
      .build()
      .unwrap()
  }

  fn error_palette_key_color(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("error_palette_key_color")
      .palette(|s| s.error_palette())
      .tone(|s| s.error_palette().key_color().tone())
      .build()
      .unwrap()
  }

  fn background(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("background")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 6.0 } else { 98.0 })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn on_background(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_background")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 90.0 } else { 10.0 })
      .background(|_| self.background().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 3.0, 4.5, 7.0).into())
      .build()
      .unwrap()
  }

  fn surface(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 6.0 } else { 98.0 })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn surface_dim(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_dim")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          6.0
        } else {
          ContrastCurve::new(87.0, 87.0, 80.0, 75.0).get(s.contrast_level())
        }
      })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn surface_bright(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_bright")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          ContrastCurve::new(24.0, 24.0, 29.0, 34.0).get(s.contrast_level())
        } else {
          98.0
        }
      })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn surface_container_lowest(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_container_lowest")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          ContrastCurve::new(4.0, 4.0, 2.0, 0.0).get(s.contrast_level())
        } else {
          100.0
        }
      })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn surface_container_low(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_container_low")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          ContrastCurve::new(10.0, 10.0, 11.0, 12.0).get(s.contrast_level())
        } else {
          ContrastCurve::new(96.0, 96.0, 96.0, 95.0).get(s.contrast_level())
        }
      })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn surface_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_container")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          ContrastCurve::new(12.0, 12.0, 16.0, 20.0).get(s.contrast_level())
        } else {
          ContrastCurve::new(94.0, 94.0, 92.0, 90.0).get(s.contrast_level())
        }
      })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn surface_container_high(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_container_high")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          ContrastCurve::new(17.0, 17.0, 21.0, 25.0).get(s.contrast_level())
        } else {
          ContrastCurve::new(92.0, 92.0, 88.0, 85.0).get(s.contrast_level())
        }
      })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn surface_container_highest(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_container_highest")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          ContrastCurve::new(22.0, 22.0, 26.0, 30.0).get(s.contrast_level())
        } else {
          ContrastCurve::new(90.0, 90.0, 84.0, 80.0).get(s.contrast_level())
        }
      })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn on_surface(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_surface")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 90.0 } else { 10.0 })
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn surface_variant(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_variant")
      .palette(|s| s.neutral_variant_palette())
      .tone(|s| if s.is_dark() { 30.0 } else { 90.0 })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn on_surface_variant(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_surface_variant")
      .palette(|s| s.neutral_variant_palette())
      .tone(|s| if s.is_dark() { 80.0 } else { 30.0 })
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn inverse_surface(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("inverse_surface")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 90.0 } else { 20.0 })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn inverse_on_surface(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("inverse_on_surface")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 20.0 } else { 95.0 })
      .background(|_| self.inverse_surface().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn outline(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("outline")
      .palette(|s| s.neutral_variant_palette())
      .tone(|s| if s.is_dark() { 60.0 } else { 50.0 })
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.5, 3.0, 4.5, 7.0).into())
      .build()
      .unwrap()
  }

  fn outline_variant(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("outline_variant")
      .palette(|s| s.neutral_variant_palette())
      .tone(|s| if s.is_dark() { 30.0 } else { 80.0 })
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .build()
      .unwrap()
  }

  fn shadow(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("shadow")
      .palette(|s| s.neutral_palette())
      .tone(|_| 0.0)
      .build()
      .unwrap()
  }

  fn scrim(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("scrim")
      .palette(|s| s.neutral_palette())
      .tone(|_| 0.0)
      .build()
      .unwrap()
  }

  fn surface_tint(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("surface_tint")
      .palette(|s| s.primary_palette())
      .tone(|s| if s.is_dark() { 80.0 } else { 40.0 })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn primary(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("primary")
      .palette(|s| s.primary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 100.0 } else { 0.0 }
        } else if s.is_dark() {
          80.0
        } else {
          40.0
        }
      })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 7.0).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.primary_container(),
          self.primary(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn primary_dim(&self) -> Option<DynamicColor> {
    None
  }

  fn on_primary(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_primary")
      .palette(|s| s.primary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 10.0 } else { 90.0 }
        } else if s.is_dark() {
          20.0
        } else {
          100.0
        }
      })
      .background(|_| self.primary().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn primary_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("primary_container")
      .palette(|s| s.primary_palette())
      .tone(|s| {
        if Self::is_fidelity(s) {
          s.source_color_hct().tone()
        } else if Self::is_monochrome(s) {
          if s.is_dark() { 85.0 } else { 25.0 }
        } else if s.is_dark() {
          30.0
        } else {
          90.0
        }
      })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.primary_container(),
          self.primary(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn on_primary_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_primary_container")
      .palette(|s| s.primary_palette())
      .tone(|s| {
        if Self::is_fidelity(s) {
          DynamicColor::foreground_tone(self.primary_container().tone()(s), 4.5)
        } else if Self::is_monochrome(s) {
          if s.is_dark() { 0.0 } else { 100.0 }
        } else if s.is_dark() {
          90.0
        } else {
          30.0
        }
      })
      .background(|_| self.primary_container().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn inverse_primary(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("inverse_primary")
      .palette(|s| s.primary_palette())
      .tone(|s| if s.is_dark() { 40.0 } else { 80.0 })
      .background(|_| self.inverse_surface().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 7.0).into())
      .build()
      .unwrap()
  }

  fn secondary(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("secondary")
      .palette(|s| s.secondary_palette())
      .tone(|s| if s.is_dark() { 80.0 } else { 40.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 7.0).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.secondary_container(),
          self.secondary(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn secondary_dim(&self) -> Option<DynamicColor> {
    None
  }

  fn on_secondary(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_secondary")
      .palette(|s| s.secondary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 10.0 } else { 100.0 }
        } else if s.is_dark() {
          20.0
        } else {
          100.0
        }
      })
      .background(|_| self.secondary().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn secondary_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("secondary_container")
      .palette(|s| s.secondary_palette())
      .tone(|s| {
        let initial_tone = if s.is_dark() { 30.0 } else { 90.0 };
        if Self::is_monochrome(s) {
          if s.is_dark() { 30.0 } else { 85.0 }
        } else if !Self::is_fidelity(s) {
          initial_tone
        } else {
          Self::find_desired_chroma_by_tone(
            s.secondary_palette().hue(),
            s.secondary_palette().chroma(),
            initial_tone,
            !s.is_dark(),
          )
        }
      })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.secondary_container(),
          self.secondary(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn on_secondary_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_secondary_container")
      .palette(|s| s.secondary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 10.0 } else { 100.0 }
        } else if !Self::is_fidelity(s) {
          if s.is_dark() { 90.0 } else { 30.0 }
        } else {
          DynamicColor::foreground_tone(self.secondary_container().tone()(s), 4.5)
        }
      })
      .background(|_| self.secondary_container().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn tertiary(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("tertiary")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 90.0 } else { 25.0 }
        } else if s.is_dark() {
          80.0
        } else {
          40.0
        }
      })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 7.0).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.tertiary_container(),
          self.tertiary(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn tertiary_dim(&self) -> Option<DynamicColor> {
    None
  }

  fn on_tertiary(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_tertiary")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 10.0 } else { 100.0 }
        } else if s.is_dark() {
          20.0
        } else {
          100.0
        }
      })
      .background(|_| self.tertiary().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn tertiary_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("tertiary_container")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 60.0 } else { 40.0 }
        } else if !Self::is_fidelity(s) {
          if s.is_dark() { 30.0 } else { 90.0 }
        } else {
          let proposed_hct = s.tertiary_palette().hct(s.source_color_hct().tone());
          dislike_analyzer::fix_if_disliked(proposed_hct).tone()
        }
      })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.tertiary_container(),
          self.tertiary(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn on_tertiary_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_tertiary_container")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 0.0 } else { 100.0 }
        } else if !Self::is_fidelity(s) {
          if s.is_dark() { 90.0 } else { 30.0 }
        } else {
          DynamicColor::foreground_tone(self.tertiary_container().tone()(s), 4.5)
        }
      })
      .background(|_| self.tertiary_container().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn error(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("error")
      .palette(|s| s.error_palette())
      .tone(|s| if s.is_dark() { 80.0 } else { 40.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 7.0).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.error_container(),
          self.error(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn error_dim(&self) -> Option<DynamicColor> {
    None
  }

  fn on_error(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_error")
      .palette(|s| s.error_palette())
      .tone(|s| if s.is_dark() { 20.0 } else { 100.0 })
      .background(|_| self.error().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn error_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("error_container")
      .palette(|s| s.error_palette())
      .tone(|s| if s.is_dark() { 30.0 } else { 90.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.error_container(),
          self.error(),
          10.0,
          TonePolarity::Nearer,
          false,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn on_error_container(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_error_container")
      .palette(|s| s.error_palette())
      .tone(|s| {
        if Self::is_monochrome(s) {
          if s.is_dark() { 90.0 } else { 10.0 }
        } else if s.is_dark() {
          90.0
        } else {
          30.0
        }
      })
      .background(|_| self.error_container().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn primary_fixed(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("primary_fixed")
      .palette(|s| s.primary_palette())
      .tone(|s| if Self::is_monochrome(s) { 40.0 } else { 90.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.primary_fixed(),
          self.primary_fixed_dim(),
          10.0,
          TonePolarity::Ligher,
          true,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn primary_fixed_dim(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("primary_fixed_dim")
      .palette(|s| s.primary_palette())
      .tone(|s| if Self::is_monochrome(s) { 30.0 } else { 80.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.primary_fixed(),
          self.primary_fixed_dim(),
          10.0,
          TonePolarity::Ligher,
          true,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn on_primary_fixed(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_primary_fixed")
      .palette(|s| s.primary_palette())
      .tone(|s| if Self::is_monochrome(s) { 100.0 } else { 10.0 })
      .background(|_| self.primary_fixed_dim().into())
      .second_background(|_| self.primary_fixed().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn on_primary_fixed_variant(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_primary_fixed_variant")
      .palette(|s| s.primary_palette())
      .tone(|s| if Self::is_monochrome(s) { 90.0 } else { 30.0 })
      .background(|_| self.primary_fixed_dim().into())
      .second_background(|_| self.primary_fixed().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn secondary_fixed(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("secondary_fixed")
      .palette(|s| s.secondary_palette())
      .tone(|s| if Self::is_monochrome(s) { 80.0 } else { 90.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.secondary_fixed(),
          self.secondary_fixed_dim(),
          10.0,
          TonePolarity::Ligher,
          true,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn secondary_fixed_dim(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("secondary_fixed_dim")
      .palette(|s| s.secondary_palette())
      .tone(|s| if Self::is_monochrome(s) { 70.0 } else { 80.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.secondary_fixed(),
          self.secondary_fixed_dim(),
          10.0,
          TonePolarity::Ligher,
          true,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn on_secondary_fixed(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_secondary_fixed")
      .palette(|s| s.secondary_palette())
      .tone(|_| 10.0)
      .background(|_| self.secondary_fixed_dim().into())
      .second_background(|_| self.secondary_fixed().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn on_secondary_fixed_variant(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_secondary_fixed_variant")
      .palette(|s| s.secondary_palette())
      .tone(|s| if Self::is_monochrome(s) { 25.0 } else { 30.0 })
      .background(|_| self.secondary_fixed_dim().into())
      .second_background(|_| self.secondary_fixed().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn tertiary_fixed(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("tertiary_fixed")
      .palette(|s| s.tertiary_palette())
      .tone(|s| if Self::is_monochrome(s) { 40.0 } else { 90.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.tertiary_fixed(),
          self.tertiary_fixed_dim(),
          10.0,
          TonePolarity::Ligher,
          true,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn tertiary_fixed_dim(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("tertiary_fixed_dim")
      .palette(|s| s.tertiary_palette())
      .tone(|s| if Self::is_monochrome(s) { 30.0 } else { 80.0 })
      .is_background(true)
      .background(|s| self.highest_surface(s).into())
      .contrast_curve(|_| ContrastCurve::new(1.0, 1.0, 3.0, 4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_stay_together(
          self.tertiary_fixed(),
          self.tertiary_fixed_dim(),
          10.0,
          TonePolarity::Ligher,
          true,
        )
        .into()
      })
      .build()
      .unwrap()
  }

  fn on_tertiary_fixed(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_tertiary_fixed")
      .palette(|s| s.tertiary_palette())
      .tone(|s| if Self::is_monochrome(s) { 100.0 } else { 10.0 })
      .background(|_| self.tertiary_fixed_dim().into())
      .second_background(|_| self.tertiary_fixed().into())
      .contrast_curve(|_| ContrastCurve::new(4.5, 7.0, 11.0, 21.0).into())
      .build()
      .unwrap()
  }

  fn on_tertiary_fixed_variant(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("on_tertiary_fixed_variant")
      .palette(|s| s.tertiary_palette())
      .tone(|s| if Self::is_monochrome(s) { 90.0 } else { 30.0 })
      .background(|_| self.tertiary_fixed_dim().into())
      .second_background(|_| self.tertiary_fixed().into())
      .contrast_curve(|_| ContrastCurve::new(3.0, 4.5, 7.0, 11.0).into())
      .build()
      .unwrap()
  }

  fn control_activated(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("control_activated")
      .palette(|s| s.primary_palette())
      .tone(|s| if s.is_dark() { 30.0 } else { 90.0 })
      .is_background(true)
      .build()
      .unwrap()
  }

  fn control_normal(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("control_normal")
      .palette(|s| s.neutral_variant_palette())
      .tone(|s| if s.is_dark() { 80.0 } else { 30.0 })
      .build()
      .unwrap()
  }

  fn control_highlight(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("control_highlight")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 100.0 } else { 00.0 })
      .opacity(|s| if s.is_dark() { 0.20 } else { 0.12 }.into())
      .build()
      .unwrap()
  }

  fn text_primary_inverse(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("text_primary_inverse")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 10.0 } else { 90.0 })
      .build()
      .unwrap()
  }

  fn text_secondary_and_tertiary_inverse(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("text_secondary_and_tertiary_inverse")
      .palette(|s| s.neutral_variant_palette())
      .tone(|s| if s.is_dark() { 30.0 } else { 80.0 })
      .build()
      .unwrap()
  }

  fn text_primary_inverse_disable_only(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("text_primary_inverse_disable_only")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 10.0 } else { 90.0 })
      .build()
      .unwrap()
  }

  fn text_secondary_and_tertiary_inverse_disabled(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("text_secondary_and_tertiary_inverse_disabled")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 10.0 } else { 90.0 })
      .build()
      .unwrap()
  }

  fn text_hint_inverse(&self) -> DynamicColor {
    DynamicColorBuilder::new()
      .name("text_hint_inverse")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 10.0 } else { 90.0 })
      .build()
      .unwrap()
  }

  fn highest_surface(&self, s: &DynamicScheme) -> DynamicColor {
    if s.is_dark() {
      self.surface_bright()
    } else {
      self.surface_dim()
    }
  }
}
