use crate::{
  constants::AUTH_COOKIE,
  contexts::site_resource_context::SiteResource,
  ui::components::common::text_input::{InputType, TextInput},
};
use leptos::{server_fn::error::NoCustomError, *};
use leptos_router::ActionForm;

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

#[component]
pub fn LoginForm() -> impl IntoView {
  let login = Action::<Login, _>::server();
  let site_resource = expect_context::<SiteResource>();
  // TODO: make unified, better looking way of handling errors.
  let login_error = Signal::derive(move || {
    login.value().get().and_then(|v| {
      v.map_err(|e| view! { <div class="text-error">{e.to_string()}</div> })
        .err()
    })
  });

  Effect::new(move |_| {
    if login.value().get().is_some_and(|r| r.is_ok()) {
      site_resource.refetch();
    }
  });

  view! {
    <ActionForm class="space-y-3" action=login>
      {login_error}
      <TextInput
        id="username"
        name="username_or_email"
        label="Username"
        required=true
        min_length=3
      />

      <TextInput
        id="password"
        name="password"
        label="Password"
        input_type=InputType::Password
        pattern=".{10,60}"
        required=true
      />

      <button class="btn btn-lg" type="submit">
        "Login"
      </button>
    </ActionForm>
  }
}
