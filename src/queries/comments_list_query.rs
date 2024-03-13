use lemmy_client::lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use leptos::{server_fn::codec::GetUrl, *};
use leptos_query::{create_query, QueryOptions, QueryScope};
use std::time::Duration;

#[server(prefix = "/serverfn", input = GetUrl)]
async fn list_comments(body: GetComments) -> Result<GetCommentsResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

  // TODO: Update once I figure out how to get the custom error types working
  client
    .list_comments(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn use_comments_scope() -> QueryScope<GetComments, Result<GetCommentsResponse, ServerFnError>> {
  create_query(
    list_comments,
    QueryOptions {
      stale_time: Some(Duration::from_secs(90)),
      ..Default::default()
    },
  )
}
