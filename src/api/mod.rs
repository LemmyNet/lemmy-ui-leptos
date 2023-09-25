use crate::errors::LemmyAppError;
use cfg_if::cfg_if;
use leptos::Serializable;
use serde::Serialize;
use serde_json::Value;

pub mod comment;
pub mod login;
pub mod post;

const ENDPOINT: &str = "https://voyager.lemmy.ml/api/v3";

pub enum HttpType {
  Get,
  #[allow(dead_code)]
  Post,
  #[allow(dead_code)]
  Put,
}

/// Used if you hit a deser error, which usually means a LemmyAPI error
/// Of type {error: string}
fn json_deser_err(json: &str) -> String {
  serde_json::from_str(json)
    .map(|v: Value| v["error"].as_str().unwrap_or("Unknown").to_string())
    .unwrap_or("Unknown".to_string())
}

pub async fn api_wrapper<Response, Form>(
  type_: HttpType,
  path: &str,
  form: &Form,
) -> Result<Response, LemmyAppError>
where
  Response: Serializable,
  Form: Serialize,
{
  let route = &build_route(path);
  #[allow(clippy::needless_late_init)]
  let json;

  cfg_if! {
    if #[cfg(feature = "ssr")] {
     let client = reqwest::Client::new();

     json = match type_ {
       HttpType::Get => {
         client
           .get(&build_fetch_query(route, form))
           .send()
           .await?
           .text()
           .await?
       }
       HttpType::Post => client.post(path).json(form).send().await?.text().await?,
       HttpType::Put => client.put(path).json(form).send().await?.text().await?,
     };
   } else {
     use wasm_bindgen::UnwrapThrowExt;

     let abort_controller = web_sys::AbortController::new().ok();
     let abort_signal = abort_controller.as_ref().map(|a| a.signal());

     json = match type_ {
       HttpType::Get => {
         gloo_net::http::Request::get(&build_fetch_query(route, form))
           .abort_signal(abort_signal.as_ref())
           .send()
           .await?
           .text()
           .await?
       }
       HttpType::Post => {
         gloo_net::http::Request::post(route)
           .json(form)
           .expect_throw("Could not parse json body")
           .abort_signal(abort_signal.as_ref())
           .send()
           .await?
           .text()
           .await?
       }
       HttpType::Put => {
         gloo_net::http::Request::put(route)
           .json(form)
           .expect_throw("Could not parse json body")
           .abort_signal(abort_signal.as_ref())
           .send()
           .await?
           .text()
           .await?
       }
     };

     // abort in-flight requests if the Scope is disposed
     // i.e., if we've navigated away from this page
     leptos::on_cleanup( move || {
       if let Some(abort_controller) = abort_controller {
         abort_controller.abort()
       }
     });
   }
  }

  // Return the error response json as an error
  Response::de(&json).map_err(|_| LemmyAppError::APIError {
    error: json_deser_err(&json),
  })
}

fn build_route(route: &str) -> String {
  format!("{ENDPOINT}/{route}")
}

fn build_fetch_query<T: Serialize>(path: &str, form: T) -> String {
  let form_str = serde_urlencoded::to_string(&form).unwrap_or(path.to_string());
  format!("{path}?{form_str}")
}
