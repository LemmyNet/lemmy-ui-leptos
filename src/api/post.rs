use super::HttpType;
use crate::{api::api_wrapper, errors::LemmyAppError};
use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};

pub async fn list_posts(form: &GetPosts) -> Result<GetPostsResponse, LemmyAppError> {
  api_wrapper::<GetPostsResponse, GetPosts>(HttpType::Get, "post/list", form).await
}

pub async fn get_post(form: &GetPost) -> Result<GetPostResponse, LemmyAppError> {
  api_wrapper::<GetPostResponse, GetPost>(HttpType::Get, "post", form).await
}
