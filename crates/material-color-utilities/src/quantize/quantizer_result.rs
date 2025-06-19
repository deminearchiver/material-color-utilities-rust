use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QuantizerResult {
  pub color_to_count: HashMap<u32, usize>,
}
