use crate::ui::components::login::login_form::LoginForm;
use leptos::{logging::*, *};

#[component]
pub fn LoginActivity() -> impl IntoView {
  let (login_error, _set_login_error) = create_signal(None::<String>);
  let (wait_for_response, _set_wait_for_response) = create_signal(false);

  // let login_action = create_action(move |(name, password): &(String, String)| {
  //   log::debug!("Try to login with {name}");
  //   let name = name.to_string();
  //   let password = password.to_string();
  //   // let on_success = on_success.clone();
  //   async move {
  //     set_wait_for_response.update(|w| *w = true);
  //     let form = Login {
  //       username_or_email: name.into(),
  //       password: password.into(),
  //       totp_2fa_token: None,
  //     };
  //     let result = login(&form).await;
  //     set_wait_for_response.update(|w| *w = false);
  //     // TODO figure out how to handle errors
  //     log::debug!("Login res: {:?}", result);
  //     // JWT can be extracted using into_inner()
  //     match result {
  //       Ok(res) => {
  //         log::debug!("jwt: {:?}", res.jwt.unwrap().into_inner());
  //         set_login_error.update(|e| *e = None);
  //         // on_success(res);
  //       }
  //       Err(err) => {
  //         let err_str = err.to_string();
  //         error!(
  //           "Unable to login with {}: {}",
  //           form.username_or_email.into_inner(),
  //           err_str,
  //         );
  //         set_login_error.update(|e| *e = Some(err_str));
  //       }
  //     }
  //   }
  // });

  let disabled = Signal::derive(move || wait_for_response.get());

  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Login Activity"</h2>
      <LoginForm error=login_error.into() disabled/>
    </main>
  }
}
