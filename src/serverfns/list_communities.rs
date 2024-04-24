use lemmy_client::lemmy_api_common::community::{
  ListCommunities as ListCommunitiesBody,
  ListCommunitiesResponse,
};
use leptos::{server_fn::codec::GetUrl, *};

#[server(prefix = "/serverfn", input = GetUrl, endpoint = "list_communities")]
pub async fn list_communities(
  body: ListCommunitiesBody,
) -> Result<ListCommunitiesResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .list_communities(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
