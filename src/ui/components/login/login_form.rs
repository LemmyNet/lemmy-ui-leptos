use crate::{
  queries::site_state_query::use_site_state,
  ui::components::common::password_input::{InputType, PasswordInput},
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_query::QueryResult;
use leptos_router::{ActionForm, NavigateOptions};

#[server(LoginAction, "/serverfn")]
pub async fn login(username_or_email: String, password: String) -> Result<(), ServerFnError> {
  use crate::lemmy_client::LemmyClient;
  use actix_session::Session;
  use actix_web::web;
  use awc::Client;
  use lemmy_api_common::person::{Login, LoginResponse};
  use leptos_actix::{extract, redirect};

  extract(|client: web::Data<Client>, session: Session| async move {
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
  })
  .await?
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let name = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());

  // let button_is_disabled =
  //   Signal::derive(move || with!(|password, name| password.is_empty() || name.is_empty()));

  let login = create_server_action::<LoginAction>();
  let login_is_success = Signal::derive(move || login.value()().is_some_and(|res| res.is_ok()));

  let QueryResult { refetch, .. } = use_site_state();
  create_isomorphic_effect(move |_| {
    if login_is_success() {
      refetch();

      cfg_if! {
        if #[cfg(feature = "ssr")] {
          leptos_actix::redirect("/");
        } else {
          let navigate = leptos_router::use_navigate();

          navigate("/", NavigateOptions { replace: true, ..Default::default() })
        }
      }
    }
  });

  view! {
    <ActionForm class="space-y-3" action=login>
      <div class="form-control w-full relative">
        <input
          id="username"
          type="text"
          required
          name="username_or_email"
          class="peer input input-bordered border-x-0 border-t-0 rounded-b-none border-b-2 focus:outline-none bg-base-200/50"
          placeholder=" "
          value=name
          on:input=move |ev| update!(| name | * name = event_target_value(& ev))
        />
        <label
          class="label absolute inset-y-0 start-2 transition-all peer-placeholder-shown:text-neutral/50 peer-[:not(:placeholder-shown)]:-top-20 peer-focus:text-current peer-[:not(:placeholder-shown)]:start-0 peer-[:not(:placeholder-shown)]:text-sm peer-focus:text-sm peer-focus:-top-20 peer-focus:start-0 pointer-events-none select-none"
          for="username"
        >
          Username
        </label>
      </div>

      <PasswordInput
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
