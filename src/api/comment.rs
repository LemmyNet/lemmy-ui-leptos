use super::HttpType;
use crate::api::api_wrapper;
use anyhow::Result;
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use leptos::Scope;

pub async fn get_comments(cx: Scope, form: &GetComments) -> Result<GetCommentsResponse> {
  api_wrapper::<GetCommentsResponse, GetComments>(cx, HttpType::Get, "comment/list", form).await
}
