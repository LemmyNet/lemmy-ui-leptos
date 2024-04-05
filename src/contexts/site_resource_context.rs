use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{
  create_blocking_resource,
  provide_context,
  server,
  server_fn::codec::GetUrl,
  with,
  Resource,
  ServerFnError,
  Signal,
};

#[server(prefix = "/serverfn", input = GetUrl)]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
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
  let site_resource = create_blocking_resource(|| (), |_| async { get_site().await });
  let user_is_logged_in = Signal::derive(move || {
    with!(|site_resource| site_resource
      .as_ref()
      .and_then(|data| data.as_ref().ok())
      .map_or(false, |s| s.my_user.is_some()))
  });

  provide_context(site_resource);
  provide_context(user_is_logged_in);
}
