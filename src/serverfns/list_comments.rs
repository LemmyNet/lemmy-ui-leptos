use lemmy_client::lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use leptos::{server_fn::codec::GetUrl, *};

#[server(prefix = "/serverfn", input = GetUrl)]
pub async fn list_comments(body: GetComments) -> Result<GetCommentsResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .list_comments(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
