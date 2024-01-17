use crate::errors::*;
use cfg_if::cfg_if;
use leptos::Serializable;
use serde::Serialize;
use serde_json::Value;
use std::time::*;
use chrono::offset::Utc;
use chrono::DateTime;
// use time::OffsetDateTime;


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
  use wasm_cookies::{cookies::*, set};

  use chrono::offset::Utc;
  use chrono::DateTime;
  let now = chrono::offset::Utc::now();

      // let now = SystemTime::now();
  let d = now + *expires;// std::time::Duration::from_secs(604800);

  // let mut now = SystemTime::now();
  // now += expires;
  // let datetime: DateTime<Utc> = now.into();

  // set(
  //   "path",
  //   "value",
  //   &CookieOptions::default(),
  //   // &CookieOptions {
  //   //   same_site: SameSite::Strict,
  //   //   secure: true,
  //   //   expires: Some(std::borrow::Cow::Borrowed(&datetime.to_rfc3339()[..])),
  //   //   domain: None,
  //   //   path: Some("/"),
  //   // },
  // );

  // leptos::logging::log!("{}", d.to_rfc2822());

  set(
    path,
    value,
    // &CookieOptions::default(),
    &CookieOptions {
      same_site: SameSite::Strict,
      secure: true,
      expires: Some(std::borrow::Cow::Borrowed(&d.to_rfc2822())), //"Sat, 04 Jan 2025 19:24:51 GMT")), //&d.to_rfc3339()[..])),
      domain: None,
      path: Some("/"),
    },
  );



  // use crate::wasm_bindgen::JsCast;
  // use leptos::document;
  // use wasm_cookies::{CookieOptions,SameSite};

  // let r = //window()
  //   document()
  //   // .ok_or(LemmyAppError {
  //   //   error_type: LemmyAppErrorType::Unknown,
  //   //   content: "no document".to_string(),
  //   // })
  //   // ?
  //   // .unwrap()
  //   .dyn_into::<web_sys::HtmlDocument>()
  //   // .map_err(|_| LemmyAppError::APIError {
  //   //   error: String::from("DOM document could not be cast"),
  //   // })
  //   // ?
  // .ok().unwrap()
  // .set_cookie(
  //     &wasm_cookies::cookies::set(
  //     path, 
  //     &value, 
  //     // &CookieOptions::default(),
  //     &CookieOptions {
  //       same_site: SameSite::Strict,
  //       secure: true,
  //       expires: Some(std::borrow::Cow::Borrowed(&now.to_rfc3339())),
  //       domain: None,
  //       path: Some("/"),
  //     },
  //   )
  // )
  //   // .map_err(|_| LemmyAppError::APIError {
  //   //   error: String::from("Cookie could not be set"),
  //   // })
  //   ;

  // r
  Ok(())
}

#[cfg(not(feature = "ssr"))]
pub async fn remove_cookie(path: &str) -> Result<(), LemmyAppError> {
  let r = wasm_cookies::delete(path);
  Ok(())
}

#[cfg(feature = "ssr")]
pub async fn set_cookie(path: &str, value: &str, expires: &Duration) -> Result<(), LemmyAppError> {
  use actix_web::{
    cookie::{
      time::OffsetDateTime,
      Cookie, SameSite,
    },
    http::{header, header::HeaderValue},
  };
  use leptos::{expect_context, logging};
  use leptos_actix::ResponseOptions;

  let response = expect_context::<ResponseOptions>();

  let mut cookie = Cookie::build(path, value).finish();
  let now = SystemTime::now();
  let d = now + *expires;

  cookie.set_expires(OffsetDateTime::from(d));
  cookie.set_path("/");
  // cookie.set_domain(domain)
  cookie.set_secure(Some(true));
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
    cookie::{
      time::{Duration, OffsetDateTime},
      Cookie,
    },
    http::{header, header::HeaderValue},
  };
  use leptos::expect_context;
  use leptos_actix::ResponseOptions;

  let response = expect_context::<ResponseOptions>();

  let mut cookie = Cookie::build(path, "").finish();
  let mut now = OffsetDateTime::now_utc();
  now += Duration::weeks(-1);
  cookie.set_expires(now);
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
  }).await?;

  Ok(cookie_value)
}