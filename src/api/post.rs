use super::build_route;
use crate::api::fetch_api;
use lemmy_api_common::post::{GetPosts, GetPostsResponse};
use leptos::Scope;

pub async fn list_posts(cx: Scope, form: &GetPosts) -> Option<GetPostsResponse> {
  fetch_api::<GetPostsResponse, GetPosts>(cx, &build_route("post/list"), form).await
}
