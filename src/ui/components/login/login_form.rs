use crate::ui::components::common::password_input::PasswordInput;
use leptos::{ev, *};
use leptos_router::ActionForm;

#[server(LoginForm, "/serverfn")]
pub async fn login(
  cx: Scope,
  username_or_email: String,
  password: String,
) -> Result<(), ServerFnError> {
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
    let res = login(cx, &form).await?;

    // TODO figure out how to handle errors
    log::debug!("Login res: {:?}", res);
    // JWT can be extracted using into_inner()

    log::debug!("jwt: {:?}", res.jwt.unwrap().into_inner());

    Ok(())
  }
  .await
}

#[component]
pub fn LoginForm(cx: Scope) -> impl IntoView {
  let (password, set_password) = create_signal(cx, String::new());
  let (name, set_name) = create_signal(cx, String::new());

  let button_is_disabled = Signal::derive(cx, move || {
    password.with(|p| p.is_empty()) || name.with(|n| n.is_empty())
  });

  let login = create_server_action::<LoginForm>(cx);

  view! { cx,
    <ActionForm action=login class="mx-auto w-full lg:w-1/3 space-y-3">
      {move || {
          if let Some(Err(err)) = login.value().get() {
              Some(view! { cx, <p style="color:red;">{err.to_string()}</p> })
          } else {
              None
          }
      }}

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
          on:keyup=move |ev: ev::KeyboardEvent| {
              let val = event_target_value(&ev);
              set_name.update(|v| *v = val);
          }

          on:change=move |ev| {
              let val = event_target_value(&ev);
              set_name.update(|v| *v = val);
          }
        />

      </div>

      <PasswordInput
        id="password"
        name="password"
        on_input=move |s| set_password.update(|p| *p = s)
      />

      <button class="btn btn-lg" prop:type="submit" prop:disabled=button_is_disabled>
        "Login"
      </button>
    </ActionForm>
  }
}
