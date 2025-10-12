use lemmy_client::LemmyRequest;
use leptos::prelude::*;

#[server(prefix = "/serverfn")]
async fn logout() -> Result<(), ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;
  client
    .logout(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(ServerFnError::new)?;

  session.purge();
  Ok(())
}

pub fn create_logout_action() -> ServerAction<Logout> {
  ServerAction::new()
}
