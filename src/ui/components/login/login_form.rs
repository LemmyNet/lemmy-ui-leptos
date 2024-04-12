use crate::{
  constants::AUTH_COOKIE,
  contexts::site_resource_context::SiteResource,
  ui::components::common::text_input::{InputType, TextInput},
  utils::derive_user_is_logged_in,
};
use leptos::{server_fn::error::NoCustomError, *};
use leptos_router::{ActionForm, NavigateOptions, Redirect};

#[server(LoginAction, "/serverfn")]
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
fn LoginRedirect() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);

  view! {
    <Show when=user_is_logged_in>
      <Redirect
        path="/"
        options=NavigateOptions {
            replace: true,
            ..Default::default()
        }
      />

    </Show>
  }
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let login = Action::<LoginAction, _>::server();
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);

  Effect::new(move |_| {
    if login.version()() > 0 && !user_is_logged_in() {
      site_resource.refetch();
    }
  });

  view! {
    <Suspense>
      <LoginRedirect/>
    </Suspense>
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
