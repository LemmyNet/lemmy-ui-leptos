use super::{build_route, HttpType};
use crate::api::api_wrapper;
use anyhow::Result;
use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};
use leptos::Scope;

pub async fn list_posts(cx: Scope, form: &GetPosts) -> Result<GetPostsResponse> {
  api_wrapper::<GetPostsResponse, GetPosts>(cx, HttpType::Get, "post/list", form).await
}

pub async fn get_post(cx: Scope, form: &GetPost) -> Result<GetPostResponse> {
  api_wrapper::<GetPostResponse, GetPost>(cx, HttpType::Get, &build_route("post"), form).await
}
