use crate::errors::LemmyAppError;
use cfg_if::cfg_if;
use leptos::Serializable;
use serde::Serialize;
use serde_json::Value;

const ENDPOINT: &str = "https://voyager.lemmy.ml/api/v3";

pub enum HttpType {
  Get,
  Post,
  Put,
}

/// Used if you hit a deser error, which usually means a LemmyAPI error
/// Of type {error: string}
// fn json_deser_err(json: &str) -> String {
//   serde_json::from_str(json)
//     .map(|v: Value| v["error"].as_str().unwrap_or("Unknown").to_string())
//     .unwrap_or("Unknown".to_string())
// }

// pub async fn api_wrapper<Response, Form>(
//   type_: HttpType,
//   path: &str,
//   form: &Form,
// ) -> Result<Response, LemmyAppError>
// where
//   Response: Serializable,
//   Form: Serialize + std::fmt::Debug,
// {
//   let route = &build_route(path);
//   #[allow(clippy::needless_late_init)]
//   let json;

//   cfg_if! {
//     if #[cfg(feature = "ssr")] {
//       let client = reqwest::Client::new();

//       let mut request_builder = match type_ {
//         HttpType::Get => client.get(&build_fetch_query(route, form)),
//         HttpType::Post => client.post(route),
//         HttpType::Put => client.put(route),
//       };

//       match get_cookie_wrapper("jwt").await {
//         Ok(jwt) => {
//           request_builder = request_builder.header("Authorization", &format!("Bearer {}", jwt)[..]);
//         },
//         Err(_) => {
//         },
//       };

//       json = match type_ {
//         HttpType::Get => request_builder.send().await?.text().await?,
//         HttpType::Post => request_builder.json(form).send().await?.text().await?,
//         HttpType::Put => request_builder.json(form).send().await?.text().await?,
//       };
//     } else {
//       use wasm_bindgen::UnwrapThrowExt;

//       let abort_controller = web_sys::AbortController::new().ok();
//       let abort_signal = abort_controller.as_ref().map(|a| a.signal());

//       let mut request_builder = match type_ {
//         HttpType::Get => gloo_net::http::Request::get(&build_fetch_query(route, form)),
//         HttpType::Post => gloo_net::http::Request::post(route),
//         HttpType::Put => gloo_net::http::Request::put(route),
//       };

//       match get_cookie_wrapper("jwt").await {
//         Ok(jwt) => {
//           request_builder = request_builder.header("Authorization", &format!("Bearer {}", jwt)[..]);
//         },
//         Err(_e) => {
//         },
//       };

//       json = match type_ {
//         HttpType::Get => {
//           request_builder
//             .abort_signal(abort_signal.as_ref())
//             .send()
//             .await?
//             .text()
//             .await?
//         }
//         HttpType::Post => {
//           request_builder
//             .abort_signal(abort_signal.as_ref())
//             .json(form)
//             .expect_throw("Could not parse json body")
//             .send()
//             .await?
//             .text()
//             .await?
//         }
//         HttpType::Put => {
//           request_builder
//             .abort_signal(abort_signal.as_ref())
//             .json(form)
//             .expect_throw("Could not parse json body")
//             .send()
//             .await?
//             .text()
//             .await?
//         }
//       };

//       leptos::on_cleanup( move || {
//         if let Some(abort_controller) = abort_controller {
//           abort_controller.abort()
//         }
//       });
//     }
//   }

//   // Return the error response json as an error
//   Response::de(&json).map_err(|_| LemmyAppError::APIError {
//     error: json_deser_err(&json),
//   })
// }

// fn build_route(route: &str) -> String {
//   format!("{ENDPOINT}/{route}")
// }

// fn build_fetch_query<T: Serialize>(path: &str, form: T) -> String {
//   let form_str = serde_urlencoded::to_string(&form).unwrap_or(path.to_string());
//   format!("{path}?{form_str}")
// }

#[cfg(not(feature = "ssr"))]
pub async fn get_cookie_wrapper(path: &str) -> Result<String, LemmyAppError> {
  use leptos::window;
  use crate::wasm_bindgen::JsCast;

  let r = window()
    .document()
    .ok_or(LemmyAppError::APIError {
      error: String::from("DOM document is None"),
    })?
    .dyn_into::<web_sys::HtmlDocument>()
    .map_err(|_| LemmyAppError::APIError {
      error: String::from("DOM document could not be cast"),
    })?
    .cookie()
    .map_err(|_| LemmyAppError::APIError {
      error: String::from("Cookie could not be set"),
    })?;

  wasm_cookies::cookies::get(&r, path)
    .ok_or(LemmyAppError::APIError {
      error: String::from("DOM cookie is None"),
    })?
    .map_err(|e| LemmyAppError::APIError {
      error: e.to_string(),
    })
}

#[cfg(not(feature = "ssr"))]
pub async fn set_cookie_wrapper(path: &str, value: &str) -> Result<(), LemmyAppError> {
  use leptos::window;
  use crate::wasm_bindgen::JsCast;
  use wasm_cookies::CookieOptions;

  let r = window()
    .document()
    .ok_or(LemmyAppError::APIError {
      error: String::from("DOM document is None"),
    })?
    .dyn_into::<web_sys::HtmlDocument>()
    .map_err(|_| LemmyAppError::APIError {
      error: String::from("DOM document could not be cast"),
    })?
    .set_cookie(&wasm_cookies::cookies::set(path, value, &CookieOptions::default())[..])
    .map_err(|_| LemmyAppError::APIError {
      error: String::from("Cookie could not be set"),
    });

  r
}

#[cfg(not(feature = "ssr"))]
pub async fn remove_cookie_wrapper(path: &str) -> Result<(), LemmyAppError> {
  use leptos::window;
  use crate::wasm_bindgen::JsCast;

  let r = window()
    .document()
    .ok_or(LemmyAppError::APIError {
      error: String::from("DOM document is None"),
    })?
    .dyn_into::<web_sys::HtmlDocument>()
    .map_err(|_| LemmyAppError::APIError {
      error: String::from("DOM document could not be cast"),
    })?
    .set_cookie(&wasm_cookies::cookies::delete(path))
    .map_err(|_| LemmyAppError::APIError {
      error: String::from("Cookie could not be set"),
    });

  r
}

#[cfg(feature = "ssr")]
pub async fn set_cookie_wrapper(path: &str, value: &str) -> Result<(), LemmyAppError> {
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
pub async fn remove_cookie_wrapper(path: &str) -> Result<(), LemmyAppError> {
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
pub async fn get_cookie_wrapper(path: &str) -> Result<String, LemmyAppError> {
  use actix_web::HttpRequest;
  use leptos_actix::extract;

  let path_string = path.to_string().clone();

  let cookie_value = extract(|req: HttpRequest| async move {
    if let Some(c) = req.cookie(&path_string) {
      let s = c.clone();
      s.value().to_string()
    } else {
      "".to_string()
    }
  })
  .await
  .map_err(|e| LemmyAppError::APIError {
    error: e.to_string(),
  })?;

  Ok(cookie_value)
}
