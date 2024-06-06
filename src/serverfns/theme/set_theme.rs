use crate::utils::types::ServerAction;
use leptos::*;

#[server(prefix = "/serverfn")]
pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
  use actix_web::{
    cookie::{Cookie, SameSite},
    http::{header, header::HeaderValue},
  };
  use leptos_actix::ResponseOptions;

  let response = expect_context::<ResponseOptions>();

  let cookie = Cookie::build("theme", theme)
    .path("/")
    .secure(!cfg!(debug_assertions))
    .same_site(SameSite::Strict)
    .finish();

  if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
    response.insert_header(header::SET_COOKIE, cookie);
  }

  Ok(())
}

pub fn create_set_theme_action() -> ServerAction<ChangeTheme> {
  Action::server()
}
