use crate::errors::*;
use std::time::Duration;

#[cfg(not(feature = "ssr"))]
pub async fn get_cookie(path: &str) -> Result<Option<String>, LemmyAppError> {
  let r = wasm_cookies::get(path);

  match r {
    Some(Ok(r)) => Ok(Some(r)),
    Some(Err(e)) => Err(e.into()),
    None => Ok(None),
  }
}

#[cfg(not(feature = "ssr"))]
pub async fn set_cookie(path: &str, value: &str, expires: &Duration) -> Result<(), LemmyAppError> {
  use chrono::offset::Utc;
  use wasm_cookies::{cookies::*, set};
  let now = Utc::now();
  let d = now + *expires;

  set(
    path,
    value,
    &CookieOptions {
      same_site: SameSite::Strict,
      secure: false,
      expires: Some(std::borrow::Cow::Borrowed(&d.to_rfc2822())),
      domain: Some("localhost"),
      path: Some("/"),
    },
  );

  Ok(())
}

#[cfg(not(feature = "ssr"))]
pub async fn remove_cookie(path: &str) -> Result<(), LemmyAppError> {
  // wasm_cookies::delete(path);

  use chrono::offset::Utc;
  use wasm_cookies::{cookies::*, set};
  let now = Utc::now();
  let d = now - std::time::Duration::from_secs(604800);

  set(
    path,
    "value",
    &CookieOptions {
      same_site: SameSite::Strict,
      secure: false,
      expires: Some(std::borrow::Cow::Borrowed(&d.to_rfc2822())),
      domain: Some("localhost"),
      path: Some("/"),
    },
  );

  Ok(())
}

#[cfg(feature = "ssr")]
pub async fn set_cookie(path: &str, value: &str, expires: &Duration) -> Result<(), LemmyAppError> {
  use actix_web::{
    cookie::{time::OffsetDateTime, Cookie, SameSite},
    http::{header, header::HeaderValue},
  };
  use leptos::{expect_context, logging};
  use leptos_actix::ResponseOptions;

  let response = expect_context::<ResponseOptions>();

  let mut cookie = Cookie::build(path, value).finish();
  let now = OffsetDateTime::now_utc();

  let s = std::time::SystemTime::now();
  let d = now + *expires;

  cookie.set_expires(OffsetDateTime::from(s));
  cookie.set_path("/");
  cookie.set_domain("localhost");
  cookie.set_secure(Some(false));
  cookie.set_same_site(Some(SameSite::Strict));

  if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
    logging::log!("{:#?}", cookie);
    response.insert_header(header::SET_COOKIE, cookie);
  }

  Ok(())
}

#[cfg(feature = "ssr")]
pub async fn remove_cookie(path: &str) -> Result<(), LemmyAppError> {
  use actix_web::{
    cookie::{time::OffsetDateTime, Cookie},
    http::{header, header::HeaderValue},
  };
  use leptos::expect_context;
  use leptos_actix::ResponseOptions;

  let response = expect_context::<ResponseOptions>();

  let mut cookie = Cookie::build(path, "").finish();
  let now = OffsetDateTime::now_utc();

  let d = now - Duration::from_secs(604800);
  cookie.set_expires(d);
  cookie.set_domain("localhost");
  cookie.set_path("/");

  if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
    response.insert_header(header::SET_COOKIE, cookie);
  }

  Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_cookie(path: &str) -> Result<Option<String>, LemmyAppError> {
  use actix_web::HttpRequest;
  use leptos_actix::extract;

  let path_string = path.to_string().clone();

  let cookie_value = extract(|req: HttpRequest| async move {
    if let Some(c) = req.cookie(&path_string) {
      let s = c.clone();
      Some(s.value().to_string())
    } else {
      None
    }
  })
  .await?;

  Ok(cookie_value.clone())
}
