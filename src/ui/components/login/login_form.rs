// use actix_web::web;
use lemmy_api_common::person::LoginResponse;
use leptos::{ev, logging::*, *};
// use leptos_actix::extract;
use leptos_router::ActionForm;

use crate::ui::components::common::password_input::PasswordInput;

#[server(LoginFormFn, "/srv")]
pub async fn login_form_fn(
  username: String,
  password: String,
) -> Result<LoginResponse, ServerFnError> {
  // use crate::lemmy_client::LemmyClient;
  use crate::api::set_cookie_wrapper;
  // use lemmy_api_common::person::Login;
  use leptos_actix::redirect;
  use crate::lemmy_client::LemmyClient;
  use actix_web::web;
  use awc::Client;
  use lemmy_api_common::person::Login;
  // use leptos::logging::log;
  use leptos_actix::extract;


  let form = Login {
    username_or_email: username.into(),
    password: password.into(),
    totp_2fa_token: None,
  };
  let result = extract(|client: web::Data<Client>| async move { client.login(&form).await }).await?;
  // let result = login(&form).await;
  redirect("/");

  match result {
    Ok(res) => match set_cookie_wrapper("jwt", &res.jwt.clone().unwrap().into_inner()[..]).await {
      Ok(_) => Ok(res),
      Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    },
    Err(err) => Err(ServerFnError::ServerError(err.to_string())),
  }
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let (password, set_password) = create_signal(String::new());
  let (name, set_name) = create_signal(String::new());
  let (error, set_error) = create_signal::<Option<String>>(None);
  let (disabled, set_disabled) = create_signal(false);

  let button_is_disabled =
    Signal::derive(move || disabled.get() || password.get().is_empty() || name.get().is_empty());

  let login_form_action = create_server_action::<LoginFormFn>();

  view! {
    <ActionForm action=login_form_action>
      <p>"LoginForm"</p>
      {move || {
          error
              .get()
              .map(|err| {
                  view! { <p style="color:red;">{err}</p> }
              })
      }}

      <input
        name="username"
        type="text"
        required
        placeholder="Username"
        prop:disabled=move || disabled.get()
        on:keyup=move |ev: ev::KeyboardEvent| {
            let val = event_target_value(&ev);
            set_name.update(|v| *v = val);
        }

        on:change=move |ev| {
            let val = event_target_value(&ev);
            set_name.update(|v| *v = val);
        }
      />

      <input
        name="password"
        type="password"
        required
        placeholder="Password"
        prop:disabled=move || disabled.get()
        on:keyup=move |ev: ev::KeyboardEvent| {
            match &*ev.key() {
                _ => {
                    let val = event_target_value(&ev);
                    set_password.update(|p| *p = val);
                }
            }
        }
      />

      <PasswordInput
        id="password"
        name="password"
        on_input=move |s| set_password.update(|p| *p = s)
      />

      <button type="submit" prop:disabled=move || button_is_disabled.get()>
        "Login"
      </button>
    </ActionForm>
  }
}
