use crate::{
  cookie::set_cookie,
  errors::{LemmyAppError, LemmyAppErrorType},
  i18n::*,
  ui::components::common::text_input::{InputType, TextInput},
};
use lemmy_api_common::person::{Login, LoginResponse};
use leptos::*;
use leptos_router::*;
use web_sys::SubmitEvent;

fn validate_login(form: &Login) -> Option<LemmyAppErrorType> {
  if form.username_or_email.len() == 0 {
    return Some(LemmyAppErrorType::EmptyUsername);
  }
  if form.password.len() == 0 {
    return Some(LemmyAppErrorType::EmptyPassword);
  }
  None
}

async fn try_login(form: Login) -> Result<LoginResponse, LemmyAppError> {
  let val = validate_login(&form);

  match val {
    None => {
      use crate::lemmy_client::*;

      let result = LemmyClient.login(form).await;

      match result {
        Ok(LoginResponse { ref jwt, .. }) => {
          if let Some(_jwt_string) = jwt {
            result
          } else {
            Err(LemmyAppError {
              error_type: LemmyAppErrorType::MissingToken,
              content: format!("{:#?}", LemmyAppErrorType::MissingToken),
            })
          }
        }
        Err(e) => Err(e),
      }
    }
    Some(e) => Err(LemmyAppError {
      error_type: e.clone(),
      content: format!("{:#?}", e),
    }),
  }
}

#[server(LoginFn, "/serverfn")]
pub async fn login(username_or_email: String, password: String) -> Result<(), ServerFnError> {
  use leptos_actix::redirect;

  let req = Login {
    username_or_email: username_or_email.into(),
    password: password.into(),
    totp_2fa_token: None,
  };

  let result = try_login(req).await;

  match result {
    Ok(LoginResponse { jwt, .. }) => {
      let r = set_cookie(
        "jwt",
        &jwt.unwrap_or_default().into_inner(),
        &core::time::Duration::from_secs(604800),
      )
      .await;
      match r {
        Ok(_o) => {
          // redirect("/");
          Ok(())
        }
        Err(e) => {
          // redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
          Ok(())
        }
      }
    }
    Err(e) => {
      // redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let _i18n = use_i18n();

  let query = use_query_map();

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();
  let user = expect_context::<RwSignal<Option<bool>>>();

  let name = create_rw_signal(String::new());
  let password = create_rw_signal(String::new());

  let login = create_server_action::<LoginFn>();

  let username_validation = create_rw_signal::<String>("".into());
  let password_validation = create_rw_signal::<String>("".into());

  let ssr_error = move || query.with(|params| params.get("error").cloned());

  if let Some(e) = ssr_error() {
    let le = serde_json::from_str::<LemmyAppError>(&e[..]);

    match le {
      Ok(e) => match e.error_type {
        LemmyAppErrorType::EmptyUsername => username_validation.set("input-error".to_string()),
        LemmyAppErrorType::EmptyPassword => password_validation.set("input-error".to_string()),
        _ => {}
      },
      Err(_) => {}
    }
  }

  let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();
    error.set(None);

    create_local_resource(
      move || (name.get(), password.get()),
      move |(name, password)| async move {
        let req = Login {
          username_or_email: name.into(),
          password: password.into(),
          totp_2fa_token: None,
        };
        let result = try_login(req.clone()).await;
        match result {
          Ok(LoginResponse { jwt: Some(jwt), .. }) => {
            let _ = set_cookie(
              "jwt",
              &jwt.clone().into_inner(),
              &core::time::Duration::from_secs(604800),
            )
            .await;
            user.set(Some(true));
            leptos_router::use_navigate()("/", Default::default());
          }
          Ok(LoginResponse { jwt: None, .. }) => {
            error.set(Some(LemmyAppError {
              error_type: LemmyAppErrorType::MissingToken,
              content: String::default(),
            }));
          }
          Err(e) => {
            error.set(Some(e.clone()));
            password_validation.set("".to_string());
            username_validation.set("".to_string());

            match e {
              LemmyAppError {
                error_type: LemmyAppErrorType::EmptyUsername,
                ..
              } => {
                username_validation.set("input-error".to_string());
              }
              LemmyAppError {
                error_type: LemmyAppErrorType::EmptyPassword,
                ..
              } => {
                password_validation.set("input-error".to_string());
              }
              _ => {}
            }
          }
        }
      },
    );
  };

  view! {
    <ActionForm class="space-y-3" action=login on:submit=on_submit>
      <TextInput
        id="username"
        name="username_or_email"
        on_input=move |s| update!(| name | * name = s)
        label="Username"
      />
      <TextInput
        id="password"
        name="password"
        validation_class=password_validation.into()
        on_input=move |s| update!(| password | * password = s)
        input_type=InputType::Password
        label="Password"
      />
      <button class="btn btn-lg" type="submit">
        "Login"
      </button>
    </ActionForm>
  }
}
