pub use lemmy_client::lemmy_api_common::community::{
  CommunityResponse,
  CreateCommunity as CreateCommunityBody,
};
use leptos::prelude::{server, server_fn::codec::PostUrl, ServerFnError};

#[server(prefix = "/serverfn", input = PostUrl)]
pub async fn create_community(
  body: CreateCommunityBody,
) -> Result<CommunityResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .create_community(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
