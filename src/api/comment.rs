use super::HttpType;
use crate::{api::api_wrapper, errors::LemmyAppError};
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use leptos::Scope;

pub async fn get_comments(
  cx: Option<Scope>,
  form: &GetComments,
) -> Result<GetCommentsResponse, LemmyAppError> {
  api_wrapper::<GetCommentsResponse, GetComments>(cx, HttpType::Get, "comment/list", form).await
}
