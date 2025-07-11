#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// All available spec versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Platform {
  Phone,
  Watch,
}
