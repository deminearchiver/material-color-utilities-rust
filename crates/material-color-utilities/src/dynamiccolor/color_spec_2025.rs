use crate::{
  dynamiccolor::{
    ColorSpec, ColorSpec2021, ContrastCurve, DeltaConstraint, DynamicColor, DynamicColorBuilder,
    DynamicScheme, DynamicSchemeBuilder, Platform, SpecVersion, ToneDeltaPair, TonePolarity,
    Variant,
  },
  hct::Hct,
  palettes::TonalPalette,
};

const SPEC_2021: ColorSpec2021 = ColorSpec2021::new();

pub(crate) struct ColorSpec2025;

impl ColorSpec2025 {
  pub const fn new() -> Self {
    Self {}
  }

  fn find_best_tone_for_chroma(
    hue: f64,
    chroma: f64,
    mut tone: f64,
    by_decreasing_tone: bool,
  ) -> f64 {
    let mut answer = tone;
    let mut best_candidate = Hct::from(hue, chroma, answer);
    while best_candidate.chroma() < chroma {
      if !(0.0..=100.0).contains(&tone) {
        break;
      }
      tone += if by_decreasing_tone { -1.0 } else { 1.0 };
      let new_candidate = Hct::from(hue, chroma, tone);
      if best_candidate.chroma() < new_candidate.chroma() {
        best_candidate = new_candidate;
        answer = tone;
      }
    }
    answer
  }

  fn t_max_c(palette: &TonalPalette) -> f64 {
    Self::t_max_c_clamped(palette, 0.0, 100.0)
  }

  fn t_max_c_clamped(palette: &TonalPalette, lower_bound: f64, upper_bound: f64) -> f64 {
    Self::t_max_c_with_chroma_multiplier(palette, lower_bound, upper_bound, 1.0)
  }

  fn t_max_c_with_chroma_multiplier(
    palette: &TonalPalette,
    lower_bound: f64,
    upper_bound: f64,
    chroma_multiplier: f64,
  ) -> f64 {
    let answer = Self::find_best_tone_for_chroma(
      palette.hue(),
      palette.chroma() * chroma_multiplier,
      100.0,
      true,
    );
    answer.clamp(lower_bound, upper_bound)
  }

  fn t_min_c(palette: &TonalPalette) -> f64 {
    Self::t_min_c_clamped(palette, 0.0, 100.0)
  }

  fn t_min_c_clamped(palette: &TonalPalette, lower_bound: f64, upper_bound: f64) -> f64 {
    let answer = Self::find_best_tone_for_chroma(palette.hue(), palette.chroma(), 0.0, false);
    answer.clamp(lower_bound, upper_bound)
  }

  const fn get_contrast_curve(default_contrast: f64) -> ContrastCurve {
    match default_contrast {
      1.5 => ContrastCurve::new(1.5, 1.5, 3.0, 4.5),
      3.0 => ContrastCurve::new(3.0, 3.0, 4.5, 7.0),
      4.5 => ContrastCurve::new(4.5, 4.5, 7.0, 11.0),
      6.0 => ContrastCurve::new(6.0, 6.0, 7.0, 11.0),
      7.0 => ContrastCurve::new(7.0, 7.0, 11.0, 21.0),
      9.0 => ContrastCurve::new(9.0, 9.0, 11.0, 21.0),
      11.0 => ContrastCurve::new(11.0, 11.0, 21.0, 21.0),
      21.0 => ContrastCurve::new(21.0, 21.0, 21.0, 21.0),
      // Shouldn't happen.
      _ => ContrastCurve::new(default_contrast, default_contrast, 7.0, 21.0),
    }
  }
}

impl Default for ColorSpec2025 {
  fn default() -> Self {
    Self::new()
  }
}

impl ColorSpec for ColorSpec2025 {
  ////////////////////////////////////////////////////////////////
  // Main Palettes                                              //
  ////////////////////////////////////////////////////////////////

  fn primary_palette_key_color(&self) -> DynamicColor {
    SPEC_2021.primary_palette_key_color()
  }

  fn secondary_palette_key_color(&self) -> DynamicColor {
    SPEC_2021.secondary_palette_key_color()
  }

  fn tertiary_palette_key_color(&self) -> DynamicColor {
    SPEC_2021.tertiary_palette_key_color()
  }

  fn neutral_palette_key_color(&self) -> DynamicColor {
    SPEC_2021.neutral_palette_key_color()
  }

  fn neutral_variant_palette_key_color(&self) -> DynamicColor {
    SPEC_2021.neutral_variant_palette_key_color()
  }

  fn error_palette_key_color(&self) -> DynamicColor {
    SPEC_2021.error_palette_key_color()
  }

  ////////////////////////////////////////////////////////////////
  // Surfaces [S]                                               //
  ////////////////////////////////////////////////////////////////

  fn background(&self) -> DynamicColor {
    // Remapped to surface for 2025 spec.
    let color2025 = DynamicColorBuilder::from(self.surface())
      .name("background")
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.background())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_background(&self) -> DynamicColor {
    // Remapped to on_surface for 2025 spec.
    let color2025 = DynamicColorBuilder::from(self.on_surface())
      .name("on_background")
      .tone(|s| {
        if s.platform() == &Platform::Watch {
          100.0
        } else {
          self.on_surface().get_tone(s)
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_background())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            4.0
          } else if Hct::is_yellow(s.neutral_palette().hue()) {
            99.0
          } else if s.variant() == &Variant::Vibrant {
            97.0
          } else {
            98.0
          }
        } else {
          0.0
        }
      })
      .is_background(true)
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_dim(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface_dim")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          4.0
        } else if Hct::is_yellow(s.neutral_palette().hue()) {
          90.0
        } else if s.variant() == &Variant::Vibrant {
          85.0
        } else {
          87.0
        }
      })
      .is_background(true)
      .chroma_multiplier(|s| {
        if !s.is_dark() {
          if s.variant() == &Variant::Neutral {
            return 2.5;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.7;
          } else if s.variant() == &Variant::Expressive {
            if Hct::is_yellow(s.neutral_palette().hue()) {
              return 2.7;
            } else {
              return 1.75;
            }
          } else if s.variant() == &Variant::Vibrant {
            return 1.36;
          }
        }
        1.0
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_dim())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_bright(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface_bright")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          18.0
        } else if Hct::is_yellow(s.neutral_palette().hue()) {
          99.0
        } else if s.variant() == &Variant::Vibrant {
          97.0
        } else {
          98.0
        }
      })
      .is_background(true)
      .chroma_multiplier(|s| {
        if s.is_dark() {
          if s.variant() == &Variant::Neutral {
            return 2.5;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.7;
          } else if s.variant() == &Variant::Expressive {
            if Hct::is_yellow(s.neutral_palette().hue()) {
              return 2.7;
            } else {
              return 1.75;
            }
          } else if s.variant() == &Variant::Vibrant {
            return 1.36;
          }
        }
        1.0
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_bright())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_container_lowest(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface_container_lowest")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 0.0 } else { 100.0 })
      .is_background(true)
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_container_lowest())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_container_low(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface_container_low")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            6.0
          } else if Hct::is_yellow(s.neutral_palette().hue()) {
            98.0
          } else if s.variant() == &Variant::Vibrant {
            95.0
          } else {
            96.0
          }
        } else {
          15.0
        }
      })
      .is_background(true)
      .chroma_multiplier(|s| {
        if s.platform() == &Platform::Phone {
          if s.variant() == &Variant::Neutral {
            return 1.3;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.25;
          } else if s.variant() == &Variant::Expressive {
            return if Hct::is_yellow(s.neutral_palette().hue()) {
              1.3
            } else {
              1.15
            };
          } else if s.variant() == &Variant::Vibrant {
            return 1.08;
          }
        }
        1.0
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_container_low())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface_container")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            9.0
          } else if Hct::is_yellow(s.neutral_palette().hue()) {
            96.0
          } else if s.variant() == &Variant::Vibrant {
            92.0
          } else {
            94.0
          }
        } else {
          20.0
        }
      })
      .is_background(true)
      .chroma_multiplier(|s| {
        if s.platform() == &Platform::Phone {
          if s.variant() == &Variant::Neutral {
            return 1.6;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.4;
          } else if s.variant() == &Variant::Expressive {
            return if Hct::is_yellow(s.neutral_palette().hue()) {
              1.6
            } else {
              1.3
            };
          } else if s.variant() == &Variant::Vibrant {
            return 1.15;
          }
        }
        1.0
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_container_high(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface_container_high")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            12.0
          } else if Hct::is_yellow(s.neutral_palette().hue()) {
            94.0
          } else if s.variant() == &Variant::Vibrant {
            90.0
          } else {
            92.0
          }
        } else {
          25.0
        }
      })
      .is_background(true)
      .chroma_multiplier(|s| {
        if s.platform() == &Platform::Phone {
          if s.variant() == &Variant::Neutral {
            return 1.9;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.5;
          } else if s.variant() == &Variant::Expressive {
            return if Hct::is_yellow(s.neutral_palette().hue()) {
              1.95
            } else {
              1.45
            };
          } else if s.variant() == &Variant::Vibrant {
            return 1.22;
          }
        }
        1.0
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_container_high())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_container_highest(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("surface_container_highest")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.is_dark() {
          15.0
        } else if Hct::is_yellow(s.neutral_palette().hue()) {
          92.0
        } else if s.variant() == &Variant::Vibrant {
          88.0
        } else {
          90.0
        }
      })
      .is_background(true)
      .chroma_multiplier(|s| {
        if s.variant() == &Variant::Neutral {
          return 2.2;
        } else if s.variant() == &Variant::TonalSpot {
          return 1.7;
        } else if s.variant() == &Variant::Expressive {
          return if Hct::is_yellow(s.neutral_palette().hue()) {
            2.3
          } else {
            1.6
          };
        } else if s.variant() == &Variant::Vibrant {
          return 1.29;
        }
        1.0
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_container_highest())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_surface(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_surface")
      .palette(|s| s.neutral_palette())
      .tone(|s| {
        if s.variant() == &Variant::Vibrant {
          Self::t_max_c_with_chroma_multiplier(s.neutral_palette(), 0.0, 100.0, 1.1)
        } else {
          DynamicColor::get_initial_tone_from_background(Some(|s: &DynamicScheme| {
            Some(if s.platform() == &Platform::Phone {
              if s.is_dark() {
                self.surface_bright()
              } else {
                self.surface_dim()
              }
            } else {
              self.surface_container_high()
            })
          }))(s)
        }
      })
      .chroma_multiplier(|s| {
        if s.platform() == &Platform::Phone {
          if s.variant() == &Variant::Neutral {
            return 2.2;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.7;
          } else if s.variant() == &Variant::Expressive {
            return if Hct::is_yellow(s.neutral_palette().hue()) {
              if s.is_dark() { 3.0 } else { 2.3 }
            } else {
              1.6
            };
          }
        }
        1.0
      })
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
        } else {
          self.surface_container_high()
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.is_dark() && s.platform() == &Platform::Phone {
          Self::get_contrast_curve(11.0)
        } else {
          Self::get_contrast_curve(9.0)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_surface())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn surface_variant(&self) -> DynamicColor {
    // Remapped to surface_container_highest for 2025 spec.
    let color2025 = DynamicColorBuilder::from(self.surface_container_highest())
      .name("surface_variant")
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_variant())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_surface_variant(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_surface_variant")
      .palette(|s| s.neutral_palette())
      .chroma_multiplier(|s| {
        if s.platform() == &Platform::Phone {
          if s.variant() == &Variant::Neutral {
            return 2.2;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.7;
          } else if s.variant() == &Variant::Expressive {
            return if Hct::is_yellow(s.neutral_palette().hue()) {
              if s.is_dark() { 3.0 } else { 2.3 }
            } else {
              1.6
            };
          }
        }
        1.0
      })
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
        } else {
          self.surface_container_high()
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            Self::get_contrast_curve(6.0)
          } else {
            Self::get_contrast_curve(4.5)
          }
        } else {
          Self::get_contrast_curve(7.0)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_surface_variant())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn inverse_surface(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("inverse_surface")
      .palette(|s| s.neutral_palette())
      .tone(|s| if s.is_dark() { 98.0 } else { 4.0 })
      .is_background(true)
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.inverse_surface())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn inverse_on_surface(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("inverse_on_surface")
      .palette(|s| s.neutral_palette())
      .background(|_| self.inverse_surface().into())
      .contrast_curve(|_| Self::get_contrast_curve(7.0).into())
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.inverse_on_surface())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn outline(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("outline")
      .palette(|s| s.neutral_palette())
      .chroma_multiplier(|s| {
        if s.platform() == &Platform::Phone {
          if s.variant() == &Variant::Neutral {
            return 2.2;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.7;
          } else if s.variant() == &Variant::Expressive {
            return if Hct::is_yellow(s.neutral_palette().hue()) {
              if s.is_dark() { 3.0 } else { 2.3 }
            } else {
              1.6
            };
          }
        }
        1.0
      })
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
        } else {
          self.surface_container_high()
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(3.0)
        } else {
          Self::get_contrast_curve(4.5)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.outline())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn outline_variant(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("outline_variant")
      .palette(|s| s.neutral_palette())
      .chroma_multiplier(|s| {
        if s.platform() == &Platform::Phone {
          if s.variant() == &Variant::Neutral {
            return 2.2;
          } else if s.variant() == &Variant::TonalSpot {
            return 1.7;
          } else if s.variant() == &Variant::Expressive {
            return if Hct::is_yellow(s.neutral_palette().hue()) {
              if s.is_dark() { 3.0 } else { 2.3 }
            } else {
              1.6
            };
          }
        }
        1.0
      })
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
        } else {
          self.surface_container_high()
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(1.5)
        } else {
          Self::get_contrast_curve(3.0)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.outline_variant())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn shadow(&self) -> DynamicColor {
    SPEC_2021.shadow()
  }

  fn scrim(&self) -> DynamicColor {
    SPEC_2021.scrim()
  }

  fn surface_tint(&self) -> DynamicColor {
    // Remapped to primary for 2025 spec.
    let color2025 = DynamicColorBuilder::from(self.primary())
      .name("surface_tint")
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.surface_tint())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Primaries [P]                                              //
  ////////////////////////////////////////////////////////////////

  fn primary(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("primary")
      .palette(|s| s.primary_palette())
      .tone(|s| match *s.variant() {
        Variant::Neutral => match *s.platform() {
          Platform::Phone => {
            if s.is_dark() {
              80.0
            } else {
              40.0
            }
          }
          Platform::Watch => 90.0,
        },
        Variant::TonalSpot => match *s.platform() {
          Platform::Phone => {
            if s.is_dark() {
              80.0
            } else {
              Self::t_max_c(s.primary_palette())
            }
          }
          Platform::Watch => Self::t_max_c_clamped(s.primary_palette(), 0.0, 90.0),
        },
        Variant::Expressive => Self::t_max_c_clamped(
          s.primary_palette(),
          0.0,
          if Hct::is_yellow(s.primary_palette().hue()) {
            25.0
          } else if Hct::is_cyan(s.primary_palette().hue()) {
            88.0
          } else {
            98.0
          },
        ),
        _ => Self::t_max_c_clamped(
          s.primary_palette(),
          0.0,
          if Hct::is_cyan(s.primary_palette().hue()) {
            88.0
          } else {
            98.0
          },
        ),
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
        } else {
          self.surface_container_high()
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        match *s.platform() {
          Platform::Phone => Self::get_contrast_curve(4.5),
          Platform::Watch => Self::get_contrast_curve(7.0),
        }
        .into()
      })
      .tone_delta_pair(|s| match *s.platform() {
        Platform::Phone => Some(ToneDeltaPair::with_constraint(
          self.primary_container(),
          self.primary(),
          5.0,
          super::TonePolarity::RelativeLighter,
          DeltaConstraint::Farther,
        )),
        Platform::Watch => None,
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.primary())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn primary_dim(&self) -> Option<DynamicColor> {
    let color2025 = DynamicColorBuilder::new()
      .name("primary_dim")
      .palette(|s| s.primary_palette())
      .tone(|s| {
        if s.variant() == &Variant::Neutral {
          85.0
        } else if s.variant() == &Variant::TonalSpot {
          Self::t_max_c_clamped(s.primary_palette(), 0.0, 90.0)
        } else {
          Self::t_max_c(s.primary_palette())
        }
      })
      .is_background(true)
      .background(|_| self.surface_container_high().into())
      .contrast_curve(|_: &DynamicScheme| Self::get_contrast_curve(4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_constraint(
          self.primary_dim().unwrap(),
          self.primary(),
          5.0,
          super::TonePolarity::Darker,
          DeltaConstraint::Farther,
        )
        .into()
      })
      .build()
      .unwrap();
    Some(color2025)
  }

  fn on_primary(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_primary")
      .palette(|s| s.primary_palette())
      .background(|s| {
        match *s.platform() {
          Platform::Phone => self.primary(),
          Platform::Watch => self.primary_dim().unwrap(),
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        match *s.platform() {
          Platform::Phone => Self::get_contrast_curve(6.0),
          Platform::Watch => Self::get_contrast_curve(7.0),
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_primary())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn primary_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("primary_container")
      .palette(|s| s.primary_palette())
      .tone(|s| {
        if s.platform() == &Platform::Watch {
          30.0
        } else if s.variant() == &Variant::Neutral {
          if s.is_dark() { 30.0 } else { 90.0 }
        } else if s.variant() == &Variant::TonalSpot {
          if s.is_dark() {
            Self::t_min_c_clamped(s.primary_palette(), 35.0, 93.0)
          } else {
            Self::t_max_c_clamped(s.primary_palette(), 0.0, 90.0)
          }
        } else if s.variant() == &Variant::Expressive {
          if s.is_dark() {
            Self::t_max_c_clamped(s.primary_palette(), 30.0, 93.0)
          } else {
            Self::t_max_c_clamped(
              s.primary_palette(),
              78.0,
              if Hct::is_cyan(s.primary_palette().hue()) {
                88.0
              } else {
                90.0
              },
            )
          }
        } else if s.is_dark() {
          Self::t_min_c_clamped(s.primary_palette(), 66.0, 93.0)
        } else {
          Self::t_max_c_clamped(
            s.primary_palette(),
            66.0,
            if Hct::is_cyan(s.primary_palette().hue()) {
              88.0
            } else {
              93.0
            },
          )
        }
      })
      .is_background(true)
      .background(|s| match *s.platform() {
        Platform::Phone => Some(if s.is_dark() {
          self.surface_bright()
        } else {
          self.surface_dim()
        }),
        Platform::Watch => None,
      })
      .tone_delta_pair(|s| match *s.platform() {
        Platform::Phone => None,
        Platform::Watch => Some(ToneDeltaPair::with_constraint(
          self.primary_container(),
          self.primary_dim().unwrap(),
          10.0,
          TonePolarity::Darker,
          DeltaConstraint::Farther,
        )),
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone && s.contrast_level() > 0.0 {
          Some(Self::get_contrast_curve(1.5))
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.primary_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_primary_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_primary_container")
      .palette(|s| s.primary_palette())
      .background(|_| self.primary_container().into())
      .contrast_curve(|s: &DynamicScheme| {
        match *s.platform() {
          Platform::Phone => Self::get_contrast_curve(6.0),
          Platform::Watch => Self::get_contrast_curve(7.0),
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_primary_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn inverse_primary(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("inverse_primary")
      .palette(|s| s.primary_palette())
      .tone(|s| Self::t_max_c(s.primary_palette()))
      .background(|_| self.inverse_surface().into())
      .contrast_curve(|s: &DynamicScheme| {
        match *s.platform() {
          Platform::Phone => Self::get_contrast_curve(6.0),
          Platform::Watch => Self::get_contrast_curve(7.0),
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.inverse_primary())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Secondaries [Q]                                            //
  ////////////////////////////////////////////////////////////////

  fn secondary(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("secondary")
      .palette(|s| s.secondary_palette())
      .tone(|s| {
        if s.platform() == &Platform::Watch {
          if s.variant() == &Variant::Neutral {
            90.0
          } else {
            Self::t_max_c_clamped(s.secondary_palette(), 0.0, 90.0)
          }
        } else if s.variant() == &Variant::Neutral {
          if s.is_dark() {
            Self::t_min_c_clamped(s.secondary_palette(), 0.0, 98.0)
          } else {
            Self::t_max_c(s.secondary_palette())
          }
        } else if s.is_dark() {
          80.0
        } else {
          Self::t_max_c(s.secondary_palette())
        }
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
        } else {
          self.surface_container_high()
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        match *s.platform() {
          Platform::Phone => Self::get_contrast_curve(4.5),
          Platform::Watch => Self::get_contrast_curve(7.0),
        }
        .into()
      })
      .tone_delta_pair(|s| match *s.platform() {
        Platform::Phone => Some(ToneDeltaPair::with_constraint(
          self.secondary_container(),
          self.secondary(),
          5.0,
          TonePolarity::RelativeLighter,
          DeltaConstraint::Farther,
        )),
        Platform::Watch => None,
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.secondary())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn secondary_dim(&self) -> Option<DynamicColor> {
    let color2025 = DynamicColorBuilder::new()
      .name("secondary_dim")
      .palette(|s| s.secondary_palette())
      .tone(|s| {
        if s.variant() == &Variant::Neutral {
          85.0
        } else {
          Self::t_max_c_clamped(s.secondary_palette(), 0.0, 90.0)
        }
      })
      .is_background(true)
      .background(|_| self.surface_container_high().into())
      .contrast_curve(|_| Self::get_contrast_curve(4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_constraint(
          self.secondary_dim().unwrap(),
          self.secondary(),
          5.0,
          TonePolarity::Darker,
          DeltaConstraint::Farther,
        )
        .into()
      })
      .build()
      .unwrap();
    Some(color2025)
  }

  fn on_secondary(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_secondary")
      .palette(|s| s.secondary_palette())
      .background(|s| match *s.platform() {
        Platform::Phone => self.secondary().into(),
        Platform::Watch => self.secondary_dim(),
      })
      .contrast_curve(|s: &DynamicScheme| {
        match *s.platform() {
          Platform::Phone => Self::get_contrast_curve(6.0),
          Platform::Watch => Self::get_contrast_curve(7.0),
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_secondary())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn secondary_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("secondary_container")
      .palette(|s| s.secondary_palette())
      .tone(|s| {
        if s.platform() == &Platform::Watch {
          30.0
        } else if s.variant() == &Variant::Vibrant {
          if s.is_dark() {
            Self::t_min_c_clamped(s.secondary_palette(), 30.0, 40.0)
          } else {
            Self::t_max_c_clamped(s.secondary_palette(), 84.0, 90.0)
          }
        } else if s.variant() == &Variant::Expressive {
          if s.is_dark() {
            15.0
          } else {
            Self::t_max_c_clamped(s.secondary_palette(), 90.0, 95.0)
          }
        } else if s.is_dark() {
          25.0
        } else {
          90.0
        }
      })
      .is_background(true)
      .background(|s| match *s.platform() {
        Platform::Phone => Some(if s.is_dark() {
          self.surface_bright()
        } else {
          self.surface_dim()
        }),
        Platform::Watch => None,
      })
      .tone_delta_pair(|s| match *s.platform() {
        Platform::Phone => None,
        Platform::Watch => Some(ToneDeltaPair::with_constraint(
          self.secondary_container(),
          self.secondary_dim().unwrap(),
          10.0,
          TonePolarity::Darker,
          DeltaConstraint::Farther,
        )),
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone && s.contrast_level() > 0.0 {
          Some(Self::get_contrast_curve(1.5))
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.secondary_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_secondary_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_secondary_container")
      .palette(|s| s.secondary_palette())
      .background(|_| self.secondary_container().into())
      .contrast_curve(|s: &DynamicScheme| {
        match *s.platform() {
          Platform::Phone => Self::get_contrast_curve(6.0),
          Platform::Watch => Self::get_contrast_curve(7.0),
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_secondary_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Tertiaries [T]                                             //
  ////////////////////////////////////////////////////////////////

  fn tertiary(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("tertiary")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        if s.platform() == &Platform::Watch {
          if s.variant() == &Variant::TonalSpot {
            Self::t_max_c_clamped(s.tertiary_palette(), 0.0, 90.0)
          } else {
            Self::t_max_c(s.tertiary_palette())
          }
        } else if s.variant() == &Variant::Expressive || s.variant() == &Variant::Vibrant {
          Self::t_max_c_clamped(
            s.tertiary_palette(),
            0.0,
            if Hct::is_cyan(s.tertiary_palette().hue()) {
              88.0
            } else if s.is_dark() {
              98.0
            } else {
              100.0
            },
          )
        } else if s.is_dark() {
          Self::t_max_c_clamped(s.tertiary_palette(), 0.0, 98.0)
        } else {
          Self::t_max_c(s.tertiary_palette())
        }
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
        } else {
          self.surface_container_high()
        }
        .into()
      })
      .contrast_curve(|s| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(4.5)
        } else {
          Self::get_contrast_curve(7.0)
        }
        .into()
      })
      .tone_delta_pair(|s| {
        if s.platform() == &Platform::Phone {
          ToneDeltaPair::with_constraint(
            self.tertiary_container(),
            self.tertiary(),
            5.0,
            TonePolarity::RelativeLighter,
            DeltaConstraint::Farther,
          )
          .into()
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.tertiary())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn tertiary_dim(&self) -> Option<DynamicColor> {
    let color2025 = DynamicColorBuilder::new()
      .name("tertiary_dim")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        if s.variant() == &Variant::TonalSpot {
          Self::t_max_c_clamped(s.tertiary_palette(), 0.0, 90.0)
        } else {
          Self::t_max_c(s.tertiary_palette())
        }
      })
      .is_background(true)
      .background(|_| self.surface_container_high().into())
      .contrast_curve(|_| Self::get_contrast_curve(4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_constraint(
          self.tertiary_dim().unwrap(),
          self.tertiary(),
          5.0,
          TonePolarity::Darker,
          DeltaConstraint::Farther,
        )
        .into()
      })
      .build()
      .unwrap();
    Some(color2025)
  }

  fn on_tertiary(&self) -> DynamicColor {
    let color2025: DynamicColor<'_> = DynamicColorBuilder::new()
      .name("on_tertiary")
      .palette(|s| s.tertiary_palette())
      .background(|s| {
        if s.platform() == &Platform::Phone {
          self.tertiary().into()
        } else {
          self.tertiary_dim()
        }
      })
      .contrast_curve(|s| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(6.0)
        } else {
          Self::get_contrast_curve(7.0)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_tertiary())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn tertiary_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("tertiary_container")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        if s.platform() == &Platform::Watch {
          if s.variant() == &Variant::TonalSpot {
            Self::t_max_c_clamped(s.tertiary_palette(), 0.0, 90.0)
          } else {
            Self::t_max_c(s.tertiary_palette())
          }
        } else if s.variant() == &Variant::Neutral {
          if s.is_dark() {
            Self::t_max_c_clamped(s.tertiary_palette(), 0.0, 93.0)
          } else {
            Self::t_max_c_clamped(s.tertiary_palette(), 0.0, 96.0)
          }
        } else if s.variant() == &Variant::TonalSpot {
          Self::t_max_c_clamped(
            s.tertiary_palette(),
            0.0,
            if s.is_dark() { 93.0 } else { 100.0 },
          )
        } else if s.variant() == &Variant::Expressive {
          Self::t_max_c_clamped(
            s.tertiary_palette(),
            75.0,
            if Hct::is_cyan(s.tertiary_palette().hue()) {
              88.0
            } else if s.is_dark() {
              93.0
            } else {
              100.0
            },
          )
        } else if s.is_dark() {
          Self::t_max_c_clamped(s.tertiary_palette(), 0.0, 93.0)
        } else {
          Self::t_max_c_clamped(s.tertiary_palette(), 72.0, 100.0)
        }
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
          .into()
        } else {
          None
        }
      })
      .tone_delta_pair(|s| {
        if s.platform() == &Platform::Watch {
          ToneDeltaPair::with_constraint(
            self.tertiary_container(),
            self.tertiary_dim().unwrap(),
            10.0,
            TonePolarity::Darker,
            DeltaConstraint::Farther,
          )
          .into()
        } else {
          None
        }
      })
      .contrast_curve(|s| {
        if s.platform() == &Platform::Phone && s.contrast_level() > 0.0 {
          Self::get_contrast_curve(1.5).into()
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.tertiary_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_tertiary_container(&self) -> DynamicColor {
    let color2025: DynamicColor<'_> = DynamicColorBuilder::new()
      .name("on_tertiary_container")
      .palette(|s| s.tertiary_palette())
      .background(|_| self.tertiary_container().into())
      .contrast_curve(|s| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(6.0)
        } else {
          Self::get_contrast_curve(7.0)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_tertiary_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Errors [E]                                                 //
  ////////////////////////////////////////////////////////////////

  fn error(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("error")
      .palette(|s| s.error_palette())
      .tone(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            Self::t_min_c_clamped(s.error_palette(), 0.0, 98.0)
          } else {
            Self::t_max_c(s.error_palette())
          }
        } else {
          Self::t_min_c(s.error_palette())
        }
      })
      .is_background(true)
      .background(|s| {
        {
          if s.platform() == &Platform::Phone {
            if s.is_dark() {
              self.surface_bright()
            } else {
              self.surface_dim()
            }
          } else {
            self.surface_container_high()
          }
        }
        .into()
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(4.5)
        } else {
          Self::get_contrast_curve(7.0)
        }
        .into()
      })
      .tone_delta_pair(|s| {
        if s.platform() == &Platform::Phone {
          Some(ToneDeltaPair::with_constraint(
            self.error_container(),
            self.error(),
            5.0,
            TonePolarity::RelativeLighter,
            DeltaConstraint::Farther,
          ))
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.error())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn error_dim(&self) -> Option<DynamicColor> {
    let color2025 = DynamicColorBuilder::new()
      .name("error_dim")
      .palette(|s| s.error_palette())
      .tone(|s| Self::t_min_c(s.error_palette()))
      .is_background(true)
      .background(|_| self.surface_container_high().into())
      .contrast_curve(|_| Self::get_contrast_curve(4.5).into())
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_constraint(
          self.error_dim().unwrap(),
          self.error(),
          5.0,
          TonePolarity::Darker,
          DeltaConstraint::Farther,
        )
        .into()
      })
      .build()
      .unwrap();
    Some(color2025)
  }

  fn on_error(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_error")
      .palette(|s| s.error_palette())
      .background(|s| {
        if s.platform() == &Platform::Phone {
          self.error().into()
        } else {
          self.error_dim()
        }
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(6.0)
        } else {
          Self::get_contrast_curve(7.0)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_error())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn error_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("error_container")
      .palette(|s| s.error_palette())
      .tone(|s| {
        if s.platform() == &Platform::Watch {
          30.0
        } else if s.is_dark() {
          Self::t_min_c_clamped(s.error_palette(), 30.0, 93.0)
        } else {
          Self::t_max_c_clamped(s.error_palette(), 0.0, 90.0)
        }
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
          .into()
        } else {
          None
        }
      })
      .tone_delta_pair(|s| {
        if s.platform() == &Platform::Watch {
          Some(ToneDeltaPair::with_constraint(
            self.error_container(),
            self.error_dim().unwrap(),
            10.0,
            TonePolarity::Darker,
            DeltaConstraint::Farther,
          ))
        } else {
          None
        }
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone && s.contrast_level() > 0.0 {
          Self::get_contrast_curve(1.5).into()
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.error_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_error_container(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_error_container")
      .palette(|s| s.error_palette())
      .background(|_| self.error_container().into())
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone {
          Self::get_contrast_curve(4.5)
        } else {
          Self::get_contrast_curve(7.0)
        }
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_error_container())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Primary Fixed Colors [PF]                                  //
  ////////////////////////////////////////////////////////////////

  fn primary_fixed(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("primary_fixed")
      .palette(|s| s.primary_palette())
      .tone(|s| {
        let temp_s = DynamicSchemeBuilder::from(s)
          .is_dark(false)
          .contrast_level(0.0)
          .build();
        self.primary_container().get_tone(&temp_s)
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
          .into()
        } else {
          None
        }
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone && s.contrast_level() > 0.0 {
          Self::get_contrast_curve(1.5).into()
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.primary_fixed())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn primary_fixed_dim(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("primary_fixed_dim")
      .palette(|s| s.primary_palette())
      .tone(|s| self.primary_fixed().get_tone(s))
      .is_background(true)
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_constraint(
          self.primary_fixed_dim(),
          self.primary_fixed(),
          5.0,
          TonePolarity::Darker,
          DeltaConstraint::Exact,
        )
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.primary_fixed_dim())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_primary_fixed(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_primary_fixed")
      .palette(|s| s.primary_palette())
      .background(|_| self.primary_fixed_dim().into())
      .contrast_curve(|_| Self::get_contrast_curve(7.0).into())
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_primary_fixed())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_primary_fixed_variant(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_primary_fixed_variant")
      .palette(|s| s.primary_palette())
      .background(|_| self.primary_fixed_dim().into())
      .contrast_curve(|_| Self::get_contrast_curve(4.5).into())
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_primary_fixed_variant())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Secondary Fixed Colors [QF]                                //
  ////////////////////////////////////////////////////////////////

  fn secondary_fixed(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("secondary_fixed")
      .palette(|s| s.secondary_palette())
      .tone(|s| {
        let temp_s = DynamicSchemeBuilder::from(s)
          .is_dark(false)
          .contrast_level(0.0)
          .build();
        self.secondary_container().get_tone(&temp_s)
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
          .into()
        } else {
          None
        }
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone && s.contrast_level() > 0.0 {
          Self::get_contrast_curve(1.5).into()
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.secondary_fixed())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn secondary_fixed_dim(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("secondary_fixed_dim")
      .palette(|s| s.secondary_palette())
      .tone(|s| self.secondary_fixed().get_tone(s))
      .is_background(true)
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_constraint(
          self.secondary_fixed_dim(),
          self.secondary_fixed(),
          5.0,
          TonePolarity::Darker,
          DeltaConstraint::Exact,
        )
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.secondary_fixed_dim())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_secondary_fixed(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_secondary_fixed")
      .palette(|s| s.secondary_palette())
      .background(|_| self.secondary_fixed_dim().into())
      .contrast_curve(|_| Self::get_contrast_curve(7.0).into())
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_secondary_fixed())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_secondary_fixed_variant(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_secondary_fixed_variant")
      .palette(|s| s.secondary_palette())
      .background(|_| self.secondary_fixed_dim().into())
      .contrast_curve(|_| Self::get_contrast_curve(4.5).into())
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_secondary_fixed_variant())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Tertiary Fixed Colors [TF]                                 //
  ////////////////////////////////////////////////////////////////

  fn tertiary_fixed(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("tertiary_fixed")
      .palette(|s| s.tertiary_palette())
      .tone(|s| {
        let temp_s = DynamicSchemeBuilder::from(s)
          .is_dark(false)
          .contrast_level(0.0)
          .build();
        self.tertiary_container().get_tone(&temp_s)
      })
      .is_background(true)
      .background(|s| {
        if s.platform() == &Platform::Phone {
          if s.is_dark() {
            self.surface_bright()
          } else {
            self.surface_dim()
          }
          .into()
        } else {
          None
        }
      })
      .contrast_curve(|s: &DynamicScheme| {
        if s.platform() == &Platform::Phone && s.contrast_level() > 0.0 {
          Self::get_contrast_curve(1.5).into()
        } else {
          None
        }
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.tertiary_fixed())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn tertiary_fixed_dim(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("tertiary_fixed_dim")
      .palette(|s| s.tertiary_palette())
      .tone(|s| self.tertiary_fixed().get_tone(s))
      .is_background(true)
      .tone_delta_pair(|_| {
        ToneDeltaPair::with_constraint(
          self.tertiary_fixed_dim(),
          self.tertiary_fixed(),
          5.0,
          TonePolarity::Darker,
          DeltaConstraint::Exact,
        )
        .into()
      })
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.tertiary_fixed_dim())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_tertiary_fixed(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_tertiary_fixed")
      .palette(|s| s.tertiary_palette())
      .background(|_| self.tertiary_fixed_dim().into())
      .contrast_curve(|_| Self::get_contrast_curve(7.0).into())
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_tertiary_fixed())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  fn on_tertiary_fixed_variant(&self) -> DynamicColor {
    let color2025 = DynamicColorBuilder::new()
      .name("on_tertiary_fixed_variant")
      .palette(|s| s.tertiary_palette())
      .background(|_| self.tertiary_fixed_dim().into())
      .contrast_curve(|_| Self::get_contrast_curve(4.5).into())
      .build()
      .unwrap();
    DynamicColorBuilder::from(SPEC_2021.on_tertiary_fixed_variant())
      .extend_spec_version(SpecVersion::Spec2025, color2025)
      .unwrap()
      .build()
      .unwrap()
  }

  ////////////////////////////////////////////////////////////////
  // Other                                                      //
  ////////////////////////////////////////////////////////////////

  fn highest_surface(&self, s: &DynamicScheme) -> DynamicColor {
    SPEC_2021.highest_surface(s)
  }
}
