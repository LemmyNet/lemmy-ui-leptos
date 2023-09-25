use crate::ui::components::common::password_input::PasswordInput;
use leptos::*;

#[server(LoginForm, "/serverfn")]
pub async fn login(username_or_email: String, password: String) -> Result<(), ServerFnError> {
  use crate::api::login::login;
  use lemmy_api_common::person::Login;
  log::debug!("Try to login with {username_or_email}");

  // let on_success = on_success.clone();
  async move {
    let form = Login {
      username_or_email: username_or_email.into(),
      password: password.into(),
      totp_2fa_token: None,
    };
    let res = login(&form).await?;

    // TODO figure out how to handle errors
    log::debug!("Login res: {:?}", res);
    // JWT can be extracted using into_inner()

    log::debug!("jwt: {:?}", res.jwt.unwrap().into_inner());

    Ok(())
  }
  .await
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let (password, set_password) = create_signal(String::new());
  let (name, set_name) = create_signal(String::new());

  let button_is_disabled =
    Signal::derive(move || password.with(|p| p.is_empty()) || name.with(|n| n.is_empty()));

  view! {
    <form class="space-y-3" on:submit=|ev| ev.prevent_default()>
      // {move || {
      //     error
      //         .get()
      //         .map(|err| {
      //             view! { <p style="color:red;">{err}</p> }
      //         })
      // }}
      <div class="form-control w-full">
        <label class="label" for="username">
          <span class="label-text">
            Username
          </span>
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
    </form>
  }
}
