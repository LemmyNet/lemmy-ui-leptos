use crate::utils::types::Theme;
use leptos::prelude::{server_fn::codec::GetUrl, *};
use std::str::FromStr;

#[server(prefix = "/serverfn", input = GetUrl)]
pub async fn get_theme() -> Result<Theme, ServerFnError> {
  use actix_web::HttpRequest;
  use leptos_actix::extract;

  let req = extract::<HttpRequest>().await?;

  Ok(req.cookie("theme").map_or(Theme::Light, |c| {
    Theme::from_str(c.value()).unwrap_or(Theme::Light)
  }))
}
