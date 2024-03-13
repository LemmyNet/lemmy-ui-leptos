use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{server_fn::codec::GetUrl, *};
use leptos_query::{create_query, QueryOptions, QueryScope, ResourceOption};
use std::time::Duration;

#[server(GetSiteResource, "/serverfn", input = GetUrl)]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get::<String>(AUTH_COOKIE)?;

  // TODO: Update once I figure out how to get the custom error types working
  client
    .get_site(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn use_site_state() -> QueryScope<(), Result<GetSiteResponse, ServerFnError>> {
  create_query(
    |_| async move { get_site().await },
    QueryOptions {
      resource_option: Some(ResourceOption::Blocking),
      stale_time: Some(Duration::from_secs(1800)),
      gc_time: Some(Duration::from_secs(3600)),
      ..QueryOptions::default()
    },
  )
}
