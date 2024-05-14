use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{server_fn::codec::GetUrl, *};

#[server(prefix = "/serverfn", input = GetUrl)]
pub async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get_jwt()?;

  client
    .get_site(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
