use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PersonId,
    person::{BlockPerson, BlockPersonResponse},
  },
  LemmyRequest,
};
use leptos::prelude::*;

#[server(prefix = "/serverfn")]
async fn block_user(id: PersonId, block: bool) -> Result<BlockPersonResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;

  client
    .block_person(LemmyRequest {
      body: BlockPerson {
        person_id: id,
        block,
      },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn create_block_user_action() -> ServerAction<BlockUser> {
  ServerAction::new()
}
