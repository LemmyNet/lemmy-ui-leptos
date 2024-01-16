use crate::errors::LemmyAppError;
use cfg_if::cfg_if;
use leptos::Serializable;
use serde::Serialize;
use serde_json::Value;

#[cfg(not(feature = "ssr"))]
pub async fn get_cookie(path: &str) -> Result<Option<String>, LemmyAppError> {
  let r = wasm_cookies::get(path);
  
  match r {
    Some(Ok(r)) => Ok(Some(r)),
    Some(Err(e)) => Err(e.into()),
    None => Ok(None),
  }
  
  //   if let Some(Ok(r)) = wasm_cookies::get(path) {
  //   r
  // } else {
  //   Ok(None)
  // }

//   use crate::wasm_bindgen::JsCast;
//   use leptos::window;

//   let cookie_string = window()
//     .document()
//     // .ok_or(LemmyAppError::APIError {
//     //   error: String::from("DOM document is None"),
//     // })
//     // ?
//     .dyn_into::<web_sys::HtmlDocument>()
//     // .map_err(|_| LemmyAppError::APIError {
//     //   error: String::from("DOM document could not be cast"),
//     // })
//     ?
//     .cookie()
//     // .map_err(|_| LemmyAppError::APIError {
//     //   error: String::from("Could not get cookie string"),
//     // })
//     ?;

//   if let Ok(value) = wasm_cookies::cookies::get(&cookie_string, name)
//     // .ok_or(LemmyAppError::APIError {
//     //   error: String::from("DOM cookie is None"),
//     // })
//     ?
//     // .map_err(|e| LemmyAppError::APIError {
//     //   error: e.to_string(),
//     // })
//   {
//     Ok(Some(value))
//   } else {
//     Ok(None)
//   }
}

#[cfg(not(feature = "ssr"))]
pub async fn set_cookie(path: &str, value: &str) -> Result<(), LemmyAppError> {
  use wasm_cookies::{cookies::*, set};
  set(
    path,
    value,
    &CookieOptions {
      same_site: SameSite::Strict,
      secure: true,
      expires: Some(std::borrow::Cow::Borrowed("Sat, 04 Jan 2025 19:24:51 GMT")),
      domain: None,
      path: None,
    },
  );
  Ok(())

//   use crate::wasm_bindgen::JsCast;
//   use leptos::window;
//   use wasm_cookies::CookieOptions;

//   let r = window()
//     .document()
//     // .ok_or(LemmyAppError::APIError {
//     //   error: String::from("DOM document is None"),
//     // })
//     ?
//     .dyn_into::<web_sys::HtmlDocument>()
//     // .map_err(|_| LemmyAppError::APIError {
//     //   error: String::from("DOM document could not be cast"),
//     // })
//     ?
//     .set_cookie(&wasm_cookies::cookies::set(path, value, &CookieOptions::default())[..])
//     // .map_err(|_| LemmyAppError::APIError {
//     //   error: String::from("Cookie could not be set"),
//     // })
//     ;

//   r
}

#[cfg(not(feature = "ssr"))]
pub async fn remove_cookie(path: &str) -> Result<(), LemmyAppError> {
  wasm_cookies::delete(path);
  Ok(())
//   use crate::wasm_bindgen::JsCast;
//   use leptos::window;

//   let r = window()
//     .document()
//     // .ok_or(LemmyAppError::APIError {
//     //   error: String::from("DOM document is None"),
//     // })
//     ?
//     .dyn_into::<web_sys::HtmlDocument>()
//     // .map_err(|_| LemmyAppError::APIError {
//     //   error: String::from("DOM document could not be cast"),
//     // })
//     ?
//     .set_cookie(&wasm_cookies::cookies::delete(path))
//     // .map_err(|_| LemmyAppError::APIError {
//     //   error: String::from("Cookie could not be set"),
//     // })
//     ;

//   r
}

#[cfg(feature = "ssr")]
pub async fn set_cookie(path: &str, value: &str) -> Result<(), LemmyAppError> {
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

  let mut cookie = Cookie::build(path, value).finish();
  let mut now = OffsetDateTime::now_utc();
  now += Duration::weeks(1);
  cookie.set_expires(now);
  cookie.set_path("/");

  if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
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
  })
  .await
//   .map_err(|e| LemmyAppError::APIError {
//     error: e.to_string(),
//   })
  ?;

  Ok(cookie_value)
}