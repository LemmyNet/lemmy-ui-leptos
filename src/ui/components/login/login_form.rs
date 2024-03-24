use crate::{
  constants::AUTH_COOKIE,
  ui::{
    components::common::text_input::{InputType, TextInput},
    contexts::site_context::{SiteRefetchFn, UserLoggedIn},
  },
};
use leptos::{server_fn::error::NoCustomError, *};
use leptos_router::{ActionForm, NavigateOptions, Redirect};

#[server(LoginAction, "/serverfn")]
pub async fn login(username_or_email: String, password: String) -> Result<(), ServerFnError> {
  use crate::utils::get_client_and_session::get_client_and_session;
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
  let login = Action::<LoginAction, _>::server();
  let SiteRefetchFn(refetch) = expect_context::<SiteRefetchFn>();
  let user_is_logged_in = expect_context::<Signal<UserLoggedIn>>();

  Effect::new(move |_| {
    if login.version()() > 0 && !user_is_logged_in().0 {
      refetch();
    }
  });

  view! {
    <Show when=move || user_is_logged_in().0>
      <Redirect
        path="/"
        options=NavigateOptions {
            replace: true,
            ..Default::default()
        }
      />

    </Show>
    <ActionForm class="space-y-3" action=login>
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
