use crate::utils::GetJwt;
use lemmy_client::lemmy_api_common::post::{GetPosts, GetPostsResponse};
use leptos::{server, server_fn::codec::GetUrl, ServerFnError};

#[server(prefix = "/serverfn", input = GetUrl)]
pub async fn list_posts(body: GetPosts) -> Result<GetPostsResponse, ServerFnError> {
  use crate::utils::get_client_and_session;
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get_jwt()?;

  client
    .list_posts(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
