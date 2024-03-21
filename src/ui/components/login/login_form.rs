use crate::{
  queries::site_state_query::use_site_refetch,
  ui::components::common::text_input::{InputType, TextInput},
};
use cfg_if::cfg_if;
use leptos::{server_fn::error::NoCustomError, *};
use leptos_router::ActionForm;

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
    session.insert("jwt", jwt.into_inner())?;
  }

  Ok(())
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let name = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());

  let login = Action::<LoginAction, _>::server();
  let refetch = use_site_refetch();

  Effect::new_isomorphic(move |_| {
    if login.version().get() > 0 {
      refetch();

      cfg_if! {
        if #[cfg(feature = "ssr")] {
          leptos_actix::redirect("/");
        } else {
          let navigate = leptos_router::use_navigate();

          navigate("/", leptos_router::NavigateOptions { replace: true, ..Default::default() })
        }
      }
    }
  });

  view! {
    <ActionForm class="space-y-3" action=login>
      <TextInput
        id="username"
        name="username_or_email"
        on_input=move |s| update!(| name | * name = s)
        label="Username"
      />

      <TextInput
        id="password"
        name="password"
        on_input=move |s| update!(| password | * password = s)
        label="Password"
        input_type=InputType::Password
      />

      <button class="btn btn-lg" type="submit">
        "Login"
      </button>
    </ActionForm>
  }
}
