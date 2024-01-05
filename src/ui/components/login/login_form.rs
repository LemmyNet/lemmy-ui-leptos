use crate::{
  queries::site_state_query::use_site_state,
  ui::components::common::password_input::PasswordInput, lemmy_errors::LemmyErrorType, i18n::*, errors::{LemmyAppError, LemmyAppErrorType},
};
use cfg_if::cfg_if;
use lemmy_api_common::{site::GetSiteResponse, person::{Login, LoginResponse}};
use leptos::*;
use leptos_i18n::t;
use leptos_query::{QueryResult, RefetchFn};
use leptos_router::*;
use wasm_cookies::CookieOptions;
use web_sys::SubmitEvent;

#[server(LoginAction, "/serverfn")]
pub async fn login(username_or_email: String, password: String, is_ssr: bool) -> Result<(), ServerFnError> {
  use actix_session::Session;
  use leptos_actix::{extract, redirect};

  let req = Login {
    username_or_email: username_or_email.into(),
    password: password.into(),
    totp_2fa_token: None,
  };

  let val = validate(&req);

  logging::log!("validation {:#?}", val);

  match val {
      None => {

        use crate::lemmy_client::*;
        let result = (Fetch {}).login(req).await;

        logging::log!("russia {:#?}", result);

        match result {
          Ok(LoginResponse { jwt, .. }) => {
            if let Some(jwt) = jwt {
              let cookie_res = extract(|session: Session| async move {
                session.insert("jwt", jwt.into_inner())
              })
              .await;
      
              match cookie_res {
                Ok(o) => {
                  if is_ssr {
                    redirect("/");
                  }
                  Ok(())
                },
                Err(e) => {
                  Err(e)
                  // LemmyAppErrorType::InternalServerError.into()
                },
              }
            } else {
              if is_ssr {
                redirect("/login?error=UnknownError");
                Ok(())
              } else {
                Err(ServerFnError::ServerError(serde_json::to_string(&LemmyAppErrorType::MissingToken)?))
              }
            }
          
          },
          // Err(ServerFnError::ServerError(e)) => {
          //   logging::log!("function server error contents {e}");
          //   if is_ssr {
          //     redirect(&format!("/login?error={}", e)[..]);
          //     Ok(())
          //   } else {
          //     Err(ServerFnError::ServerError(e))
          //   }
          // },
          Err(e) => {
            if is_ssr {
              redirect("/login?error=UnknownError");
              Ok(())
            } else {
              Err(ServerFnError::ServerError(serde_json::to_string(&e)?))
            }
          }
        }
      
      
      },
      Some(e) => {
        Err(ServerFnError::ServerError(serde_json::to_string(&LemmyAppError { error_type: e.clone(), content: format!("{:#?}", e) })?))
      }
  }
}

fn validate(form: &Login) -> Option<LemmyAppErrorType> {
  if form.username_or_email.len() == 0 {
    return Some(LemmyAppErrorType::EmptyUsername);
  }
  if form.password.len() == 0 {
    return Some(LemmyAppErrorType::EmptyPassword);
  }
  None
}

#[component]
pub fn LoginForm() -> impl IntoView {
  let i18n = use_i18n();

  let name = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());

  let login = create_server_action::<LoginAction>();
  // let login_is_success = Signal::derive(move || login.value()().is_some_and(|res| res.is_ok()));

  let QueryResult { refetch, .. } = use_site_state();

  #[cfg(feature = "ssr")]
  let is_ssr = create_rw_signal::<bool>(true);
  #[cfg(not(feature = "ssr"))]
  let is_ssr = create_rw_signal::<bool>(false);


  // let QueryResult { refetch, .. } = expect_context::<QueryResult<Result<GetSiteResponse, ServerFnError>, RefetchFn>>();

  // let refetch = expect_context::<dyn RefetchFn>();

  let error = create_rw_signal::<Option<String>>(None);
  let error_type = create_rw_signal::<String>("alert-error".into());

  create_effect(move |_| match login.value().get() {
    None => {
      logging::log!("none");
    }
    Some(Ok(_o)) => {
      logging::log!("ok {:#?}", _o);
      // authenticated.set(true);
      let navigate = leptos_router::use_navigate();
      navigate("/", Default::default());
    }
    Some(Err(ServerFnError::ServerError(e))) => {
      let le = serde_json::from_str::<LemmyAppError>(&e[..]);
      // let le = serde_json::from_str::<LemmyErrorType>(&e[..]);
      // logging::log!("server error contents {e} {}", serde_json::to_string(&LemmyAppErrorType::ApiError(LemmyErrorType::IncorrectLogin)).ok().unwrap());

      // logging::log!("component server error contents {e} {}", serde_json::to_string(&LemmyAppErrorType::ApiError(LemmyErrorType::IncorrectLogin)).ok().unwrap());

      match le {
        // Ok(LemmyAppError { error_type: LemmyAppErrorType::ApiError{ inner: Some(LemmyErrorType::IncorrectLogin) }}) => {
        Ok(LemmyAppError { error_type: LemmyAppErrorType::ApiError(LemmyErrorType::IncorrectLogin), content: _ }) => {
          error.set(Some(t!(i18n, invalid_login)().to_string()));
        },
        Ok(LemmyAppError { error_type: LemmyAppErrorType::EmptyUsername, content: _ }) => {
          error.set(Some(t!(i18n, empty_username)().to_string()));
        },
        Ok(LemmyAppError { error_type: LemmyAppErrorType::EmptyPassword, content: _ }) => {
          error.set(Some(t!(i18n, empty_password)().to_string()));
        },
        Ok(LemmyAppError { error_type: LemmyAppErrorType::Unknown, content: _ }) => {
          error.set(Some("t!(i18n, unknown)().to_string()".into()));
        },
        // Ok(LemmyErrorType::IncorrectLogin) => {
        //     error.set(Some(t!(i18n, invalid_login)().to_string()));
        // },
        Ok(o) => {
          logging::log!("other {}", o);
          error.set(Some(t!(i18n, unknown)().to_string()));
        },
        Err(g) => {
          logging::log!("errare {}", g);
          error.set(Some(t!(i18n, unknown)().to_string()));
        },
      }
    },
    Some(Err(v)) => {
      logging::log!("errare {}", v);
      error.set(Some(t!(i18n, unknown)().to_string()));
    },
  });

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
    logging::log!("vallllyyyy");

    // let data = LoginAction::from_event(&ev);

    // if data.is_err() {
    //   //     // ev.prevent_default() will prevent form submission
    //       ev.prevent_default();
    // }
    let req = Login { username_or_email: name.get().into(), password: password.get().into(), totp_2fa_token: None };
  
    let soyeah = validate(&req);
    // let soyeah = validate(&data.expect("msg"));

    if let Some(e) = soyeah {
      ev.prevent_default();
    } else {
      // ev.prevent_default();

      // logging::log!("yayayay");

      // let loooogy = create_resource(move || (), move |()| async move {
      //   logging::log!("loggin in");
      //   let req = Login { username_or_email: name.get().into(), password: password.get().into(), totp_2fa_token: None };

      //   let result = {
      //     use crate::lemmy_client::*;
      //     Some((Fetch {}).login(req).await)
      //   };

      //   logging::log!("CSR result {:#?}", result);

      //   match result {
      //     Some(Ok(o)) => {
      //       logging::log!("CSR cookie {:#?}", o);
      //       #[cfg(not(feature = "ssr"))]
      //       use wasm_cookies::set;
      //       #[cfg(not(feature = "ssr"))]
      //       set("jwt", &o.jwt.clone().unwrap().to_string()[..], &CookieOptions { same_site: wasm_cookies::cookies::SameSite::Strict, secure: true, expires: Some(std::borrow::Cow::Borrowed("Sat, 04 Jan 2025 19:24:51 GMT")), domain: None, path: None } );

      //       let navigate = leptos_router::use_navigate();
      //       navigate("/", Default::default());
      //       Some(o)
      //     },
      //     Some(Err(e)) => {
      //       error.set(Some(e.to_string()));
      //       None
      //     }
      //     _ => None,
      //   }
      // });
    }
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
          class="input input-bordered"
          placeholder="Username"
          value=name
          on:input=move |ev| update!(| name | *name = event_target_value(& ev))
        />
      </div>

      <PasswordInput
        id="password"
        name="password"
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
