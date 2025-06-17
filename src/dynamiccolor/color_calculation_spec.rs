use crate::{
  dynamiccolor::{DynamicColor, DynamicScheme},
  hct::Hct,
};

pub trait ColorCalculationSpec {
  fn get_hct<'a>(&self, scheme: &'a DynamicScheme, color: &DynamicColor<'a>) -> Hct;
  fn get_tone<'a>(&self, scheme: &'a DynamicScheme, color: &DynamicColor<'a>) -> f64;
}
