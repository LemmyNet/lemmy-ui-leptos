use crate::ui::components::common::password_input::PasswordInput;
use leptos::*;
use leptos_router::ActionForm;

#[server(LoginAction, "/serverfn")]
pub async fn login(username_or_email: String, password: String) -> Result<(), ServerFnError> {
  use crate::lemmy_client::{LemmyClient, LemmyRequest};
  use actix_session::Session;
  use actix_web::web;
  use awc::Client;
  use lemmy_api_common::person::{Login, LoginResponse};
  use leptos_actix::extract;

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

    Ok(())
  })
  .await?
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let (password, set_password) = create_signal(String::new());
  let (name, set_name) = create_signal(String::new());

  let button_is_disabled =
    Signal::derive(move || password.with(|p| p.is_empty()) || name.with(|n| n.is_empty()));

  let login = create_server_action::<LoginAction>();

  view! {
    <ActionForm class="space-y-3" action=login>
      // {move || {
      // error
      // .get()
      // .map(|err| {
      // view! { <p style="color:red;">{err}</p> }
      // })
      // }}
      <div class="form-control w-full">
        <label class="label" for="username">
          <span class="label-text">Username</span>
        </label>
        <input
          id="username"
          type="text"
          required
          name="username_or_email"
          class="input input-bordered"
          placeholder="Username"
          on:input=move |ev| set_name.update(|v| *v = event_target_value(&ev))
        />
      </div>

      <PasswordInput
        id="password"
        name="password"
        on_input=move |s| set_password.update(|p| *p = s)
      />

      <button class="btn btn-lg" type="submit" disabled=button_is_disabled>
        "Login"
      </button>
    </ActionForm>
  }
}
