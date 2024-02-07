use crate::{
  queries::site_state_query::use_site_state,
  ui::components::common::text_input::{InputType, TextInput},
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_query::QueryResult;
use leptos_router::ActionForm;

#[server(LoginAction, "/serverfn")]
pub async fn login(username_or_email: String, password: String) -> Result<(), ServerFnError> {
  use crate::lemmy_client::LemmyClient;
  use actix_session::Session;
  use actix_web::web;
  use lemmy_api_common::person::{Login, LoginResponse};
  use leptos_actix::{extract, redirect};

  let client = extract::<web::Data<awc::Client>>().await?;
  let session = extract::<Session>().await?;

  let req = Login {
    username_or_email: username_or_email.into(),
    password: password.into(),
    totp_2fa_token: None,
  };

  let LoginResponse { jwt, .. } = client.login(req).await?;
  if let Some(jwt) = jwt {
    session.insert("jwt", jwt.into_inner())?;
  }

  redirect("/");
  Ok(())
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let name = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());

  let login = create_server_action::<LoginAction>();
  let login_is_success = Signal::derive(move || login.value().get().is_some_and(|res| res.is_ok()));

  let QueryResult { refetch, .. } = use_site_state();

  create_isomorphic_effect(move |_| {
    if login_is_success.get() {
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
