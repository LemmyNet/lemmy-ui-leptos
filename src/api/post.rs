use crate::api::{fetch_api, ENDPOINT};
use lemmy_api_common::post::GetPostsResponse;
use leptos::Scope;

pub fn posts() -> String {
  format!("{}/post/list", ENDPOINT)
}

pub async fn fetch_posts(cx: Scope) -> Option<GetPostsResponse> {
  fetch_api::<GetPostsResponse>(cx, &posts()).await
}
