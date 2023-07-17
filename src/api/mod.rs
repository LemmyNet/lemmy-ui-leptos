use leptos::{Scope, Serializable};
use serde::Serialize;

pub mod comment;
pub mod post;

const ENDPOINT: &str = "https://voyager.lemmy.ml/api/v3";

pub fn build_route(route: &str) -> String {
  format!("{}/{}", ENDPOINT, route)
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<Response, Form>(cx: Scope, path: &str, form: &Form) -> Option<Response>
where
  Response: Serializable,
  Form: Serialize,
{
  let abort_controller = web_sys::AbortController::new().ok();
  let abort_signal = abort_controller.as_ref().map(|a| a.signal());
  let path_with_query = build_fetch_query(path, form);
  let json = gloo_net::http::Request::get(&path_with_query)
    .abort_signal(abort_signal.as_ref())
    .send()
    .await
    .map_err(|e| log::error!("{e}"))
    .ok()?
    .text()
    .await
    .ok()?;

  // abort in-flight requests if the Scope is disposed
  // i.e., if we've navigated away from this page
  leptos::on_cleanup(cx, move || {
    if let Some(abort_controller) = abort_controller {
      abort_controller.abort()
    }
  });
  Response::de(&json).ok()
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<Response, Form>(_cx: Scope, path: &str, form: &Form) -> Option<Response>
where
  Response: Serializable,
  Form: Serialize,
{
  let path_with_query = build_fetch_query(path, form);
  let client = reqwest::Client::new();
  let json = client
    .get(&path_with_query)
    .send()
    .await
    .map_err(|e| log::error!("{e}"))
    .ok()?
    .text()
    .await
    .ok()?;
  Response::de(&json).map_err(|e| log::error!("{e}")).ok()
}

fn build_fetch_query<T: Serialize>(path: &str, form: T) -> String {
  let form_str = serde_urlencoded::to_string(&form).unwrap_or(path.to_string());
  format!("{}?{}", path, form_str)
}
