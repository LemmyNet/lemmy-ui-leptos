use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{
  create_blocking_resource,
  provide_context,
  server,
  server_fn::codec::GetUrl,
  Resource,
  ServerFnError,
};

#[server(prefix = "/serverfn", input = GetUrl, endpoint = "get_site")]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .get_site(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub type SiteResource = Resource<(), Result<GetSiteResponse, ServerFnError>>;

pub fn provide_site_resource_context() {
  let site_resource = create_blocking_resource(|| (), |_| get_site());

  provide_context(site_resource);
}
