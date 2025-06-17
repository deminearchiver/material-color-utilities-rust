use crate::hct::Hct;

pub fn is_disliked(hct: &Hct) -> bool {
  let hue_passes = hct.hue().round() >= 90.0 && hct.hue().round() <= 111.0;
  let chroma_passes = hct.chroma().round() > 16.0;
  let tone_passes = hct.tone().round() < 65.0;
  hue_passes && chroma_passes && tone_passes
}

pub fn fix_if_disliked(hct: Hct) -> Hct {
  if is_disliked(&hct) {
    Hct::from(hct.hue(), hct.chroma(), 70.0)
  } else {
    hct
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn likes_monk_skin_tone_scale_colors() {
    let monk_skin_tone_scale_colors: [u32; 10] = [
      0xfff6ede4, 0xfff3e7db, 0xfff7ead0, 0xffeadaba, 0xffd7bd96, 0xffa07e56, 0xff825c43,
      0xff604134, 0xff3a312a, 0xff292420,
    ];
    for color in monk_skin_tone_scale_colors {
      assert!(!is_disliked(&Hct::from_int(color)));
    }
  }

  #[test]
  fn dislikes_bile_colors() {
    let unlikable: [u32; 5] = [0xff95884b, 0xff716b40, 0xffb08e00, 0xff4c4308, 0xff464521];
    for color in unlikable {
      assert!(is_disliked(&Hct::from_int(color)));
    }
  }

  #[test]
  fn makes_bile_colors_likable() {
    let unlikable: [u32; 5] = [0xff95884b, 0xff716b40, 0xffb08e00, 0xff4c4308, 0xff464521];
    for color in unlikable {
      let hct = Hct::from_int(color);
      assert!(is_disliked(&hct));
      let likable = fix_if_disliked(hct);
      assert!(!is_disliked(&likable));
    }
  }
}
