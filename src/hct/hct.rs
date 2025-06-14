use std::fmt::Display;

use num_traits::{Float, FromPrimitive};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
  hct::{Cam16, ViewingConditions, hct_solver},
  utils,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Hct {
  hue: f64,
  chroma: f64,
  tone: f64,
  argb: u32,
}

impl Hct {
  fn new(argb: u32) -> Self {
    let cam = Cam16::from_int(argb);
    Self {
      argb,
      hue: cam.hue(),
      chroma: cam.chroma(),
      tone: utils::color::lstar_from_argb(argb),
    }
  }
  pub fn from(hue: f64, chroma: f64, tone: f64) -> Self {
    let argb = hct_solver::solve_to_int(hue, chroma, tone);
    Self::new(argb)
  }

  pub fn from_int(argb: u32) -> Self {
    Self::new(argb)
  }

  pub fn hue(&self) -> f64 {
    self.hue
  }

  pub fn set_hue(&mut self, new_hue: f64) {
    self.set_internal_state(hct_solver::solve_to_int(
      new_hue,
      self.chroma,
      self.tone,
    ));
  }

  pub fn chroma(&self) -> f64 {
    self.chroma
  }

  pub fn set_chroma(&mut self, new_chroma: f64) {
    self.set_internal_state(hct_solver::solve_to_int(
      self.hue, new_chroma, self.tone,
    ));
  }

  pub fn tone(&self) -> f64 {
    self.tone
  }

  pub fn set_tone(&mut self, new_tone: f64) {
    self.set_internal_state(hct_solver::solve_to_int(
      self.hue,
      self.chroma,
      new_tone,
    ));
  }

  pub fn to_int(&self) -> u32 {
    self.argb
  }

  pub fn in_viewing_conditions(&self, vc: &ViewingConditions) -> Hct {
    // 1. Use CAM16 to find XYZ coordinates of color in specified VC.
    let cam16 = Cam16::from_int(self.to_int());
    let viewed_in_vc = cam16.xyz_in_viewing_conditions(vc);

    // 2. Create CAM16 of those XYZ coordinates in default VC.
    let recast_in_vc = Cam16::from_xyz_in_viewing_conditions(
      viewed_in_vc[0],
      viewed_in_vc[1],
      viewed_in_vc[2],
      &ViewingConditions::default(),
    );

    // 3. Create HCT from:
    // - CAM16 using default VC with XYZ coordinates in specified VC.
    // - L* converted from Y in XYZ coordinates in specified VC.
    Hct::from(
      recast_in_vc.hue(),
      recast_in_vc.chroma(),
      utils::color::lstar_from_y(viewed_in_vc[1]),
    )
  }

  fn set_internal_state(&mut self, argb: u32) {
    self.argb = argb;
    let cam = Cam16::from_int(argb);
    self.hue = cam.hue();
    self.chroma = cam.chroma();
    self.tone = utils::color::lstar_from_argb(argb);
  }

  pub fn is_blue<T>(hue: T) -> bool
  where
    T: Float + FromPrimitive,
  {
    hue >= T::from_f64(250.0).unwrap() && hue < T::from_f64(270.0).unwrap()
  }

  pub fn is_yellow<T>(hue: T) -> bool
  where
    T: Float + FromPrimitive,
  {
    hue >= T::from_f64(105.0).unwrap() && hue < T::from_f64(125.0).unwrap()
  }

  pub fn is_cyan<T>(hue: T) -> bool
  where
    T: Float + FromPrimitive,
  {
    hue >= T::from_f64(170.0).unwrap() && hue < T::from_f64(207.0).unwrap()
  }
}

impl Display for Hct {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "HCT({}, {}, {})",
      self.hue().round() as i32,
      self.chroma().round() as i32,
      self.tone().round() as i32,
    )
  }
}
