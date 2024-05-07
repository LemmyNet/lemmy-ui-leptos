use leptos::{
  create_blocking_resource,
  provide_context,
  server,
  server_fn::codec::GetUrl,
  Attribute,
  IntoAttribute,
  Oco,
  Resource,
  ServerFnError,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

pub type ThemeResource = Resource<(), Result<Theme, ServerFnError>>;

#[server(prefix = "/serverfn", input = GetUrl)]
async fn get_theme() -> Result<Theme, ServerFnError> {
  use actix_web::HttpRequest;
  use leptos_actix::extract;

  let req = extract::<HttpRequest>().await?;

  Ok(req.cookie("theme").map_or(Theme::Retro, |c| {
    Theme::from_str(c.value()).unwrap_or(Theme::Retro)
  }))
}

pub fn provide_theme_resource_context() {
  let theme_resource = create_blocking_resource(|| (), |_| get_theme());

  provide_context(theme_resource);
}
