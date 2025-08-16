#![allow(clippy::type_complexity)]

use std::{
  cell::{RefCell, RefMut},
  collections::HashMap,
  fmt::Debug,
  rc::Rc,
};

use by_address::ByAddress;

use crate::{
  contrast,
  dynamiccolor::{ContrastCurve, DynamicScheme, SpecVersion, ToneDeltaPair},
  hct::Hct,
  palettes::TonalPalette,
};

pub struct DynamicColor<'a> {
  name: String,
  palette: Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> &'a TonalPalette + 'a>>,
  tone: Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> f64 + 'a>>,
  is_background: bool,
  chroma_multiplier: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> f64 + 'a>>>,
  background: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>>>,
  second_background:
    Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>>>,
  contrast_curve: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<ContrastCurve> + 'a>>>,
  tone_delta_pair:
    Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<ToneDeltaPair<'a>> + 'a>>>,
  opacity: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<f64> + 'a>>>,
  hct_cache: RefCell<HashMap<ByAddress<&'a DynamicScheme>, Hct>>,
}

impl<'a> DynamicColor<'a> {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    name: String,
    palette: impl FnMut(&'a DynamicScheme) -> &'a TonalPalette + 'a,
    tone: impl FnMut(&'a DynamicScheme) -> f64 + 'a,
    is_background: bool,
    chroma_multiplier: Option<impl FnMut(&'a DynamicScheme) -> f64 + 'a>,
    background: Option<impl FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>,
    second_background: Option<impl FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>,
    contrast_curve: Option<impl FnMut(&'a DynamicScheme) -> Option<ContrastCurve> + 'a>,
    tone_delta_pair: Option<impl FnMut(&'a DynamicScheme) -> Option<ToneDeltaPair<'a>> + 'a>,
    opacity: Option<impl FnMut(&'a DynamicScheme) -> Option<f64> + 'a>,
  ) -> Self {
    Self {
      name,
      palette: Rc::new(RefCell::new(palette)),
      tone: Rc::new(RefCell::new(tone)) as Rc<RefCell<_>>,
      is_background,
      chroma_multiplier: chroma_multiplier.map(|f| Rc::new(RefCell::new(f)) as Rc<RefCell<_>>),
      background: background.map(|f| Rc::new(RefCell::new(f)) as Rc<RefCell<_>>),
      second_background: second_background.map(|f| Rc::new(RefCell::new(f)) as Rc<RefCell<_>>),
      contrast_curve: contrast_curve.map(|f| Rc::new(RefCell::new(f)) as Rc<RefCell<_>>),
      tone_delta_pair: tone_delta_pair.map(|f| Rc::new(RefCell::new(f)) as Rc<RefCell<_>>),
      opacity: opacity.map(|f| Rc::new(RefCell::new(f)) as Rc<RefCell<_>>),
      hct_cache: RefCell::new(HashMap::default()),
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn palette(&self) -> RefMut<'_, dyn FnMut(&'a DynamicScheme) -> &'a TonalPalette> {
    self.palette.borrow_mut()
  }

  pub fn tone(&self) -> RefMut<'_, dyn FnMut(&'a DynamicScheme) -> f64> {
    self.tone.borrow_mut()
  }

  pub fn is_background(&self) -> bool {
    self.is_background
  }

  pub fn chroma_multiplier(&self) -> Option<RefMut<'_, dyn FnMut(&'a DynamicScheme) -> f64>> {
    match self.chroma_multiplier.as_ref() {
      Some(f) => Some(f.borrow_mut()),
      None => None,
    }
  }

  pub fn background(
    &self,
  ) -> Option<RefMut<'_, dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>>>> {
    match self.background.as_ref() {
      Some(f) => Some(f.borrow_mut()),
      None => None,
    }
  }

  pub fn second_background(
    &self,
  ) -> Option<RefMut<'_, dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>>>> {
    match self.second_background.as_ref() {
      Some(f) => Some(f.borrow_mut()),
      None => None,
    }
  }

  pub fn contrast_curve(
    &self,
  ) -> Option<RefMut<'_, dyn FnMut(&'a DynamicScheme) -> Option<ContrastCurve>>> {
    match self.contrast_curve.as_ref() {
      Some(f) => Some(f.borrow_mut()),
      None => None,
    }
  }

  pub fn tone_delta_pair(
    &self,
  ) -> Option<RefMut<'_, dyn FnMut(&'a DynamicScheme) -> Option<ToneDeltaPair<'a>>>> {
    match self.tone_delta_pair.as_ref() {
      Some(f) => Some(f.borrow_mut()),
      None => None,
    }
  }

  pub fn opacity(&self) -> Option<RefMut<'_, dyn FnMut(&'a DynamicScheme) -> Option<f64>>> {
    match self.opacity.as_ref() {
      Some(f) => Some(f.borrow_mut()),
      None => None,
    }
  }

  pub fn get_hct(&self, scheme: &'a DynamicScheme) -> Hct {
    if !self.hct_cache.borrow().contains_key(&ByAddress(scheme)) {
      let answer = scheme
        .spec_version()
        .color_calculation_spec()
        .get_hct(scheme, self);
      if self.hct_cache.borrow().len() > 4 {
        self.hct_cache.borrow_mut().clear();
      }
      self
        .hct_cache
        .borrow_mut()
        .insert(ByAddress(scheme), answer);
    }
    self
      .hct_cache
      .borrow()
      .get(&ByAddress(scheme))
      .unwrap()
      .clone()
  }

  pub fn get_argb(&self, scheme: &'a DynamicScheme) -> u32 {
    let argb = self.get_hct(scheme).to_int();
    if let Some(percentage) = self.opacity().and_then(|mut f| f(scheme)) {
      let alpha = (percentage * 255.0).round().clamp(0.0, 255.0) as u32;
      (argb & 0x00ffffff) | (alpha << 24)
    } else {
      argb
    }
  }

  pub fn get_tone(&self, scheme: &'a DynamicScheme) -> f64 {
    scheme
      .spec_version()
      .color_calculation_spec()
      .get_tone(scheme, self)
  }

  pub fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
    let lighter_tone = contrast::lighter_unsafe(bg_tone, ratio);
    let darker_tone = contrast::darker_unsafe(bg_tone, ratio);
    let lighter_ratio = contrast::ratio_of_tones(lighter_tone, bg_tone);
    let darker_ratio = contrast::ratio_of_tones(darker_tone, bg_tone);
    let prefer_lighter = Self::tone_prefers_light_foreground(bg_tone);

    if prefer_lighter {
      // "Neglible difference" handles an edge case where the initial contrast ratio is high
      // (ex. 13.0), and the ratio passed to the function is that high ratio, and both the lighter
      // and darker ratio fails to pass that ratio.
      //
      // This was observed with Tonal Spot's On Primary Container turning black momentarily between
      // high and max contrast in light mode. PC's standard tone was T90, OPC's was T10, it was
      // light mode, and the contrast level was 0.6568521221032331.
      let negligible_difference =
        (lighter_ratio - darker_ratio).abs() < 0.1 && lighter_ratio < ratio && darker_ratio < ratio;
      if lighter_ratio >= ratio || lighter_ratio >= darker_ratio || negligible_difference {
        lighter_tone
      } else {
        darker_tone
      }
    } else if darker_ratio >= ratio || darker_ratio >= lighter_ratio {
      darker_tone
    } else {
      lighter_tone
    }
  }

  pub fn enable_light_foreground(tone: f64) -> f64 {
    if Self::tone_prefers_light_foreground(tone) && !Self::tone_allows_light_foreground(tone) {
      49.0
    } else {
      tone
    }
  }

  pub fn tone_prefers_light_foreground(tone: f64) -> bool {
    tone.round() < 60.0
  }

  pub fn tone_allows_light_foreground(tone: f64) -> bool {
    tone.round() <= 49.0
  }

  pub fn get_initial_tone_from_background(
    background: Option<impl FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>,
  ) -> Box<dyn FnMut(&'a DynamicScheme) -> f64 + 'a> {
    if let Some(mut background) = background {
      Box::new(move |s| {
        background(s)
          .map(|color| color.get_tone(s))
          .unwrap_or_else(|| 50.0)
      })
    } else {
      Box::new(|_contrast_level| 50.0)
    }
  }
}

impl Debug for DynamicColor<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DynamicColor")
      .field("name", &self.name())
      .field("palette", &"<anonymous closure>")
      .field("tone", &"<anonymous closure>")
      .field("is_background", &self.is_background())
      .field("chroma_multiplier", &"<anonymous closure>")
      .field("background", &"<anonymous closure>")
      .field("second_background", &"<anonymous closure>")
      .field("contrast_curve", &"<anonymous closure>")
      .field("tone_delta_pair", &"<anonymous closure>")
      .field("opacity", &"<anonymous closure>")
      .finish()
  }
}

impl<'a> TryFrom<DynamicColorBuilder<'a>> for DynamicColor<'a> {
  type Error = ();

  fn try_from(value: DynamicColorBuilder<'a>) -> Result<Self, Self::Error> {
    value.build().ok_or(())
  }
}

#[derive(Default)]
pub struct DynamicColorBuilder<'a> {
  name: Option<String>,
  palette: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> &'a TonalPalette + 'a>>>,
  tone: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> f64 + 'a>>>,
  is_background: Option<bool>,
  chroma_multiplier: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> f64 + 'a>>>,
  background: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>>>,
  second_background:
    Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>>>,
  contrast_curve: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<ContrastCurve> + 'a>>>,
  tone_delta_pair:
    Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<ToneDeltaPair<'a>> + 'a>>>,
  opacity: Option<Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<f64> + 'a>>>,
}

impl<'a> DynamicColorBuilder<'a> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn name<I>(mut self, name: I) -> Self
  where
    I: Into<String>,
  {
    self.name = Some(name.into());
    self
  }

  pub fn palette(
    mut self,
    palette: impl FnMut(&'a DynamicScheme) -> &'a TonalPalette + 'a,
  ) -> Self {
    self.palette = Some(Rc::new(RefCell::new(palette)));
    self
  }

  pub fn tone(mut self, tone: impl FnMut(&'a DynamicScheme) -> f64 + 'a) -> Self {
    self.tone = Some(Rc::new(RefCell::new(tone)));
    self
  }

  pub fn is_background(mut self, is_background: bool) -> Self {
    self.is_background = Some(is_background);
    self
  }

  pub fn chroma_multiplier(
    mut self,
    chroma_multiplier: impl FnMut(&'a DynamicScheme) -> f64 + 'a,
  ) -> Self {
    self.chroma_multiplier = Some(Rc::new(RefCell::new(chroma_multiplier)));
    self
  }

  pub fn background(
    mut self,
    background: impl FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a,
  ) -> Self {
    self.background = Some(Rc::new(RefCell::new(background)));
    self
  }
  pub fn second_background(
    mut self,
    second_background: impl FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a,
  ) -> Self {
    self.second_background = Some(Rc::new(RefCell::new(second_background)));
    self
  }

  pub fn contrast_curve(
    mut self,
    contrast_curve: impl FnMut(&'a DynamicScheme) -> Option<ContrastCurve> + 'a,
  ) -> Self {
    self.contrast_curve = Some(Rc::new(RefCell::new(contrast_curve)));
    self
  }

  pub fn tone_delta_pair(
    mut self,
    tone_delta_pair: impl FnMut(&'a DynamicScheme) -> Option<ToneDeltaPair<'a>> + 'a,
  ) -> Self {
    self.tone_delta_pair = Some(Rc::new(RefCell::new(tone_delta_pair)));
    self
  }
  pub fn opacity(mut self, opacity: impl FnMut(&'a DynamicScheme) -> Option<f64> + 'a) -> Self {
    self.opacity = Some(Rc::new(RefCell::new(opacity)));
    self
  }

  pub fn extend_spec_version(
    self,
    spec_version: SpecVersion,
    extended_color: DynamicColor<'a>,
  ) -> Result<Self, String> {
    let name = self.name.unwrap();
    let is_background = self.is_background.unwrap();
    if &name != extended_color.name() {
      return Err(format!(
        "Attempting to extend color {} with color {} of different name for spec version {:?}.",
        name,
        extended_color.name(),
        spec_version,
      ));
    }
    if is_background != extended_color.is_background() {
      return Err(format!(
        "Attempting to extend color {} as a {} with color {} as a {} for spec version {:?}.",
        name,
        if is_background {
          "background"
        } else {
          "foreground"
        },
        extended_color.name(),
        if extended_color.is_background() {
          "background"
        } else {
          "foreground"
        },
        spec_version,
      ));
    }
    // let palette = self.palette.unwrap();
    let builder = DynamicColorBuilder::new()
      .name(name)
      .is_background(is_background)
      .palette(move |s| {
        if s.spec_version() == &spec_version {
          &extended_color.palette
        } else {
          self.palette.as_ref().unwrap()
        }
        .borrow_mut()(s)
      })
      .tone(move |s| {
        if s.spec_version() == &spec_version {
          &extended_color.tone
        } else {
          self.tone.as_ref().unwrap()
        }
        .borrow_mut()(s)
      })
      .chroma_multiplier(move |s: &'a DynamicScheme| {
        if s.spec_version() == &spec_version {
          &extended_color.chroma_multiplier
        } else {
          &self.chroma_multiplier
        }
        .as_ref()
        .map(|f| f.borrow_mut()(s))
        .unwrap_or(1.0)
      })
      .background(move |s: &'a DynamicScheme| {
        if s.spec_version() == &spec_version {
          &extended_color.background
        } else {
          &self.background
        }
        .as_ref()
        .and_then(|f| f.borrow_mut()(s))
      })
      .second_background(move |s| {
        if s.spec_version() == &spec_version {
          &extended_color.second_background
        } else {
          &self.second_background
        }
        .as_ref()
        .and_then(|f| f.borrow_mut()(s))
      })
      .contrast_curve(move |s: &'a DynamicScheme| {
        if s.spec_version() == &spec_version {
          &extended_color.contrast_curve
        } else {
          &self.contrast_curve
        }
        .as_ref()
        .and_then(|f| f.borrow_mut()(s))
      })
      .tone_delta_pair(move |s: &'a DynamicScheme| {
        if s.spec_version() == &spec_version {
          &extended_color.tone_delta_pair
        } else {
          &self.tone_delta_pair
        }
        .as_ref()
        .and_then(|f| f.borrow_mut()(s))
      })
      .opacity(move |s: &'a DynamicScheme| {
        if s.spec_version() == &spec_version {
          &extended_color.opacity
        } else {
          &self.opacity
        }
        .as_ref()
        .and_then(|f| f.borrow_mut()(s))
      });
    Ok(builder)
  }

  pub fn build(self) -> Option<DynamicColor<'a>> {
    if self.name.is_none() || self.palette.is_none() {
      return None;
    }
    let name = self.name.unwrap();
    let palette = self.palette.unwrap();
    let is_background = self.is_background.unwrap_or(false);
    let chroma_multiplier = self.chroma_multiplier;
    let background = self.background;
    let second_background = self.second_background;
    let contrast_curve = self.contrast_curve;
    let tone_delta_pair = self.tone_delta_pair;
    let opacity = self.opacity;
    let tone = match self.tone {
      Some(tone) => tone,
      None => {
        let tone = match background.as_ref() {
          Some(background) => {
            let outer: Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>> =
              Rc::clone(background);
            let callback = move |s: &'a DynamicScheme| {
              let inner = Rc::clone(&outer);
              inner.borrow_mut()(s)
            };
            DynamicColor::get_initial_tone_from_background(Some(callback))
          }
          None => DynamicColor::get_initial_tone_from_background(
            None::<Box<dyn FnMut(&'a DynamicScheme) -> Option<DynamicColor<'a>> + 'a>>,
          ),
        };
        Rc::new(RefCell::new(tone)) as Rc<RefCell<dyn FnMut(&'a DynamicScheme) -> f64 + 'a>>
      }
    };

    Some(DynamicColor {
      name,
      palette,
      tone,
      is_background,
      chroma_multiplier,
      background,
      second_background,
      contrast_curve,
      tone_delta_pair,
      opacity,
      hct_cache: RefCell::new(HashMap::default()),
    })
  }
}

impl<'a> From<DynamicColor<'a>> for DynamicColorBuilder<'a> {
  fn from(value: DynamicColor<'a>) -> Self {
    DynamicColorBuilder {
      name: Some(value.name),
      palette: Some(value.palette),
      tone: Some(value.tone),
      is_background: Some(value.is_background),
      chroma_multiplier: value.chroma_multiplier,
      background: value.background,
      second_background: value.second_background,
      contrast_curve: value.contrast_curve,
      tone_delta_pair: value.tone_delta_pair,
      opacity: value.opacity,
    }
  }
}
