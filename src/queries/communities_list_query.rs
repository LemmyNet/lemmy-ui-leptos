use lemmy_client::lemmy_api_common::community::{
  ListCommunities as ListCommunitiesBody,
  ListCommunitiesResponse,
};
use leptos::{server_fn::codec::GetUrl, *};
use leptos_query::{create_query, QueryOptions, QueryScope};

#[server(prefix = "/serverfn", input = GetUrl)]
async fn list_communities(
  body: ListCommunitiesBody,
) -> Result<ListCommunitiesResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

  // TODO: Update once I figure out how to get the custom error types working
  client
    .list_communities(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn use_communities_scope(
  options: QueryOptions<Result<ListCommunitiesResponse, ServerFnError>>,
) -> QueryScope<ListCommunitiesBody, Result<ListCommunitiesResponse, ServerFnError>> {
  create_query(list_communities, options)
}
