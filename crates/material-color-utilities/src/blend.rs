use crate::{
  hct::{Cam16, Hct},
  utils,
};

/// Blend the design color's HCT hue towards the key color's HCT
/// hue, in a way that leaves the original color recognizable and
/// recognizably shifted towards the key color.
pub fn harmonize(design_color: u32, source_color: u32) -> u32 {
  let from_hct = Hct::from_int(design_color);
  let to_hct = Hct::from_int(source_color);
  let difference_degrees = utils::math::difference_degrees(from_hct.hue(), to_hct.hue());
  let rotation_degrees = f64::min(difference_degrees * 0.5, 15.0);
  let output_hue = utils::math::sanitize_degrees(
    from_hct.hue()
      + rotation_degrees * utils::math::rotation_direction(from_hct.hue(), to_hct.hue()),
  );
  Hct::from(output_hue, from_hct.chroma(), from_hct.tone()).to_int()
}

/// Blends hue from one color into another. The chroma and tone of
/// the original color are maintained.
pub fn hct_hue(from: u32, to: u32, amount: f64) -> u32 {
  let ucs = cam16_ucs(from, to, amount);
  let ucs_cam = Cam16::from_int(ucs);
  let from_cam = Cam16::from_int(from);
  let blended = Hct::from(
    ucs_cam.hue(),
    from_cam.chroma(),
    utils::color::lstar_from_argb(from),
  );
  blended.to_int()
}

/// Blend in CAM16-UCS space.
pub fn cam16_ucs(from: u32, to: u32, amount: f64) -> u32 {
  let from_cam = Cam16::from_int(from);
  let to_cam = Cam16::from_int(to);
  let from_j = from_cam.jstar();
  let from_a = from_cam.astar();
  let from_b = from_cam.bstar();
  let to_j = to_cam.j();
  let to_a = to_cam.astar();
  let to_b = to_cam.bstar();
  let jstar = from_j + (to_j - from_j) * amount;
  let astar = from_a + (to_a - from_a) * amount;
  let bstar = from_b + (to_b - from_b) * amount;
  Cam16::from_ucs(jstar, astar, bstar).to_int()
}

#[cfg(test)]
mod tests {
  use super::*;

  const RED: u32 = 0xffff0000;
  const BLUE: u32 = 0xff0000ff;
  const GREEN: u32 = 0xff00ff00;
  const YELLOW: u32 = 0xffffff00;

  #[test]
  fn harmonize_red_to_blue() {
    let answer = harmonize(RED, BLUE);
    assert_eq!(answer, 0xfffb0057);
  }

  #[test]
  fn harmonize_red_to_green() {
    let answer = harmonize(RED, GREEN);
    assert_eq!(answer, 0xffd85600);
  }

  #[test]
  fn harmonize_red_to_yellow() {
    let answer = harmonize(RED, YELLOW);
    assert_eq!(answer, 0xffd85600);
  }

  #[test]
  fn harmonize_blue_to_green() {
    let answer = harmonize(BLUE, GREEN);
    assert_eq!(answer, 0xff0047a3);
  }

  #[test]
  fn harmonize_blue_to_red() {
    let answer = harmonize(BLUE, RED);
    assert_eq!(answer, 0xff5700dc);
  }

  #[test]
  fn harmonize_blue_to_yellow() {
    let answer = harmonize(BLUE, YELLOW);
    assert_eq!(answer, 0xff0047a3);
  }

  #[test]
  fn harmonize_green_to_blue() {
    let answer = harmonize(GREEN, BLUE);
    assert_eq!(answer, 0xff00fc94);
  }

  #[test]
  fn harmonize_green_to_red() {
    let answer = harmonize(GREEN, RED);
    assert_eq!(answer, 0xffb1f000);
  }

  #[test]
  fn harmonize_green_to_yellow() {
    let answer = harmonize(GREEN, YELLOW);
    assert_eq!(answer, 0xffb1f000);
  }

  #[test]
  fn harmonize_yellow_to_blue() {
    let answer = harmonize(YELLOW, BLUE);
    assert_eq!(answer, 0xffebffba);
  }

  #[test]
  fn harmonize_yellow_to_green() {
    let answer = harmonize(YELLOW, GREEN);
    assert_eq!(answer, 0xffebffba);
  }

  #[test]
  fn harmonize_yellow_to_red() {
    let answer = harmonize(YELLOW, RED);
    assert_eq!(answer, 0xfffff6e3);
  }
}
