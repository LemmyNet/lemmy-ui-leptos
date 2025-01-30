use crate::utils::types::ServerActionFn;
use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PostId,
    post::{CreatePostLike, PostResponse},
  },
  LemmyRequest,
};
use leptos::prelude::*;

#[server(prefix = "/serverfn")]
async fn vote_post(id: PostId, score: i16) -> Result<PostResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};

  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .like_post(LemmyRequest {
      body: CreatePostLike { post_id: id, score },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn create_vote_post_action() -> ServerAction<VotePost> {
  ServerAction::new()
}

impl ServerActionFn for VotePost {
  type Out = PostResponse;
}
