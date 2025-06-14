mod cam16;
#[allow(clippy::module_inception)]
mod hct;
mod hct_solver;
mod viewing_conditions;

pub(crate) use cam16::XYZ_TO_CAM16RGB;

pub use cam16::Cam16;
pub use hct::Hct;
pub use viewing_conditions::ViewingConditions;

#[cfg(test)]
mod tests {
  use super::*;

  use crate::utils;

  #[test]
  fn preserves_original_color() {
    for r in (0..296).step_by(37) {
      for g in (0..296).step_by(37) {
        for b in (0..296).step_by(37) {
          let argb = utils::color::argb_from_rgb(
            r.min(255) as u8,
            g.min(255) as u8,
            b.min(255) as u8,
          );
          let hct = Hct::from_int(argb);
          let reconstructed =
            Hct::from(hct.hue(), hct.chroma(), hct.tone()).to_int();
          assert_eq!(reconstructed, argb);
        }
      }
    }
  }
}
