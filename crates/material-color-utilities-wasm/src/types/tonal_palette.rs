use material_color_utilities::{
  dynamiccolor::{DynamicColor, DynamicScheme, MaterialDynamicColors},
  hct::Hct,
  palettes::TonalPalette,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::{
  DynamicSchemeOptions, DynamicSchemePlatform, DynamicSchemeSpecVersion, DynamicSchemeVariant,
};

#[derive(Deserialize, Serialize, Tsify)]
#[serde(untagged, rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum TonalPaletteOptions {
  Argb { argb: u32 },
  HueAndChroma { hue: f64, chroma: f64 },
}

impl From<TonalPaletteOptions> for TonalPalette {
  fn from(options: TonalPaletteOptions) -> Self {
    match options {
      TonalPaletteOptions::Argb { argb } => Self::from_int(argb),
      TonalPaletteOptions::HueAndChroma { hue, chroma } => Self::from_hue_and_chroma(hue, chroma),
    }
  }
}

#[wasm_bindgen(js_name = "Hct")]
pub struct JsHct {
  inner: Hct,
}

#[wasm_bindgen(js_class = "Hct")]
impl JsHct {
  #[wasm_bindgen(js_name = "from")]
  pub fn from(hue: f64, chroma: f64, tone: f64) -> Self {
    From::from(Hct::from(hue, chroma, tone))
  }

  #[wasm_bindgen(js_name = "fromInt")]
  pub fn from_int(argb: u32) -> Self {
    From::from(Hct::from_int(argb))
  }

  #[wasm_bindgen(getter, js_name = "hue")]
  pub fn hue(&self) -> f64 {
    self.inner.hue()
  }

  #[wasm_bindgen(getter, js_name = "chroma")]
  pub fn chroma(&self) -> f64 {
    self.inner.chroma()
  }

  #[wasm_bindgen(getter, js_name = "tone")]
  pub fn tone(&self) -> f64 {
    self.inner.tone()
  }
}

impl JsHct {
  pub fn into_inner(self) -> Hct {
    self.inner
  }
}

impl From<Hct> for JsHct {
  fn from(value: Hct) -> Self {
    Self { inner: value }
  }
}

// impl From<JsHct> for Hct {
//   fn from(value: JsHct) -> Self {
//     value.inner
//   }
// }

#[wasm_bindgen(js_name = "TonalPalette")]
pub struct JsTonalPalette {
  inner: TonalPalette,
}

#[wasm_bindgen(js_class = "TonalPalette")]
impl JsTonalPalette {
  #[wasm_bindgen(js_name = "fromInt")]
  pub fn from_int(argb: u32) -> Self {
    Self::from(TonalPalette::from_int(argb))
  }

  #[wasm_bindgen(js_name = "fromHct")]
  pub fn from_hct(hct: JsHct) -> Self {
    Self::from(TonalPalette::from_hct(hct.into_inner()))
  }

  #[wasm_bindgen(js_name = "fromHueAndChroma")]
  pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
    Self::from(TonalPalette::from_hue_and_chroma(hue, chroma))
  }

  #[wasm_bindgen(getter, js_name = "hue")]
  pub fn hue(&self) -> f64 {
    self.inner.hue()
  }

  #[wasm_bindgen(getter, js_name = "chroma")]
  pub fn chroma(&self) -> f64 {
    self.inner.chroma()
  }

  #[wasm_bindgen(getter, js_name = "keyColor")]
  pub fn key_color(&self) -> JsHct {
    self.inner.key_color().clone().into()
  }

  #[wasm_bindgen(js_name = "tone")]
  pub fn tone(&self, tone: u8) -> u32 {
    self.inner.tone(tone)
  }

  #[wasm_bindgen(js_name = "getHct")]
  pub fn get_hct(&self, tone: f64) -> JsHct {
    self.inner.hct(tone).into()
  }
}

impl JsTonalPalette {
  pub fn into_inner(self) -> TonalPalette {
    self.inner
  }
}

impl From<TonalPalette> for JsTonalPalette {
  fn from(value: TonalPalette) -> Self {
    Self { inner: value }
  }
}

#[wasm_bindgen(js_name = "DynamicScheme")]
pub struct JsDynamicScheme {
  inner: DynamicScheme,
}

#[wasm_bindgen(js_class = "DynamicScheme")]
impl JsDynamicScheme {
  #[wasm_bindgen(constructor)]
  pub fn new(options: DynamicSchemeOptions) -> Self {
    Self::from(DynamicScheme::from(options))
  }

  #[wasm_bindgen(getter, js_name = "sourceColorHct")]
  pub fn source_color_hct(&self) -> JsHct {
    self.inner.source_color_hct().clone().into()
  }

  #[wasm_bindgen(getter, js_name = "variant")]
  pub fn variant(&self) -> DynamicSchemeVariant {
    (*self.inner.variant()).into()
  }

  #[wasm_bindgen(getter, js_name = "isDark")]
  pub fn is_dark(&self) -> bool {
    self.inner.is_dark()
  }

  #[wasm_bindgen(getter, js_name = "platform")]
  pub fn platform(&self) -> DynamicSchemePlatform {
    (*self.inner.platform()).into()
  }

  #[wasm_bindgen(getter, js_name = "contrastLevel")]
  pub fn contrast_level(&self) -> f64 {
    self.inner.contrast_level()
  }

  #[wasm_bindgen(getter, js_name = "specVersion")]
  pub fn spec_version(&self) -> DynamicSchemeSpecVersion {
    (*self.inner.spec_version()).into()
  }

  #[wasm_bindgen(getter, js_name = "sourceColorArgb")]
  pub fn source_color_argb(&self) -> u32 {
    self.inner.source_color_argb()
  }

  #[wasm_bindgen(getter, js_name = "primaryPalette")]
  pub fn primary_palette(&self) -> JsTonalPalette {
    self.inner.primary_palette().clone().into()
  }

  #[wasm_bindgen(getter, js_name = "secondaryPalette")]
  pub fn secondary_palette(&self) -> JsTonalPalette {
    self.inner.secondary_palette().clone().into()
  }

  #[wasm_bindgen(getter, js_name = "tertiaryPalette")]
  pub fn tertiary_palette(&self) -> JsTonalPalette {
    self.inner.tertiary_palette().clone().into()
  }

  #[wasm_bindgen(getter, js_name = "neutralPalette")]
  pub fn neutral_palette(&self) -> JsTonalPalette {
    self.inner.neutral_palette().clone().into()
  }

  #[wasm_bindgen(getter, js_name = "neutralVariantPalette")]
  pub fn neutral_variant_palette(&self) -> JsTonalPalette {
    self.inner.neutral_variant_palette().clone().into()
  }

  #[wasm_bindgen(getter, js_name = "errorPalette")]
  pub fn error_palette(&self) -> JsTonalPalette {
    self.inner.error_palette().clone().into()
  }
}

impl JsDynamicScheme {
  pub fn into_inner(self) -> DynamicScheme {
    self.inner
  }
}

impl From<DynamicScheme> for JsDynamicScheme {
  fn from(value: DynamicScheme) -> Self {
    Self { inner: value }
  }
}

#[wasm_bindgen(js_name = "DynamicColor")]
pub struct JsDynamicColor {
  inner: Box<DynamicColor<'static>>,
}

#[wasm_bindgen(js_class = "DynamicColor")]
impl JsDynamicColor {
  // #[wasm_bindgen]
  // pub fn get_tone(&self, scheme: &JsDynamicScheme) -> f64 {}
}

#[derive(Default)]
#[wasm_bindgen(js_name = "MaterialDynamicColors")]
pub struct JsMaterialDynamicColors;

const COLORS: MaterialDynamicColors = MaterialDynamicColors::new();

#[wasm_bindgen(js_class = "MaterialDynamicColors")]
impl JsMaterialDynamicColors {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self
  }

  #[wasm_bindgen(getter, js_name = "primary")]
  pub fn primary(self) -> JsDynamicColor {
    JsDynamicColor {
      inner: Box::new(COLORS.primary()),
    }
  }
}
