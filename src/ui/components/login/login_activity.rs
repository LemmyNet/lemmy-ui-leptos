use crate::{api::login::login, ui::components::login::login_form::LoginForm};
use lemmy_api_common::person::Login;
use leptos::*;

#[component]
pub fn LoginActivity(cx: Scope) -> impl IntoView {
  let (login_error, set_login_error) = create_signal(cx, None::<String>);
  let (wait_for_response, set_wait_for_response) = create_signal(cx, false);

  let login_action = create_action(cx, move |(name, password): &(String, String)| {
    log::debug!("Try to login with {name}");
    let name = name.to_string();
    let password = password.to_string();
    // let on_success = on_success.clone();
    async move {
      set_wait_for_response.update(|w| *w = true);
      let form = Login {
        username_or_email: name.into(),
        password: password.into(),
        totp_2fa_token: None,
      };
      let result = login(cx, &form).await;
      set_wait_for_response.update(|w| *w = false);
      // TODO figure out how to handle errors
      log::debug!("Login res: {:?}", result);
      // JWT can be extracted using into_inner()
      log::debug!("jwt: {:?}", result.unwrap().jwt.unwrap().into_inner());
      // match result {
      //   Ok(res) => {
      //     set_login_error.update(|e| *e = None);
      //     on_success(res);
      //   }
      //   Err(err) => {
      //     let msg = match err {
      //       api::Error::Fetch(js_err) => {
      //         format!("{js_err:?}")
      //       }
      //       api::Error::Api(err) => err.message,
      //     };
      //     error!("Unable to login with {}: {msg}", credentials.email);
      //     set_login_error.update(|e| *e = Some(msg));
      //   }
      // }
    }
  });

  let disabled = Signal::derive(cx, move || wait_for_response.get());

  view! { cx,
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Login Activity"</h2>
      <LoginForm action=login_action error=login_error.into() disabled/>

    </main>
  }
}
