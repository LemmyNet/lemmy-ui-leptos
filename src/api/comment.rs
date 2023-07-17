use super::build_route;
use crate::api::fetch_api;
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use leptos::Scope;

pub async fn get_comments(cx: Scope, form: &GetComments) -> Option<GetCommentsResponse> {
  fetch_api::<GetCommentsResponse, GetComments>(cx, &build_route("comment/list"), form).await
}
