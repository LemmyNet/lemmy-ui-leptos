use crate::{
  cookie::{remove_cookie, set_cookie},
  errors::{self, message_from_error, LemmyAppError},
  i18n::*,
  lemmy_client::*,
  ui::components::common::icon::{
    Icon,
    IconType::{Donate, Notifications, Search},
  },
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_router::*;
use web_sys::SubmitEvent;

#[server(LogoutFn, "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  // use leptos_actix::redirect;
  let result = LemmyClient.logout().await;
  match result {
    Ok(_o) => {
      let r = remove_cookie("jwt").await;
      match r {
        Ok(_o) => {
          // redirect("/");
          Ok(())
        }
        Err(_e) => {
          // redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
          Ok(())
        }
      }
    }
    Err(_e) => {
      // redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[server(ChangeLangFn, "/serverfn")]
pub async fn change_lang(lang: String) -> Result<(), ServerFnError> {
  let _ = set_cookie(
    "i18n_pref_locale",
    &lang.to_lowercase(),
    &core::time::Duration::from_secs(604800),
  )
  .await;
  Ok(())
}

#[server(ChangeThemeFn, "/serverfn")]
pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
  // use leptos_actix::redirect;
  let r = set_cookie("theme", &theme, &core::time::Duration::from_secs(604800)).await;
  match r {
    Ok(_o) => Ok(()),
    Err(_e) => {
      // redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[component]
pub fn TopNav(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>
) -> impl IntoView {
  let i18n = use_i18n();

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();

  if let Some(Err(e)) = site_signal.get() {
    error.set(Some(e));
  }

  let query = use_query_map();
  let ssr_error = move || query.with(|params| params.get("error").cloned());

  if let Some(e) = ssr_error() {
    if !e.is_empty() {
      let r = serde_json::from_str::<LemmyAppError>(&e[..]);

      match r {
        Ok(e) => {
          error.set(Some(e));
        }
        Err(_) => {
          logging::error!("error decoding error - log and ignore in UI?");
        }
      }
    }
  }

  let user = expect_context::<RwSignal<Option<bool>>>();

  let logout_action = create_server_action::<LogoutFn>();

  let on_logout_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        let result = LemmyClient.logout().await;
        match result {
          Ok(_o) => {
            let _ = remove_cookie("jwt").await;
            user.set(Some(false));
          }
          Err(e) => {
            logging::warn!("logout error {:#?}", e);
            error.set(Some(e));
          }
        }
      },
    );
  };

  let ui_theme = expect_context::<RwSignal<Option<String>>>();
  let theme_action = create_server_action::<ChangeThemeFn>();

  let on_theme_submit = move |theme_name: &'static str| {
    move |ev: SubmitEvent| {
      ev.prevent_default();
      let _res = create_local_resource(
        move || theme_name.to_string(),
        move |t| async move {
          let _ = set_cookie("theme", &t, &core::time::Duration::from_secs(604800)).await;
        },
      );
      ui_theme.set(Some(theme_name.to_string()));
    }
  };

  let lang_action = create_server_action::<ChangeLangFn>();

  let on_lang_submit = move |lang: Locale| {
    move |ev: SubmitEvent| {
      ev.prevent_default();
      i18n.set_locale(lang);
    }
  };

  view! {
    <nav class="navbar container mx-auto hidden sm:flex sticky top-0 bg-base-100 z-[1]">
      <div class="navbar-start">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <A href="/" class="text-xl whitespace-nowrap">
              {move || {
                  if let Some(Ok(m)) = site_signal.get() {
                      m.site_view.site.name
                  } else {
                      "Lemmy".to_string()
                  }
              }}
            </A>
          </li>
          <li>
            <A href="/communities" class="text-md">
              {t!(i18n, communities)}
            </A>
          </li>
          <li>
            <A href="/create_post" class="text-md">
              {t!(i18n, create_post)}
            </A>
          </li>
          <li>
            <A href="/create_community" class="text-md">
              {t!(i18n, create_community)}
            </A>
          </li>
          <li>
            <a href="//join-lemmy.org/donate">
              <span title="t!(i18n, donate)">
                <Icon icon=Donate/>
              </span>
            </a>
          </li>
        </ul>
      </div>
      <div class="navbar-end">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <A href="/search">
              <span title="t!(i18n, search)">
                <Icon icon=Search/>
              </span>
            </A>
          </li>
          <li class="z-[1]">
            <details>
              <summary>"Lang"</summary>
              <ul>
                <li>
                  <ActionForm action=lang_action on:submit=on_lang_submit(Locale::fr)>
                    <input type="hidden" name="lang" value="FR"/>
                    <button type="submit">"FR"</button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm action=lang_action on:submit=on_lang_submit(Locale::en)>
                    <input type="hidden" name="lang" value="EN"/>
                    <button type="submit">"EN"</button>
                  </ActionForm>
                </li>
              </ul>
            </details>
          </li>
          <li class="z-[1]">
            <details>
              <summary>"Theme"</summary>
              <ul>
                <li>
                  <ActionForm action=theme_action on:submit=on_theme_submit("dark")>
                    <input type="hidden" name="theme" value="dark"/>
                    <button type="submit">"Dark"</button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm action=theme_action on:submit=on_theme_submit("light")>
                    <input type="hidden" name="theme" value="light"/>
                    <button type="submit">"Light"</button>
                  </ActionForm>
                </li>
                <li>
                  <ActionForm action=theme_action on:submit=on_theme_submit("retro")>
                    <input type="hidden" name="theme" value="retro"/>
                    <button type="submit">"Retro"</button>
                  </ActionForm>
                </li>
              </ul>
            </details>
          </li>
          <Show
            when=move || {
                if let Some(Ok(GetSiteResponse { my_user: Some(_), .. })) = site_signal.get() {
                    true
                } else {
                    false
                }
            }

            fallback=move || {
                view! {
                  <li>
                    <A href="/login">{t!(i18n, login)}</A>
                  </li>
                  <li>
                    <A href="/signup">{t!(i18n, signup)}</A>
                  </li>
                }
            }
          >

            <li>
              <A href="/inbox">
                <span title=t!(i18n, unread_messages)>
                  <Icon icon=Notifications/>
                </span>
              </A>
            </li>
            <li>
              <details>
                <summary>
                  {move || {
                      if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = site_signal
                          .get()
                      {
                          m.local_user_view
                              .person
                              .display_name
                              .unwrap_or(m.local_user_view.person.name)
                      } else {
                          String::default()
                      }
                  }}

                </summary>
                <ul class="z-10">
                  <li>
                    <A href=move || {
                        format!(
                            "/u/{}",
                            if let Some(Ok(GetSiteResponse { my_user: Some(m), .. })) = site_signal
                                .get()
                            {
                                m.local_user_view.person.name
                            } else {
                                String::default()
                            },
                        )
                    }>{t!(i18n, profile)}</A>
                  </li>
                  <li>
                    <A href="/settings">{t!(i18n, settings)}</A>
                  </li>
                  <div class="divider my-0"></div>
                  <li>
                    <ActionForm action=logout_action on:submit=on_logout_submit>
                      <button type="submit">{t!(i18n, logout)}</button>
                    </ActionForm>
                  </li>
                </ul>
              </details>
            </li>
          </Show>
        </ul>
      </div>
    </nav>
    <Show
      when=move || error.get().is_some()
      fallback=move || {
          view! { <div class="hidden"></div> }
      }
    >

      {move || {
          error
              .get()
              .map(|err| {
                  view! {
                    <div class="container mx-auto alert alert-error">
                      <span>{message_from_error(&err)} " - " {err.content}</span>
                    </div>
                  }
              })
      }}

    </Show>
  }
}

#[component]
pub fn BottomNav(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>
) -> impl IntoView {
  let i18n = use_i18n();
  const FE_VERSION: &str = env!("CARGO_PKG_VERSION");

  view! {
    <nav class="container navbar mx-auto hidden sm:flex">
      <div class="navbar-start w-auto"></div>
      <div class="navbar-end grow w-auto">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <a href="//github.com/LemmyNet/lemmy-ui-leptos/releases" class="text-md">
              "FE: "
              {FE_VERSION}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
              "BE: "
              {move || {
                  if let Some(Ok(m)) = site_signal.get() {
                      m.version
                  } else {
                      "Lemmy".to_string()
                  }
              }}
            </a>
          </li>
          <li>
            <A href="/modlog" class="text-md">
              {t!(i18n, modlog)}
            </A>
          </li>
          <li>
            <A href="/instances" class="text-md">
              {t!(i18n, instances)}
            </A>
          </li>
          <li>
            <a href="//join-lemmy.org/docs/en/index.html" class="text-md">
              {t!(i18n, docs)}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet" class="text-md">
              {t!(i18n, code)}
            </a>
          </li>
          <li>
            <a href="//join-lemmy.org" class="text-md">
              "join-lemmy.org"
            </a>
          </li>
        </ul>
      </div>
    </nav>
  }
}
