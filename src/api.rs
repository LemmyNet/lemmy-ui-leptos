use lemmy_api_common::post::GetPostsResponse;
use leptos::{Scope, Serializable};

const ENDPOINT: &str = "https://voyager.lemmy.ml/api/v3";

pub fn posts() -> String {
  format!("{}/post/list", ENDPOINT)
}

pub async fn fetch_posts(cx: Scope) -> Option<GetPostsResponse> {
  fetch_api::<GetPostsResponse>(cx, &posts()).await
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(cx: Scope, path: &str) -> Option<T>
where
  T: Serializable,
{
  let abort_controller = web_sys::AbortController::new().ok();
  let abort_signal = abort_controller.as_ref().map(|a| a.signal());

  let json = gloo_net::http::Request::get(path)
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
  T::de(&json).ok()
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(_cx: Scope, path: &str) -> Option<T>
where
  T: Serializable,
{
  let json = reqwest::get(path)
    .await
    .map_err(|e| log::error!("{e}"))
    .ok()?
    .text()
    .await
    .ok()?;
  T::de(&json).map_err(|e| log::error!("{e}")).ok()
}
