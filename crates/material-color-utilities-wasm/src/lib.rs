use std::collections::HashMap;

use material_color_utilities::{
  dynamiccolor::{DynamicScheme, MaterialDynamicColors},
  palettes::TonalPalette,
};
use serde::{Deserialize, Serialize};
use tsify::{declare, Tsify};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::types::{DynamicColors, DynamicSchemeOptions, TonalPaletteOptions};

mod types;
mod utils;

// #[declare]
// type Return = HashMap<String, u32>;

#[derive(Deserialize, Serialize, Tsify)]
#[tsify(from_wasm_abi, into_wasm_abi, hashmap_as_object)]
pub struct Return(HashMap<String, u32>);

#[wasm_bindgen]
pub fn material_dynamic_colors(options: DynamicSchemeOptions) -> DynamicColors<String> {
  let scheme = DynamicScheme::from(options);
  DynamicColors::from(&scheme)
  // let result: HashMap<_, _> = MaterialDynamicColors::new()
  //   .all_dynamic_colors()
  //   .into_iter()
  //   .map(|dynamic_color| {
  //     (
  //       dynamic_color.name().clone(),
  //       dynamic_color.get_argb(&scheme),
  //     )
  //   })
  //   .collect();
  // Return(result)
  // Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn tonal_palette_get_tone(options: TonalPaletteOptions) {
  let palette = TonalPalette::from(options);
}
