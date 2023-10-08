use crate::{
  api::{api_wrapper, HttpType},
  errors::LemmyAppError,
};
use lemmy_api_common::person::{Login, LoginResponse};
use leptos::{ev, logging::*, *};
// use crate::use_query_map;
use leptos_router::use_query_map;
use leptos_router::ActionForm;

pub async fn login(form: &Login) -> Result<LoginResponse, LemmyAppError> {
  api_wrapper::<LoginResponse, Login>(HttpType::Post, "user/login", form).await
}

#[server(LoginFormFn, "/serverfn")]
pub async fn login_form_fn(
  username: String,
  password: String,
  is_ssr_only: bool,
) -> Result<LoginResponse, ServerFnError> {
  use crate::{api::set_cookie_wrapper, lemmy_client::LemmyClient};
  use actix_web::web;
  use awc::Client;
  use lemmy_api_common::person::Login;
  use leptos_actix::{extract, redirect};

  let form = Login {
    username_or_email: username.into(),
    password: password.into(),
    totp_2fa_token: None,
  };
  // let result =
  //   extract(|client: web::Data<Client>| async move { client.login(&form).await }).await?;
  let result = login(&form).await;

  match result {
    Ok(res) => match set_cookie_wrapper("jwt", &res.jwt.clone().unwrap().into_inner()[..]).await {
      Ok(_) => {
        if is_ssr_only {
          redirect("/");
        }
        Ok(res)
      }
      Err(e) => {
        if is_ssr_only {
          redirect("/login?error=1");
        }
        Err(ServerFnError::ServerError(e.to_string()))
      }
    },
    Err(err) => {
      if is_ssr_only {
        redirect("/login?error=1");
        Ok(LoginResponse {
          jwt: None,
          registration_created: false,
          verify_email_sent: false,
        })
      } else {
        Err(ServerFnError::ServerError(err.to_string()))
      }
    }
  }
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let (password, set_password) = create_signal(String::new());
  let (name, set_name) = create_signal(String::new());
  let error = create_rw_signal::<Option<String>>(None);
  let (disabled, _set_disabled) = create_signal(false);

  let is_ssr_only = create_rw_signal::<bool>(true);

  let query = use_query_map();
  error.set(
    match query
      .with(|q| q.get("error").and_then(|page| page.parse::<i64>().ok()))
      .unwrap_or(0)
    {
      1 => Some(
        "An error occurred while trying to login. Please try again or come back later.".to_string(),
      ),
      _x => None,
    },
  );

  #[cfg(not(feature = "ssr"))]
  is_ssr_only.set(false);

  let _button_is_disabled =
    Signal::derive(move || disabled.get() || password.get().is_empty() || name.get().is_empty());

  let login_form_action = create_server_action::<LoginFormFn>();

  let authenticated = use_context::<RwSignal<bool>>().unwrap_or(create_rw_signal(false));

  create_effect(move |_| match login_form_action.value().get() {
    None => {}
    Some(Ok(_o)) => {
      authenticated.set(true);
      let navigate = leptos_router::use_navigate();
      navigate("/", Default::default());
    }
    Some(Err(e)) => {
      error.set(Some(e.to_string()));
    }
  });

  view! {
    {move || {
        error
            .get()
            .map(|err| {
                view! {
                  <div class="alert alert-error">
                    <span>{err}</span>
                  </div>
                }
            })
    }}

    <ActionForm action=login_form_action>
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

        class="input input-bordered"
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

        class="input input-bordered"
      />
      <input name="is_ssr_only" type="hidden" value=move || format!("{}", is_ssr_only.get())/>
      <button type="submit" class="btn">
        "Login"
      </button>
    </ActionForm>
  }
}
