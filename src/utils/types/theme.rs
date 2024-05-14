use leptos::*;
use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Theme {
  Light,
  Dark,
  Retro,
}

impl IntoAttribute for Theme {
  fn into_attribute(self) -> Attribute {
    Attribute::String(Oco::Borrowed(self.into()))
  }

  fn into_attribute_boxed(self: Box<Self>) -> Attribute {
    self.into_attribute()
  }
}
