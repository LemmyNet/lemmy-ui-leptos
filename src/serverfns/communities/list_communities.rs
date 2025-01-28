use lemmy_client::lemmy_api_common::community::{
  ListCommunities as ListCommunitiesBody,
  ListCommunitiesResponse,
};
use leptos::prelude::{server_fn::codec::GetUrl, *};

#[server(prefix = "/serverfn", input = GetUrl)]
pub async fn list_communities(
  body: ListCommunitiesBody,
) -> Result<ListCommunitiesResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .list_communities(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
