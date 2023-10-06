// use actix_web::web;

use lemmy_api_common::person::{LoginResponse, Login};
use leptos::{ev, logging::*, *};
// use leptos_actix::extract;
use leptos_router::ActionForm;

use crate::{errors::LemmyAppError, api::{api_wrapper, HttpType}};
// use leptos::wasm_bindgen::UnwrapThrowExt;


pub async fn login(form: &Login) -> Result<LoginResponse, LemmyAppError> {
  api_wrapper::<LoginResponse, Login>(HttpType::Post, "user/login", form).await
}


#[server(LoginFormFn, "/serverfn")]
pub async fn login_form_fn(
  username: String,
  password: String,
) -> Result<LoginResponse, ServerFnError> {
  // use crate::lemmy_client::LemmyClient;
  use crate::{api::set_cookie_wrapper, lemmy_client::LemmyClient};
  use actix_web::web;
  use awc::Client;
  use lemmy_api_common::person::Login;
  // use leptos::logging::log;
  use leptos_actix::extract;
  // use lemmy_api_common::person::Login;
  use leptos_actix::redirect;

  let form = Login {
    username_or_email: username.into(),
    password: password.into(),
    totp_2fa_token: None,
  };
  // let result: Result<LoginResponse, LemmyAppError> = Ok(LoginResponse { jwt: Some("lajdnaksdj".to_string().into()), registration_created: false, verify_email_sent: false });
  // let result =
  //   extract(|client: web::Data<Client>| async move { client.login(&form).await }).await?;
  let result = login(&form).await;
  // redirect("/");

  // let result = awc::Client::new().login(&form).await;

  match result {
    Ok(res) => {
      match set_cookie_wrapper("jwt", &res.jwt.clone().unwrap().into_inner()[..]).await {
        Ok(_) => Ok(res),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
      }
    },
    Err(err) => Err(ServerFnError::ServerError(err.to_string())),
  }
  // Ok(LoginResponse { jwt: Some("()".to_string().into()), registration_created: false, verify_email_sent: false })
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let (password, set_password) = create_signal(String::new());
  let (name, set_name) = create_signal(String::new());
  let (error, _set_error) = create_signal::<Option<String>>(None);
  let (disabled, _set_disabled) = create_signal(false);

  let _button_is_disabled =
    Signal::derive(move || disabled.get() || password.get().is_empty() || name.get().is_empty());

  let login_form_action = create_server_action::<LoginFormFn>();

  create_effect(move |_| {

    match login_form_action.value().get() {
      None => {
        leptos::logging::log!("none");
      },
      Some(Ok(o)) => {
        leptos::logging::log!("ok");
      },
      Some(Err(e)) => {
        leptos::logging::log!("error");
      },
    }

  });

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

      // <PasswordInput
      // id="password"
      // name="password"
      // on_input=move |s| set_password.update(|p| *p = s)
      // />

      <button type="submit">"Login"</button>
    </ActionForm>
  }
}
