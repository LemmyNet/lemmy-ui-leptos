use super::build_route;
use crate::api::fetch_api;
use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};
use leptos::Scope;

pub async fn list_posts(cx: Scope, form: &GetPosts) -> Option<GetPostsResponse> {
  fetch_api::<GetPostsResponse, GetPosts>(cx, &build_route("post/list"), form).await
}

pub async fn get_post(cx: Scope, form: &GetPost) -> Option<GetPostResponse> {
  fetch_api::<GetPostResponse, GetPost>(cx, &build_route("post"), form).await
}
