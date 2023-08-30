use super::HttpType;
use crate::{api::api_wrapper, errors::LemmyAppError};
use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};
use leptos::Scope;

pub async fn list_posts(
  cx: Option<Scope>,
  form: &GetPosts,
) -> Result<GetPostsResponse, LemmyAppError> {
  api_wrapper::<GetPostsResponse, GetPosts>(cx, HttpType::Get, "post/list", form).await
}

pub async fn get_post(cx: Option<Scope>, form: &GetPost) -> Result<GetPostResponse, LemmyAppError> {
  api_wrapper::<GetPostResponse, GetPost>(cx, HttpType::Get, "post", form).await
}
