use crate::{
  dislike_analyzer,
  dynamiccolor::{DynamicScheme, PalettesSpec, Platform, Variant},
  hct::Hct,
  palettes::TonalPalette,
  temperature_cache::TemperatureCache,
  utils,
};

pub(crate) struct PalettesSpec2021;

impl PalettesSpec2021 {
  pub const fn new() -> Self {
    Self
  }
}

impl PalettesSpec for PalettesSpec2021 {
  fn primary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    _is_dark: bool,
    _platform: &Platform,
    _contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Content | Variant::Fidelity => {
        TonalPalette::from_hue_and_chroma(source_color_hct.hue(), source_color_hct.chroma())
      }
      Variant::FruitSalad => TonalPalette::from_hue_and_chroma(
        utils::math::sanitize_degrees(source_color_hct.hue() - 50.0),
        48.0,
      ),
      Variant::Monochrome => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 0.0),
      Variant::Neutral => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 12.0),
      Variant::Rainbow => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 48.0),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 36.0),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        utils::math::sanitize_degrees(source_color_hct.hue() + 240.0),
        40.0,
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 200.0),
    }
  }

  fn secondary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    _is_dark: bool,
    _platform: &Platform,
    _contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Content | Variant::Fidelity => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        f64::max(
          source_color_hct.chroma() - 32.0,
          source_color_hct.chroma() * 0.5,
        ),
      ),
      Variant::FruitSalad => TonalPalette::from_hue_and_chroma(
        utils::math::sanitize_degrees(source_color_hct.hue() - 50.0),
        36.0,
      ),
      Variant::Monochrome => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 0.0),
      Variant::Neutral => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 8.0),
      Variant::Rainbow => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 16.0),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 16.0),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0],
          &[45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0],
        ),
        24.0,
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0],
          &[18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0],
        ),
        24.0,
      ),
    }
  }

  fn tertiary_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    _is_dark: bool,
    _platform: &Platform,
    _contrast_level: f64,
  ) -> TonalPalette {
    let mut temperature_cache = TemperatureCache::new(source_color_hct.clone());
    match *variant {
      Variant::Content => TonalPalette::from_hct(dislike_analyzer::fix_if_disliked(
        temperature_cache.analogous_with(3, 6)[2].clone(),
      )),
      Variant::Fidelity => TonalPalette::from_hct(dislike_analyzer::fix_if_disliked(
        temperature_cache.complement().clone(),
      )),
      Variant::FruitSalad => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 36.0),
      Variant::Monochrome => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 0.0),
      Variant::Neutral => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 16.0),
      Variant::Rainbow | Variant::TonalSpot => TonalPalette::from_hue_and_chroma(
        utils::math::sanitize_degrees(source_color_hct.hue() + 60.0),
        24.0,
      ),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0],
          &[120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0],
        ),
        32.0,
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(
        DynamicScheme::get_rotated_hue(
          source_color_hct,
          &[0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0],
          &[35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0],
        ),
        32.0,
      ),
    }
  }

  fn neutral_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    _is_dark: bool,
    _platform: &Platform,
    _contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Content | Variant::Fidelity => {
        TonalPalette::from_hue_and_chroma(source_color_hct.hue(), source_color_hct.chroma() / 8.0)
      }
      Variant::FruitSalad => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 10.0),
      Variant::Monochrome => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 0.0),
      Variant::Neutral => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 2.0),
      Variant::Rainbow => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 0.0),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 6.0),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        utils::math::sanitize_degrees(source_color_hct.hue() + 15.0),
        8.0,
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 10.0),
    }
  }

  fn neutral_variant_palette(
    &self,
    variant: &Variant,
    source_color_hct: &Hct,
    _is_dark: bool,
    _platform: &Platform,
    _contrast_level: f64,
  ) -> TonalPalette {
    match *variant {
      Variant::Content | Variant::Fidelity => TonalPalette::from_hue_and_chroma(
        source_color_hct.hue(),
        source_color_hct.chroma() / 8.0 + 4.0,
      ),
      Variant::FruitSalad => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 16.0),
      Variant::Monochrome => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 0.0),
      Variant::Neutral => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 2.0),
      Variant::Rainbow => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 0.0),
      Variant::TonalSpot => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 8.0),
      Variant::Expressive => TonalPalette::from_hue_and_chroma(
        utils::math::sanitize_degrees(source_color_hct.hue() + 15.0),
        12.0,
      ),
      Variant::Vibrant => TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 12.0),
    }
  }

  fn error_palette(
    &self,
    _variant: &Variant,
    _source_color_hct: &Hct,
    _is_dark: bool,
    _platform: &Platform,
    _contrast_level: f64,
  ) -> Option<TonalPalette> {
    None
  }
}
