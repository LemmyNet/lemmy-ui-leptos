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
pub fn LoginForm(
  cx: Scope,
  action: Action<(String, String), ()>,
  error: Signal<Option<String>>,
  disabled: Signal<bool>,
) -> impl IntoView {
  let (password, set_password) = create_signal(cx, String::new());
  let (name, set_name) = create_signal(cx, String::new());

  let dispatch_action = move || action.dispatch((name.get(), password.get()));

  let button_is_disabled = Signal::derive(cx, move || {
    disabled.get() || password.get().is_empty() || name.get().is_empty()
  });

  let login = create_server_action::<LoginForm>(cx);

  view! { cx,
    <ActionForm action=login>
      <p>"LoginForm"</p>
      {move || {
          error
              .get()
              .map(|err| {
                  view! { cx, <p style="color:red;">{err}</p> }
              })
      }}

      <input
        type="text"
        required
        name="username_or_email"
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
        type="password"
        required
        name="password"
        placeholder="Password"
        prop:disabled=move || disabled.get()
        on:keyup=move |ev: ev::KeyboardEvent| {
            match &*ev.key() {
                "Enter" => {
                    dispatch_action();
                }
                _ => {
                    let val = event_target_value(&ev);
                    set_password.update(|p| *p = val);
                }
            }
        }

        on:change=move |ev| {
            let val = event_target_value(&ev);
            set_password.update(|p| *p = val);
        }
      />

      <button prop:type="submit" prop:disabled=move || button_is_disabled.get() on:click=move |_| dispatch_action()>
        "Login"
      </button>
    </ActionForm>
  }
}
