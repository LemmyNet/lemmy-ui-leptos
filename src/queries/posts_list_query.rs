use lemmy_client::lemmy_api_common::post::{GetPosts, GetPostsResponse};
use leptos::{server, server_fn::codec::GetUrl, ServerFnError};
use leptos_query::{create_query, QueryOptions, QueryScope};

#[server(prefix = "/serverfn", input = GetUrl)]
async fn list_posts(body: GetPosts) -> Result<GetPostsResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .list_posts(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn use_posts() -> QueryScope<GetPosts, Result<GetPostsResponse, ServerFnError>> {
  create_query(list_posts, QueryOptions::default())
}
