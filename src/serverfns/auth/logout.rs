use crate::utils::types::ServerAction;
use lemmy_client::LemmyRequest;
use leptos::{server_fn::error::NoCustomError, *};

#[server(prefix = "/serverfn")]
async fn logout() -> Result<(), ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get_jwt()?;
  client
    .logout(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

  session.purge();
  Ok(())
}

pub fn create_logout_action() -> ServerAction<Logout> {
  Action::server()
}
