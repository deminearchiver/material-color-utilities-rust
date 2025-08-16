mod tonal_palette;

pub use tonal_palette::*;

use material_color_utilities::{
  dynamiccolor::{
    DynamicScheme, DynamicSchemeBuilder, MaterialDynamicColors, Platform, SpecVersion, Variant,
  },
  hct::Hct,
  utils::string::css_hex_from_argb,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Tsify,
)]
#[serde(rename = "Variant", rename_all = "kebab-case")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum DynamicSchemeVariant {
  Monochrome,
  Neutral,
  TonalSpot,
  Vibrant,
  Expressive,
  Fidelity,
  Content,
  Rainbow,
  FruitSalad,
}

impl From<Variant> for DynamicSchemeVariant {
  fn from(value: Variant) -> Self {
    match value {
      Variant::Monochrome => Self::Monochrome,
      Variant::Neutral => Self::Neutral,
      Variant::TonalSpot => Self::TonalSpot,
      Variant::Vibrant => Self::Vibrant,
      Variant::Expressive => Self::Expressive,
      Variant::Fidelity => Self::Fidelity,
      Variant::Content => Self::Content,
      Variant::Rainbow => Self::Rainbow,
      Variant::FruitSalad => Self::FruitSalad,
    }
  }
}

impl From<DynamicSchemeVariant> for Variant {
  fn from(value: DynamicSchemeVariant) -> Self {
    match value {
      DynamicSchemeVariant::Monochrome => Self::Monochrome,
      DynamicSchemeVariant::Neutral => Self::Neutral,
      DynamicSchemeVariant::TonalSpot => Self::TonalSpot,
      DynamicSchemeVariant::Vibrant => Self::Vibrant,
      DynamicSchemeVariant::Expressive => Self::Expressive,
      DynamicSchemeVariant::Fidelity => Self::Fidelity,
      DynamicSchemeVariant::Content => Self::Content,
      DynamicSchemeVariant::Rainbow => Self::Rainbow,
      DynamicSchemeVariant::FruitSalad => Self::FruitSalad,
    }
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Tsify,
)]
#[serde(rename = "Platform", rename_all = "kebab-case")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum DynamicSchemePlatform {
  Phone,
  Watch,
}

impl From<Platform> for DynamicSchemePlatform {
  fn from(value: Platform) -> Self {
    match value {
      Platform::Phone => Self::Phone,
      Platform::Watch => Self::Watch,
    }
  }
}

impl From<DynamicSchemePlatform> for Platform {
  fn from(value: DynamicSchemePlatform) -> Self {
    match value {
      DynamicSchemePlatform::Phone => Self::Phone,
      DynamicSchemePlatform::Watch => Self::Watch,
    }
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Tsify,
)]
#[serde(rename = "SpecVersion", rename_all = "kebab-case")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum DynamicSchemeSpecVersion {
  #[serde(rename = "2021")]
  Spec2021,
  #[serde(rename = "2025")]
  Spec2025,
}

impl From<SpecVersion> for DynamicSchemeSpecVersion {
  fn from(value: SpecVersion) -> Self {
    match value {
      SpecVersion::Spec2021 => Self::Spec2021,
      SpecVersion::Spec2025 => Self::Spec2025,
    }
  }
}
impl From<DynamicSchemeSpecVersion> for SpecVersion {
  fn from(value: DynamicSchemeSpecVersion) -> Self {
    match value {
      DynamicSchemeSpecVersion::Spec2021 => Self::Spec2021,
      DynamicSchemeSpecVersion::Spec2025 => Self::Spec2025,
    }
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DynamicSchemeOptions {
  #[tsify(optional)]
  is_dark: Option<bool>,
  #[tsify(optional)]
  variant: Option<DynamicSchemeVariant>,
  #[tsify(optional)]
  platform: Option<DynamicSchemePlatform>,
  #[tsify(optional)]
  spec_version: Option<DynamicSchemeSpecVersion>,
}

impl From<DynamicSchemeOptions> for DynamicSchemeBuilder {
  fn from(options: DynamicSchemeOptions) -> Self {
    let mut builder = DynamicSchemeBuilder::default();
    if let Some(is_dark) = options.is_dark {
      builder = builder.is_dark(is_dark);
    }
    if let Some(variant) = options.variant {
      builder = builder.variant(variant.into());
    }
    if let Some(platform) = options.platform {
      builder = builder.platform(platform.into());
    }
    if let Some(spec_version) = options.spec_version {
      builder = builder.spec_version(spec_version.into());
    }
    builder
  }
}

impl From<DynamicSchemeOptions> for DynamicScheme {
  fn from(options: DynamicSchemeOptions) -> Self {
    DynamicSchemeBuilder::from(options).build()
  }
}

#[derive(Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DynamicColors<T> {
  primary_palette_key_color: T,
  secondary_palette_key_color: T,
  tertiary_palette_key_color: T,
  neutral_palette_key_color: T,
  neutral_variant_palette_key_color: T,
  error_palette_key_color: T,
  background: T,
  on_background: T,
  surface: T,
  surface_dim: T,
  surface_bright: T,
  surface_container_lowest: T,
  surface_container_low: T,
  surface_container: T,
  surface_container_high: T,
  surface_container_highest: T,
  on_surface: T,
  surface_variant: T,
  on_surface_variant: T,
  inverse_surface: T,
  inverse_on_surface: T,
  outline: T,
  outline_variant: T,
  shadow: T,
  scrim: T,
  surface_tint: T,
  primary: T,
  primary_dim: T,
  on_primary: T,
  primary_container: T,
  on_primary_container: T,
  inverse_primary: T,
  primary_fixed: T,
  primary_fixed_dim: T,
  on_primary_fixed: T,
  on_primary_fixed_variant: T,
  secondary: T,
  secondary_dim: T,
  on_secondary: T,
  secondary_container: T,
  on_secondary_container: T,
  secondary_fixed: T,
  secondary_fixed_dim: T,
  on_secondary_fixed: T,
  on_secondary_fixed_variant: T,
  tertiary: T,
  tertiary_dim: T,
  on_tertiary: T,
  tertiary_container: T,
  on_tertiary_container: T,
  tertiary_fixed: T,
  tertiary_fixed_dim: T,
  on_tertiary_fixed: T,
  on_tertiary_fixed_variant: T,
  error: T,
  error_dim: T,
  on_error: T,
  error_container: T,
  on_error_container: T,
  control_activated: T,
  control_normal: T,
  control_highlight: T,
  text_primary_inverse: T,
  text_secondary_and_tertiary_inverse: T,
  text_primary_inverse_disable_only: T,
  text_secondary_and_tertiary_inverse_disabled: T,
  text_hint_inverse: T,
}

impl From<&DynamicScheme> for DynamicColors<Hct> {
  fn from(scheme: &DynamicScheme) -> Self {
    const DYNAMIC_COLORS: MaterialDynamicColors = MaterialDynamicColors::new();
    Self {
      primary_palette_key_color: DYNAMIC_COLORS.primary_palette_key_color().get_hct(scheme),
      secondary_palette_key_color: DYNAMIC_COLORS.secondary_palette_key_color().get_hct(scheme),
      tertiary_palette_key_color: DYNAMIC_COLORS.tertiary_palette_key_color().get_hct(scheme),
      neutral_palette_key_color: DYNAMIC_COLORS.neutral_palette_key_color().get_hct(scheme),
      neutral_variant_palette_key_color: DYNAMIC_COLORS
        .neutral_variant_palette_key_color()
        .get_hct(scheme),
      error_palette_key_color: DYNAMIC_COLORS.error_palette_key_color().get_hct(scheme),
      background: DYNAMIC_COLORS.background().get_hct(scheme),
      on_background: DYNAMIC_COLORS.on_background().get_hct(scheme),
      surface: DYNAMIC_COLORS.surface().get_hct(scheme),
      surface_dim: DYNAMIC_COLORS.surface_dim().get_hct(scheme),
      surface_bright: DYNAMIC_COLORS.surface_bright().get_hct(scheme),
      surface_container_lowest: DYNAMIC_COLORS.surface_container_lowest().get_hct(scheme),
      surface_container_low: DYNAMIC_COLORS.surface_container_low().get_hct(scheme),
      surface_container: DYNAMIC_COLORS.surface_container().get_hct(scheme),
      surface_container_high: DYNAMIC_COLORS.surface_container_high().get_hct(scheme),
      surface_container_highest: DYNAMIC_COLORS.surface_container_highest().get_hct(scheme),
      on_surface: DYNAMIC_COLORS.on_surface().get_hct(scheme),
      surface_variant: DYNAMIC_COLORS.surface_variant().get_hct(scheme),
      on_surface_variant: DYNAMIC_COLORS.on_surface_variant().get_hct(scheme),
      inverse_surface: DYNAMIC_COLORS.inverse_surface().get_hct(scheme),
      inverse_on_surface: DYNAMIC_COLORS.inverse_on_surface().get_hct(scheme),
      outline: DYNAMIC_COLORS.outline().get_hct(scheme),
      outline_variant: DYNAMIC_COLORS.outline_variant().get_hct(scheme),
      shadow: DYNAMIC_COLORS.shadow().get_hct(scheme),
      scrim: DYNAMIC_COLORS.scrim().get_hct(scheme),
      surface_tint: DYNAMIC_COLORS.surface_tint().get_hct(scheme),
      primary: DYNAMIC_COLORS.primary().get_hct(scheme),
      primary_dim: DYNAMIC_COLORS.primary_dim().get_hct(scheme),
      on_primary: DYNAMIC_COLORS.on_primary().get_hct(scheme),
      primary_container: DYNAMIC_COLORS.primary_container().get_hct(scheme),
      on_primary_container: DYNAMIC_COLORS.on_primary_container().get_hct(scheme),
      inverse_primary: DYNAMIC_COLORS.inverse_primary().get_hct(scheme),
      primary_fixed: DYNAMIC_COLORS.primary_fixed().get_hct(scheme),
      primary_fixed_dim: DYNAMIC_COLORS.primary_fixed_dim().get_hct(scheme),
      on_primary_fixed: DYNAMIC_COLORS.on_primary_fixed().get_hct(scheme),
      on_primary_fixed_variant: DYNAMIC_COLORS.on_primary_fixed_variant().get_hct(scheme),
      secondary: DYNAMIC_COLORS.secondary().get_hct(scheme),
      secondary_dim: DYNAMIC_COLORS.secondary_dim().get_hct(scheme),
      on_secondary: DYNAMIC_COLORS.on_secondary().get_hct(scheme),
      secondary_container: DYNAMIC_COLORS.secondary_container().get_hct(scheme),
      on_secondary_container: DYNAMIC_COLORS.on_secondary_container().get_hct(scheme),
      secondary_fixed: DYNAMIC_COLORS.secondary_fixed().get_hct(scheme),
      secondary_fixed_dim: DYNAMIC_COLORS.secondary_fixed_dim().get_hct(scheme),
      on_secondary_fixed: DYNAMIC_COLORS.on_secondary_fixed().get_hct(scheme),
      on_secondary_fixed_variant: DYNAMIC_COLORS.on_secondary_fixed_variant().get_hct(scheme),
      tertiary: DYNAMIC_COLORS.tertiary().get_hct(scheme),
      tertiary_dim: DYNAMIC_COLORS.tertiary_dim().get_hct(scheme),
      on_tertiary: DYNAMIC_COLORS.on_tertiary().get_hct(scheme),
      tertiary_container: DYNAMIC_COLORS.tertiary_container().get_hct(scheme),
      on_tertiary_container: DYNAMIC_COLORS.on_tertiary_container().get_hct(scheme),
      tertiary_fixed: DYNAMIC_COLORS.tertiary_fixed().get_hct(scheme),
      tertiary_fixed_dim: DYNAMIC_COLORS.tertiary_fixed_dim().get_hct(scheme),
      on_tertiary_fixed: DYNAMIC_COLORS.on_tertiary_fixed().get_hct(scheme),
      on_tertiary_fixed_variant: DYNAMIC_COLORS.on_tertiary_fixed_variant().get_hct(scheme),
      error: DYNAMIC_COLORS.error().get_hct(scheme),
      error_dim: DYNAMIC_COLORS.error_dim().get_hct(scheme),
      on_error: DYNAMIC_COLORS.on_error().get_hct(scheme),
      error_container: DYNAMIC_COLORS.error_container().get_hct(scheme),
      on_error_container: DYNAMIC_COLORS.on_error_container().get_hct(scheme),
      control_activated: DYNAMIC_COLORS.control_activated().get_hct(scheme),
      control_normal: DYNAMIC_COLORS.control_normal().get_hct(scheme),
      control_highlight: DYNAMIC_COLORS.control_highlight().get_hct(scheme),
      text_primary_inverse: DYNAMIC_COLORS.text_primary_inverse().get_hct(scheme),
      text_secondary_and_tertiary_inverse: DYNAMIC_COLORS
        .text_secondary_and_tertiary_inverse()
        .get_hct(scheme),
      text_primary_inverse_disable_only: DYNAMIC_COLORS
        .text_primary_inverse_disable_only()
        .get_hct(scheme),
      text_secondary_and_tertiary_inverse_disabled: DYNAMIC_COLORS
        .text_secondary_and_tertiary_inverse_disabled()
        .get_hct(scheme),
      text_hint_inverse: DYNAMIC_COLORS.text_hint_inverse().get_hct(scheme),
    }
  }
}

impl From<&DynamicScheme> for DynamicColors<u32> {
  fn from(scheme: &DynamicScheme) -> Self {
    const DYNAMIC_COLORS: MaterialDynamicColors = MaterialDynamicColors::new();
    Self {
      primary_palette_key_color: DYNAMIC_COLORS.primary_palette_key_color().get_argb(scheme),
      secondary_palette_key_color: DYNAMIC_COLORS
        .secondary_palette_key_color()
        .get_argb(scheme),
      tertiary_palette_key_color: DYNAMIC_COLORS.tertiary_palette_key_color().get_argb(scheme),
      neutral_palette_key_color: DYNAMIC_COLORS.neutral_palette_key_color().get_argb(scheme),
      neutral_variant_palette_key_color: DYNAMIC_COLORS
        .neutral_variant_palette_key_color()
        .get_argb(scheme),
      error_palette_key_color: DYNAMIC_COLORS.error_palette_key_color().get_argb(scheme),
      background: DYNAMIC_COLORS.background().get_argb(scheme),
      on_background: DYNAMIC_COLORS.on_background().get_argb(scheme),
      surface: DYNAMIC_COLORS.surface().get_argb(scheme),
      surface_dim: DYNAMIC_COLORS.surface_dim().get_argb(scheme),
      surface_bright: DYNAMIC_COLORS.surface_bright().get_argb(scheme),
      surface_container_lowest: DYNAMIC_COLORS.surface_container_lowest().get_argb(scheme),
      surface_container_low: DYNAMIC_COLORS.surface_container_low().get_argb(scheme),
      surface_container: DYNAMIC_COLORS.surface_container().get_argb(scheme),
      surface_container_high: DYNAMIC_COLORS.surface_container_high().get_argb(scheme),
      surface_container_highest: DYNAMIC_COLORS.surface_container_highest().get_argb(scheme),
      on_surface: DYNAMIC_COLORS.on_surface().get_argb(scheme),
      surface_variant: DYNAMIC_COLORS.surface_variant().get_argb(scheme),
      on_surface_variant: DYNAMIC_COLORS.on_surface_variant().get_argb(scheme),
      inverse_surface: DYNAMIC_COLORS.inverse_surface().get_argb(scheme),
      inverse_on_surface: DYNAMIC_COLORS.inverse_on_surface().get_argb(scheme),
      outline: DYNAMIC_COLORS.outline().get_argb(scheme),
      outline_variant: DYNAMIC_COLORS.outline_variant().get_argb(scheme),
      shadow: DYNAMIC_COLORS.shadow().get_argb(scheme),
      scrim: DYNAMIC_COLORS.scrim().get_argb(scheme),
      surface_tint: DYNAMIC_COLORS.surface_tint().get_argb(scheme),
      primary: DYNAMIC_COLORS.primary().get_argb(scheme),
      primary_dim: DYNAMIC_COLORS.primary_dim().get_argb(scheme),
      on_primary: DYNAMIC_COLORS.on_primary().get_argb(scheme),
      primary_container: DYNAMIC_COLORS.primary_container().get_argb(scheme),
      on_primary_container: DYNAMIC_COLORS.on_primary_container().get_argb(scheme),
      inverse_primary: DYNAMIC_COLORS.inverse_primary().get_argb(scheme),
      primary_fixed: DYNAMIC_COLORS.primary_fixed().get_argb(scheme),
      primary_fixed_dim: DYNAMIC_COLORS.primary_fixed_dim().get_argb(scheme),
      on_primary_fixed: DYNAMIC_COLORS.on_primary_fixed().get_argb(scheme),
      on_primary_fixed_variant: DYNAMIC_COLORS.on_primary_fixed_variant().get_argb(scheme),
      secondary: DYNAMIC_COLORS.secondary().get_argb(scheme),
      secondary_dim: DYNAMIC_COLORS.secondary_dim().get_argb(scheme),
      on_secondary: DYNAMIC_COLORS.on_secondary().get_argb(scheme),
      secondary_container: DYNAMIC_COLORS.secondary_container().get_argb(scheme),
      on_secondary_container: DYNAMIC_COLORS.on_secondary_container().get_argb(scheme),
      secondary_fixed: DYNAMIC_COLORS.secondary_fixed().get_argb(scheme),
      secondary_fixed_dim: DYNAMIC_COLORS.secondary_fixed_dim().get_argb(scheme),
      on_secondary_fixed: DYNAMIC_COLORS.on_secondary_fixed().get_argb(scheme),
      on_secondary_fixed_variant: DYNAMIC_COLORS.on_secondary_fixed_variant().get_argb(scheme),
      tertiary: DYNAMIC_COLORS.tertiary().get_argb(scheme),
      tertiary_dim: DYNAMIC_COLORS.tertiary_dim().get_argb(scheme),
      on_tertiary: DYNAMIC_COLORS.on_tertiary().get_argb(scheme),
      tertiary_container: DYNAMIC_COLORS.tertiary_container().get_argb(scheme),
      on_tertiary_container: DYNAMIC_COLORS.on_tertiary_container().get_argb(scheme),
      tertiary_fixed: DYNAMIC_COLORS.tertiary_fixed().get_argb(scheme),
      tertiary_fixed_dim: DYNAMIC_COLORS.tertiary_fixed_dim().get_argb(scheme),
      on_tertiary_fixed: DYNAMIC_COLORS.on_tertiary_fixed().get_argb(scheme),
      on_tertiary_fixed_variant: DYNAMIC_COLORS.on_tertiary_fixed_variant().get_argb(scheme),
      error: DYNAMIC_COLORS.error().get_argb(scheme),
      error_dim: DYNAMIC_COLORS.error_dim().get_argb(scheme),
      on_error: DYNAMIC_COLORS.on_error().get_argb(scheme),
      error_container: DYNAMIC_COLORS.error_container().get_argb(scheme),
      on_error_container: DYNAMIC_COLORS.on_error_container().get_argb(scheme),
      control_activated: DYNAMIC_COLORS.control_activated().get_argb(scheme),
      control_normal: DYNAMIC_COLORS.control_normal().get_argb(scheme),
      control_highlight: DYNAMIC_COLORS.control_highlight().get_argb(scheme),
      text_primary_inverse: DYNAMIC_COLORS.text_primary_inverse().get_argb(scheme),
      text_secondary_and_tertiary_inverse: DYNAMIC_COLORS
        .text_secondary_and_tertiary_inverse()
        .get_argb(scheme),
      text_primary_inverse_disable_only: DYNAMIC_COLORS
        .text_primary_inverse_disable_only()
        .get_argb(scheme),
      text_secondary_and_tertiary_inverse_disabled: DYNAMIC_COLORS
        .text_secondary_and_tertiary_inverse_disabled()
        .get_argb(scheme),
      text_hint_inverse: DYNAMIC_COLORS.text_hint_inverse().get_argb(scheme),
    }
  }
}
impl From<&DynamicColors<u32>> for DynamicColors<String> {
  fn from(value: &DynamicColors<u32>) -> Self {
    Self {
      primary_palette_key_color: css_hex_from_argb(value.primary_palette_key_color),
      secondary_palette_key_color: css_hex_from_argb(value.secondary_palette_key_color),
      tertiary_palette_key_color: css_hex_from_argb(value.tertiary_palette_key_color),
      neutral_palette_key_color: css_hex_from_argb(value.neutral_palette_key_color),
      neutral_variant_palette_key_color: css_hex_from_argb(value.neutral_variant_palette_key_color),
      error_palette_key_color: css_hex_from_argb(value.error_palette_key_color),
      background: css_hex_from_argb(value.background),
      on_background: css_hex_from_argb(value.on_background),
      surface: css_hex_from_argb(value.surface),
      surface_dim: css_hex_from_argb(value.surface_dim),
      surface_bright: css_hex_from_argb(value.surface_bright),
      surface_container_lowest: css_hex_from_argb(value.surface_container_lowest),
      surface_container_low: css_hex_from_argb(value.surface_container_low),
      surface_container: css_hex_from_argb(value.surface_container),
      surface_container_high: css_hex_from_argb(value.surface_container_high),
      surface_container_highest: css_hex_from_argb(value.surface_container_highest),
      on_surface: css_hex_from_argb(value.on_surface),
      surface_variant: css_hex_from_argb(value.surface_variant),
      on_surface_variant: css_hex_from_argb(value.on_surface_variant),
      inverse_surface: css_hex_from_argb(value.inverse_surface),
      inverse_on_surface: css_hex_from_argb(value.inverse_on_surface),
      outline: css_hex_from_argb(value.outline),
      outline_variant: css_hex_from_argb(value.outline_variant),
      shadow: css_hex_from_argb(value.shadow),
      scrim: css_hex_from_argb(value.scrim),
      surface_tint: css_hex_from_argb(value.surface_tint),
      primary: css_hex_from_argb(value.primary),
      primary_dim: css_hex_from_argb(value.primary_dim),
      on_primary: css_hex_from_argb(value.on_primary),
      primary_container: css_hex_from_argb(value.primary_container),
      on_primary_container: css_hex_from_argb(value.on_primary_container),
      inverse_primary: css_hex_from_argb(value.inverse_primary),
      primary_fixed: css_hex_from_argb(value.primary_fixed),
      primary_fixed_dim: css_hex_from_argb(value.primary_fixed_dim),
      on_primary_fixed: css_hex_from_argb(value.on_primary_fixed),
      on_primary_fixed_variant: css_hex_from_argb(value.on_primary_fixed_variant),
      secondary: css_hex_from_argb(value.secondary),
      secondary_dim: css_hex_from_argb(value.secondary_dim),
      on_secondary: css_hex_from_argb(value.on_secondary),
      secondary_container: css_hex_from_argb(value.secondary_container),
      on_secondary_container: css_hex_from_argb(value.on_secondary_container),
      secondary_fixed: css_hex_from_argb(value.secondary_fixed),
      secondary_fixed_dim: css_hex_from_argb(value.secondary_fixed_dim),
      on_secondary_fixed: css_hex_from_argb(value.on_secondary_fixed),
      on_secondary_fixed_variant: css_hex_from_argb(value.on_secondary_fixed_variant),
      tertiary: css_hex_from_argb(value.tertiary),
      tertiary_dim: css_hex_from_argb(value.tertiary_dim),
      on_tertiary: css_hex_from_argb(value.on_tertiary),
      tertiary_container: css_hex_from_argb(value.tertiary_container),
      on_tertiary_container: css_hex_from_argb(value.on_tertiary_container),
      tertiary_fixed: css_hex_from_argb(value.tertiary_fixed),
      tertiary_fixed_dim: css_hex_from_argb(value.tertiary_fixed_dim),
      on_tertiary_fixed: css_hex_from_argb(value.on_tertiary_fixed),
      on_tertiary_fixed_variant: css_hex_from_argb(value.on_tertiary_fixed_variant),
      error: css_hex_from_argb(value.error),
      error_dim: css_hex_from_argb(value.error_dim),
      on_error: css_hex_from_argb(value.on_error),
      error_container: css_hex_from_argb(value.error_container),
      on_error_container: css_hex_from_argb(value.on_error_container),
      control_activated: css_hex_from_argb(value.control_activated),
      control_normal: css_hex_from_argb(value.control_normal),
      control_highlight: css_hex_from_argb(value.control_highlight),
      text_primary_inverse: css_hex_from_argb(value.text_primary_inverse),
      text_secondary_and_tertiary_inverse: css_hex_from_argb(
        value.text_secondary_and_tertiary_inverse,
      ),
      text_primary_inverse_disable_only: css_hex_from_argb(value.text_primary_inverse_disable_only),
      text_secondary_and_tertiary_inverse_disabled: css_hex_from_argb(
        value.text_secondary_and_tertiary_inverse_disabled,
      ),
      text_hint_inverse: css_hex_from_argb(value.text_hint_inverse),
    }
  }
}

impl From<&DynamicScheme> for DynamicColors<String> {
  fn from(scheme: &DynamicScheme) -> Self {
    let argb: DynamicColors<u32> = DynamicColors::from(scheme);
    (&argb).into()
  }
}
