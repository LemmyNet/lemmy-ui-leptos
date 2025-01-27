use leptos::prelude::*;
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

impl IntoAttributeValue for Theme {
  type Output = Oco<'static, str>;

  fn into_attribute_value(self) -> Self::Output {
    Oco::Borrowed(self.into())
  }
}
