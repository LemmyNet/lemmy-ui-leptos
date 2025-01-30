use crate::constants::AUTH_COOKIE;
use leptos::prelude::{server_fn::error::NoCustomError, *};

#[server(prefix = "/serverfn")]
pub async fn login(username_or_email: String, password: String) -> Result<(), ServerFnError> {
  use crate::utils::get_client_and_session;
  use lemmy_client::lemmy_api_common::person::Login as LoginBody;

  let (client, session) = get_client_and_session().await?;

  let req = LoginBody {
    username_or_email: username_or_email.into(),
    password: password.into(),
    totp_2fa_token: None,
  };

  if let Some(jwt) = client
    .login(req)
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?
    .jwt
  {
    session.insert(AUTH_COOKIE, jwt.into_inner())?;
  }

  Ok(())
}

pub fn create_login_action() -> ServerAction<Login> {
  ServerAction::new()
}
