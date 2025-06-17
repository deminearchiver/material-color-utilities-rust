use crate::{
  dynamiccolor::{DynamicScheme, PalettesSpec, PalettesSpec2021, Platform, Variant},
  hct::Hct,
  palettes::TonalPalette,
};

pub(crate) struct PalettesSpec2025 {
  spec_2021: PalettesSpec2021,
}

impl PalettesSpec2025 {
  pub const fn new() -> Self {
    Self {
      spec_2021: PalettesSpec2021::new(),
    }
  }

  fn expressive_neutral_hue(source_color_hct: &Hct) -> f64 {
    DynamicScheme::get_rotated_hue(
      source_color_hct,
      &[0.0, 71.0, 124.0, 253.0, 278.0, 300.0, 360.0],
      &[10.0, 0.0, 10.0, 0.0, 10.0, 0.0],
    )
  }

  fn expressive_neutral_chroma(source_color_hct: &Hct, is_dark: bool, platform: &Platform) -> f64 {
    let neutral_hue = Self::expressive_neutral_hue(source_color_hct);
    match *platform {
      Platform::Phone => {
        if is_dark {
          if Hct::is_yellow(neutral_hue) {
            6.0
          } else {
            14.0
          }
        } else {
          18.0
        }
      }
      Platform::Watch => 12.0,
    }
  }

  fn vibrant_neutral_hue(source_color_hct: &Hct) -> f64 {
    DynamicScheme::get_rotated_hue(
      source_color_hct,
      &[0.0, 38.0, 105.0, 140.0, 333.0, 360.0],
      &[-14.0, 10.0, -14.0, 10.0, -14.0],
    )
  }

  fn vibrant_neutral_chroma(source_color_hct: &Hct, platform: &Platform) -> f64 {
    let neutral_hue = Self::vibrant_neutral_hue(source_color_hct);
    match *platform {
      Platform::Phone => 28.0,
      Platform::Watch => {
        if Hct::is_blue(neutral_hue) {
          28.0
        } else {
          20.0
        }
      }
    }
  }
}

impl PalettesSpec for PalettesSpec2025 {
  fn primary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Neutral => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone if Hct::is_blue(source_color_hct.hue()) => 12.0,
          Platform::Phone => 8.0,
          Platform::Watch if Hct::is_blue(source_color_hct.hue()) => 16.0,
          Platform::Watch => 12.0,
        },
      ),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        if platform == &Platform::Phone && is_dark {
          26.0
        } else {
          32.0
        },
      ),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone if is_dark => 36.0,
          Platform::Phone => 48.0,
          Platform::Watch => 40.0,
        },
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone => 74.0,
          Platform::Watch => 56.0,
        },
      ),
      _ => {
        self
          .spec_2021
          .primary_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      }
    }
  }

  fn secondary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Neutral => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone if Hct::is_blue(source_color_hct.hue()) => 6.0,
          Platform::Phone => 4.0,
          Platform::Watch if Hct::is_blue(source_color_hct.hue()) => 10.0,
          Platform::Watch => 6.0,
        },
      ),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 16.0),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0],
          &[-160.0, 155.0, -100.0, 96.0, -96.0, -156.0, -165.0, -160.0],
        ),
        match *platform {
          Platform::Phone if is_dark => 16.0,
          Platform::Phone => 24.0,
          Platform::Watch => 24.0,
        },
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 38.0, 105.0, 140.0, 333.0, 360.0],
          &[-14.0, 10.0, -14.0, 10.0, -14.0],
        ),
        match *platform {
          Platform::Phone => 56.0,
          Platform::Watch => 36.0,
        },
      ),
      _ => self.spec_2021.secondary_palette(
        variant,
        source_color_hct,
        is_dark,
        platform,
        contrast_level,
      ),
    }
  }

  fn tertiary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Neutral => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 38.0, 105.0, 161.0, 204.0, 278.0, 333.0, 360.0],
          &[-32.0, 26.0, 10.0, -39.0, 24.0, -15.0, -32.0],
        ),
        match *platform {
          Platform::Phone => 20.0,
          Platform::Watch => 36.0,
        },
      ),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 20.0, 71.0, 161.0, 333.0, 360.0],
          &[-40.0, 48.0, -32.0, 40.0, -32.0],
        ),
        match *platform {
          Platform::Phone => 28.0,
          Platform::Watch => 32.0,
        },
      ),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 105.0, 140.0, 204.0, 253.0, 278.0, 300.0, 333.0, 360.0],
          &[-165.0, 160.0, -105.0, 101.0, -101.0, -160.0, -170.0, -165.0],
        ),
        48.0,
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 38.0, 71.0, 105.0, 140.0, 161.0, 253.0, 333.0, 360.0],
          &[-72.0, 35.0, 24.0, -24.0, 62.0, 50.0, 62.0, -72.0],
        ),
        56.0,
      ),
      _ => self.spec_2021.tertiary_palette(
        variant,
        source_color_hct,
        is_dark,
        platform,
        contrast_level,
      ),
    }
  }

  fn neutral_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Neutral => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone => 1.4,
          Platform::Watch => 6.0,
        },
      ),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone => 5.0,
          Platform::Watch => 10.0,
        },
      ),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        Self::expressive_neutral_hue(source_color_hct),
        Self::expressive_neutral_chroma(source_color_hct, is_dark, platform),
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(
        Self::vibrant_neutral_hue(source_color_hct),
        Self::vibrant_neutral_chroma(source_color_hct, platform),
      ),
      _ => {
        self
          .spec_2021
          .neutral_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      }
    }
  }

  fn neutral_variant_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Neutral => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone => 1.4,
          Platform::Watch => 6.0,
        } * 2.2,
      ),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        match *platform {
          Platform::Phone => 5.0,
          Platform::Watch => 10.0,
        } * 1.7,
      ),
      Variant::Expressive => {
        let expressive_neutral_hue = Self::expressive_neutral_hue(source_color_hct);
        let expressive_neutral_chroma =
          Self::expressive_neutral_chroma(source_color_hct, is_dark, platform);
        TonalPalette::from_hue_and_chroma(
          expressive_neutral_hue,
          expressive_neutral_chroma
            * (if (105.0..125.0).contains(&expressive_neutral_hue) {
              1.6
            } else {
              2.3
            }),
        )
      }
      Variant::Vibrant => {
        let vibrant_neutral_hue = Self::vibrant_neutral_hue(source_color_hct);
        let vibrant_neutral_chroma = Self::vibrant_neutral_chroma(source_color_hct, platform);
        TonalPalette::from_hue_and_chroma(vibrant_neutral_hue, vibrant_neutral_chroma * 1.29)
      }
      _ => self.spec_2021.neutral_variant_palette(
        variant,
        source_color_hct,
        is_dark,
        platform,
        contrast_level,
      ),
    }
  }

  fn error_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    platform: &Platform,
    contrast_level: f64,
  ) -> Option<TonalPalette> {
    let error_hue = DynamicScheme::get_piecewise_value(
      source_color_hct,
      &[0.0, 3.0, 13.0, 23.0, 33.0, 43.0, 153.0, 273.0, 360.0],
      &[12.0, 22.0, 32.0, 12.0, 22.0, 32.0, 22.0, 12.0],
    );
    match *variant {
      Variant::Neutral => Some(TonalPalette::from_hue_and_chroma(
        error_hue,
        match *platform {
          Platform::Phone => 50.0,
          Platform::Watch => 40.0,
        },
      )),
      Variant::TonalSpot => Some(TonalPalette::from_hue_and_chroma(
        error_hue,
        match *platform {
          Platform::Phone => 60.0,
          Platform::Watch => 48.0,
        },
      )),
      Variant::Expressive => Some(TonalPalette::from_hue_and_chroma(
        error_hue,
        match *platform {
          Platform::Phone => 64.0,
          Platform::Watch => 48.0,
        },
      )),
      Variant::Vibrant => Some(TonalPalette::from_hue_and_chroma(
        error_hue,
        match *platform {
          Platform::Phone => 80.0,
          Platform::Watch => 60.0,
        },
      )),
      _ => {
        self
          .spec_2021
          .error_palette(variant, source_color_hct, is_dark, platform, contrast_level)
      }
    }
  }
}
