use anyhow::{anyhow, Result};
use leptos::{Scope, Serializable};
use serde::Serialize;

pub mod comment;
pub mod login;
pub mod post;

const ENDPOINT: &str = "https://voyager.lemmy.ml/api/v3";

pub enum HttpType {
  Get,
  Post,
  Put,
}

#[cfg(not(feature = "ssr"))]
pub async fn api_wrapper<Response, Form>(
  cx: Scope,
  type_: HttpType,
  path: &str,
  form: &Form,
) -> Result<Response>
where
  Response: Serializable,
  Form: Serialize,
{
  use wasm_bindgen::UnwrapThrowExt;

  let route = &build_route(path);
  let abort_controller = web_sys::AbortController::new().ok();
  let abort_signal = abort_controller.as_ref().map(|a| a.signal());

  let json = match type_ {
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
  leptos::on_cleanup(cx, move || {
    if let Some(abort_controller) = abort_controller {
      abort_controller.abort()
    }
  });

  // Return the error response json as an error
  Response::de(&json).map_err(|_| anyhow!(json.clone()))
}

#[cfg(feature = "ssr")]
pub async fn api_wrapper<Response, Form>(
  _cx: Scope,
  type_: HttpType,
  path: &str,
  form: &Form,
) -> Result<Response>
where
  Response: Serializable,
  Form: Serialize,
{
  let route = &build_route(path);
  let client = reqwest::Client::new();

  let json = match type_ {
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

  // Return the error response json as an error
  Response::de(&json).map_err(|_| anyhow!(json.clone()))
}

fn build_route(route: &str) -> String {
  format!("{}/{}", ENDPOINT, route)
}

fn build_fetch_query<T: Serialize>(path: &str, form: T) -> String {
  let form_str = serde_urlencoded::to_string(&form).unwrap_or(path.to_string());
  format!("{}?{}", path, form_str)
}
