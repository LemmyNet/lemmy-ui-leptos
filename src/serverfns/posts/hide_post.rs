use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PostId,
    post::HidePost as HidePostForm,
    SuccessResponse,
  },
  LemmyRequest,
};
use leptos::prelude::*;

#[server(prefix = "/serverfn")]
async fn hide_post(id: PostId, hide: bool) -> Result<SuccessResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .hide_post(LemmyRequest {
      body: HidePostForm {
        post_ids: Vec::from([id]),
        hide,
      },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn create_hide_post_action() -> ServerAction<HidePost> {
  ServerAction::new()
}

pub type HidePostAction = ServerAction<HidePost>;
