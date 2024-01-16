use crate::{
  errors::{message_from_error, LemmyAppError, LemmyAppErrorType},
  i18n::*,
  queries::site_state_query::use_site_state,
  ui::components::common::text_input::{InputType, TextInput},
};
use lemmy_api_common::person::{Login, LoginResponse};
use leptos::*;
use leptos_query::QueryResult;
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

      let result = (Fetch {}).login(form).await;

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
  use actix_session::Session;
  use leptos_actix::{extract, redirect};

  let req = Login {
    username_or_email: username_or_email.into(),
    password: password.into(),
    totp_2fa_token: None,
  };

  let result = try_login(req).await;

  match result {
    Ok(LoginResponse { jwt, .. }) => {
      let cookie_res =
        extract(|session: Session| async move { session.insert("jwt", jwt.unwrap().into_inner()) })
          .await;

      match cookie_res {
        Ok(_o) => {
          redirect("/");
          Ok(())
        }
        Err(e) => Err(e),
      }
    }
    Err(e) => {
      redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let _i18n = use_i18n();

  let query = use_query_map();

  let ssr_error = move || query.with(|params| params.get("error").cloned());

  let name = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());

  let login = create_server_action::<LoginFn>();

  let error = create_rw_signal::<Option<String>>(None);
  let error_type = create_rw_signal::<String>("alert-error".into());

  let username_validation = create_rw_signal::<String>("".into());
  let password_validation = create_rw_signal::<String>("".into());

  // let QueryResult { .. } = use_site_state();

  if let Some(e) = ssr_error() {
    let le = serde_json::from_str::<LemmyAppError>(&e[..]);

    match le {
      Ok(e) => {
        error.set(Some(message_from_error(&e)));

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
      Err(_) => {
        logging::log!("ssr_e ERR");
      }
    }
  }

  let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    #[cfg(not(feature = "ssr"))]
    create_resource(
      move || (name, password),
      move |(name, password)| async move {
        let req = Login {
          username_or_email: name.get().into(),
          password: password.get().into(),
          totp_2fa_token: None,
        };

        let result = try_login(req.clone()).await;

        match result {
          Ok(LoginResponse { jwt, .. }) => {
            // #[cfg(not(feature = "ssr"))]
            // {
            use wasm_cookies::{cookies::CookieOptions, set};
            set(
              "jwt",
              &jwt.clone().unwrap().to_string()[..],
              &CookieOptions {
                same_site: wasm_cookies::cookies::SameSite::Strict,
                secure: true,
                expires: Some(std::borrow::Cow::Borrowed("Sat, 04 Jan 2025 19:24:51 GMT")),
                domain: None,
                path: None,
              },
            );
            // }

            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
          }
          Err(e) => {
            error.set(Some(message_from_error(&e)));
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
    <div class="w-full flex flex-col sm:flex-row flex-grow overflow-hidden">
      <main role="main" class="w-full h-full flex-grow p-3 overflow-auto">
        {move || {
            error
                .get()
                .map(|err| {
                    view! {
                      <div class=move || format!("alert {}", error_type.get())>
                        <span>{err}</span>
                      </div>
                    }
                })
        }}

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
      </main>
    </div>
  }
}
