use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PostId,
    post::{PostResponse, SavePost as SavePostBody},
  },
  LemmyRequest,
};
use leptos::prelude::*;

#[server(prefix = "/serverfn")]
async fn save_post(id: PostId, save: bool) -> Result<PostResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .save_post(LemmyRequest {
      body: SavePostBody { post_id: id, save },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn create_save_post_action() -> ServerAction<SavePost> {
  ServerAction::new()
}
