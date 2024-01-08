use crate::{
  errors::{LemmyAppError, LemmyAppErrorType},
  i18n::*,
  lemmy_errors::LemmyErrorType,
  queries::site_state_query::use_site_state,
  ui::components::common::password_input::PasswordInput,
};
use cfg_if::cfg_if;
use lemmy_api_common::{
  person::{Login, LoginResponse},
  site::GetSiteResponse,
};
use leptos::*;
use leptos_i18n::t;
use leptos_query::{QueryResult, RefetchFn};
use leptos_router::*;
use wasm_cookies::CookieOptions;
use web_sys::SubmitEvent;

fn message_from_error(error: &LemmyAppError) -> String {
  let i18n = use_i18n();

  match error {
    LemmyAppError {
      error_type: LemmyAppErrorType::ApiError(LemmyErrorType::IncorrectLogin),
      ..
    } => t!(i18n, invalid_login)().to_string(),
    LemmyAppError {
      error_type: LemmyAppErrorType::EmptyUsername,
      ..
    } => t!(i18n, empty_username)().to_string(),
    LemmyAppError {
      error_type: LemmyAppErrorType::EmptyPassword,
      ..
    } => t!(i18n, empty_password)().to_string(),
    LemmyAppError {
      error_type: LemmyAppErrorType::Unknown,
      ..
    } => t!(i18n, unknown)().to_string(),
    _ => t!(i18n, unknown)().to_string(),
  }
}

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
          if let Some(jwt_string) = jwt {
            result
          //       let cookie_res =
          //         extract(|session: Session| async move { session.insert("jwt", jwt.into_inner()) })
          //           .await;

          //       match cookie_res {
          //         Ok(o) => {
          //           if is_ssr {
          //             redirect("/");
          //           }
          //           Ok(())
          //         }
          //         Err(e) => Err(e),
          //       }
          } else {
            Err(LemmyAppError {
              error_type: LemmyAppErrorType::MissingToken,
              content: format!("{:#?}", LemmyAppErrorType::MissingToken),
            })

            //       if is_ssr {
            //         redirect("/login?error=UnknownError");
            //         Ok(())
            //       } else {
            //         Err(ServerFnError::ServerError(serde_json::to_string(
            //           &LemmyAppErrorType::MissingToken,
            //         )?))
            //       }
          }
        }
        Err(e) => {
          Err(e)
          // LemmyAppError {
          //   error_type: e,
          //   content: format!("{:#?}", e),
          // },
          //     if is_ssr {
          //       redirect("/login?error=UnknownError");
          //       Ok(())
          //     } else {
          //       Err(ServerFnError::ServerError(serde_json::to_string(&e)?))
          //     }
        }
      }
    }
    Some(e) =>
    //Err(ServerFnError::ServerError(serde_json::to_string(
    // &
    {
      Err(LemmyAppError {
        error_type: e.clone(),
        content: format!("{:#?}", e),
      })
    }
    // )?)),
  }
}

#[server(LoginAction, "/serverfn")]
pub async fn login(
  username_or_email: String,
  password: String,
  is_ssr: bool,
) -> Result<(), ServerFnError> {
  use actix_session::Session;
  use leptos_actix::{extract, redirect};

  let req = Login {
    username_or_email: username_or_email.into(),
    password: password.into(),
    totp_2fa_token: None,
  };

  let result = try_login(req).await;

  // let val = validate(&req);

  // logging::log!("validation {:#?}", val);

  // match val {
  //   None => {
  //     use crate::lemmy_client::*;
  //     let result = (Fetch {}).login(req).await;

  //     logging::log!("russia {:#?}", result);

  match result {
    Ok(LoginResponse { jwt, .. }) => {
      // if let Some(jwt) = jwt {
      let cookie_res =
        extract(|session: Session| async move { session.insert("jwt", jwt.unwrap().into_inner()) })
          .await;

      match cookie_res {
        Ok(o) => {
          if is_ssr {
            redirect("/");
          }
          Ok(())
        }
        Err(e) => Err(e),
      }
      // } else {
      //   if is_ssr {
      //     redirect("/login?error=UnknownError");
      //     Ok(())
      //   } else {
      //     Err(ServerFnError::ServerError(serde_json::to_string(
      //       &LemmyAppErrorType::MissingToken,
      //     )?))
      // }
      // }
    }
    Err(e) => {
      if is_ssr {
        redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
        Ok(())
      } else {
        Err(ServerFnError::ServerError(serde_json::to_string(&e)?))
      }
    }
  }
  // }
  // Some(e) => Err(ServerFnError::ServerError(serde_json::to_string(
  //   &LemmyAppError {
  //     error_type: e.clone(),
  //     content: format!("{:#?}", e),
  //   },
  // )?)),
  //   }
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let i18n = use_i18n();

  // let params = use_params_map();
  let query = use_query_map();

  // id: || -> Option<String>
  let ssr_error = move || query.with(|params| params.get("error").cloned());

  let name = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());

  let login = create_server_action::<LoginAction>();
  // let login_is_success = Signal::derive(move || login.value()().is_some_and(|res| res.is_ok()));

  #[cfg(feature = "ssr")]
  let is_ssr = create_rw_signal::<bool>(true);
  #[cfg(not(feature = "ssr"))]
  let is_ssr = create_rw_signal::<bool>(false);

  // let QueryResult { refetch, .. } = expect_context::<QueryResult<Result<GetSiteResponse, ServerFnError>, RefetchFn>>();

  // let refetch = expect_context::<dyn RefetchFn>();

  let error = create_rw_signal::<Option<String>>(None);
  let error_type = create_rw_signal::<String>("alert-error".into());

  let username_validation = create_rw_signal::<String>("".into());
  let password_validation = create_rw_signal::<String>("".into());

  // query string reaction

  let QueryResult { refetch, .. } = use_site_state();

  // logging::log!("ssr {:#?}", ssr_error());

  // create_effect(move |_| match query.with(|params| params.get("error").cloned()) {
  //   None => {
  //     logging::log!("ssr_e NONE");
  //   }
  if let Some(e) = ssr_error() {
    let le = serde_json::from_str::<LemmyAppError>(&e[..]);
    // logging::log!("ssr_e SOME");

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
        // match e {
        //   LemmyAppErrorType::EmptyUsername  => {
        //     username_validation.set("input-error".to_string());
        //   }
        //   LemmyAppErrorType::EmptyPassword => {
        //     password_validation.set("input-error".to_string());
        //   }
        //   _ => {
        //   }
        // }
      }
      Err(_) => {
        logging::log!("ssr_e ERR");
      }
    }
  }
  // });

  // create_effect(move |_| match login.value().get() {
  //   None => {
  //   }
  //   Some(Ok(_o)) => {
  //     // authenticated.set(true);
  //     let navigate = leptos_router::use_navigate();
  //     navigate("/", Default::default());
  //   }
  //   Some(Err(ServerFnError::ServerError(e))) => {
  //     let le = serde_json::from_str::<LemmyAppError>(&e[..]);

  //     match le {
  //       // Ok(LemmyAppError {
  //       //   error_type: LemmyAppErrorType::ApiError(LemmyErrorType::IncorrectLogin),
  //       //   content: _,
  //       // }) => {
  //       //   error.set(Some(t!(i18n, invalid_login)().to_string()));
  //       // }
  //       // Ok(LemmyAppError {
  //       //   error_type: LemmyAppErrorType::EmptyUsername,
  //       //   content: _,
  //       // }) => {
  //       //   error.set(Some(t!(i18n, empty_username)().to_string()));
  //       // }
  //       // Ok(LemmyAppError {
  //       //   error_type: LemmyAppErrorType::EmptyPassword,
  //       //   content: _,
  //       // }) => {
  //       //   error.set(Some(t!(i18n, empty_password)().to_string()));
  //       // }
  //       // Ok(LemmyAppError {
  //       //   error_type: LemmyAppErrorType::Unknown,
  //       //   content: _,
  //       // }) => {
  //       //   error.set(Some("t!(i18n, unknown)().to_string()".into()));
  //       // }
  //       Ok(o) => {
  //         error.set(Some(error_message_from_error(&o)));
  //       }
  //       Err(g) => {
  //         error.set(Some(t!(i18n, unknown)().to_string()));
  //       }
  //     }
  //   }
  //   Some(Err(v)) => {
  //     error.set(Some(t!(i18n, unknown)().to_string()));
  //   }
  // });

  // create_effect(move |_| {
  //   if login_is_success() {
  //     refetch();
  //     logging::log!("REFETCH");

  //     // cfg_if! {
  //     //   if #[cfg(feature = "ssr")] {
  //     //     leptos_actix::redirect("/");
  //     //   } else {
  //         let navigate = leptos_router::use_navigate();

  //         navigate("/", leptos_router::NavigateOptions { replace: true, ..Default::default() })
  //       // }
  //     // }
  //   }
  // });

  let on_submit = move |ev: SubmitEvent| {
    // logging::log!("vallllyyyy");

    ev.prevent_default();

    // #[cfg(not(feature = "ssr"))]
    create_resource(
      move || (name, password),
      move |(name, password)| async move {
        let req = Login {
          username_or_email: name.get().into(),
          password: password.get().into(),
          totp_2fa_token: None,
        };

        let result = try_login(req.clone()).await;
        // result

        match result {
          Ok(LoginResponse { jwt, .. }) => {
            // if let Some(jwt_string) = jwt {

            #[cfg(not(feature = "ssr"))]
            {
              use wasm_cookies::set;
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
            }

            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());

            // let cookie_res =
            //   extract(|session: Session| async move { session.insert("jwt", jwt.into_inner()) })
            //     .await;

            // match cookie_res {
            //   Ok(o) => {
            //     if is_ssr {
            //       redirect("/");
            //     }
            //     Ok(())
            //   }
            //   Err(e) => Err(e),
            // }
            // } else {
            //   error.set(Some(error_message_from_error(&e)));

            // if is_ssr {
            //   redirect("/login?error=UnknownError");
            //   Ok(())
            // } else {
            //   Err(ServerFnError::ServerError(serde_json::to_string(
            //     &LemmyAppErrorType::MissingToken,
            //   )?))
            // }
            // }
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

            // if is_ssr {
            //   redirect("/login?error=UnknownError");
            //   Ok(())
            // } else {
            //   Err(ServerFnError::ServerError(serde_json::to_string(&e)?))
            // }
          }
        }
      },
    );

    // let soyeah = validate(&req);

    // if let Some(e) = soyeah.clone() {
    //   error.set(Some(error_message_from_error(&LemmyAppError { error_type: e, content: String::default() })));
    //   ev.prevent_default();
    // }

    // #[cfg(feature = "csr")]
    // if soyeah.is_none() {
    //   ev.prevent_default();

    //   logging::log!("yayayay");

    //   let loooogy = create_resource(move || (), move |()| async move {
    //     logging::log!("loggin in");
    //     let req = Login { username_or_email: name.get().into(), password: password.get().into(), totp_2fa_token: None };

    //     let result = {
    //       use crate::lemmy_client::*;
    //       Some((Fetch {}).login(req).await)
    //     };

    //     logging::log!("CSR result {:#?}", result);

    //     match result {
    //       Some(Ok(o)) => {
    //         logging::log!("CSR cookie {:#?}", o);
    //         use wasm_cookies::set;
    //         set("jwt", &o.jwt.clone().unwrap().to_string()[..], &CookieOptions { same_site: wasm_cookies::cookies::SameSite::Strict, secure: true, expires: Some(std::borrow::Cow::Borrowed("Sat, 04 Jan 2025 19:24:51 GMT")), domain: None, path: None } );

    //         let navigate = leptos_router::use_navigate();
    //         navigate("/", Default::default());
    //         Some(o)
    //       },
    //       Some(Err(e)) => {
    //         error.set(Some(error_message_from_error(&e)));
    //         None
    //       }
    //       _ => None,
    //     }

    //   });
    // }
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
    <ActionForm class="space-y-3" on:submit=on_submit action=login>
      <div class="form-control w-full">
        <label class="label" for="username">
          <span class="label-text">Username</span>
        </label>
        <input
          id="username"
          type="text"

          name="username_or_email"
          class=move || format!("input input-bordered {}", username_validation.get())
          placeholder="Username"
          value=name
          on:input=move |ev| update!(| name | *name = event_target_value(& ev))
        />
      </div>

      <PasswordInput
        id="password"
        name="password"
        validation_class=password_validation.into()
        on_input=move |s| update!(| password | *password = s)
      />

      <input name="is_ssr" type="hidden" value=move || format!("{}", is_ssr.get())/>

      <button class="btn btn-lg" type="submit">
        "Login"
      </button>
    </ActionForm>

      </main>
    </div>
  }
}
