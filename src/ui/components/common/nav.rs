use crate::{
  cookie::{get_cookie, remove_cookie, set_cookie},
  errors::{message_from_error, LemmyAppError},
  i18n::*,
  lemmy_client::*,
  queries::site_state_query::*,
};
use chrono::Duration;
use lemmy_api_common::{
  lemmy_db_schema::{newtypes::PostId, source::person::Person},
  post::GetPost,
  site::{GetSiteResponse, MyUserInfo},
};
use leptos::*;
use leptos_router::*;
use phosphor_leptos::{Bell, Heart, MagnifyingGlass};
use web_sys::{MouseEvent, SubmitEvent};

#[server(LogoutFn, "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use leptos_actix::{extract, redirect};
  let result = LemmyClient.logout().await;
  match result {
    Ok(o) => {
      let r = remove_cookie("jwt").await;
      match r {
        Ok(_o) => {
          redirect("/");
          Ok(())
        }
        Err(e) => {
          redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
          Ok(())
        }
      }
    }
    Err(e) => {
      redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[server(ChangeLangFn, "/serverfn")]
pub async fn change_lang(lang: String) -> Result<(), ServerFnError> {
  set_cookie(
    "i18n_pref_locale",
    &lang.to_lowercase(),
    &core::time::Duration::from_secs(604800),
  )
  .await;
  Ok(())
}

#[server(ChangeThemeFn, "/serverfn")]
pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
  use leptos_actix::{extract, redirect};
  let r = set_cookie("theme", &theme, &core::time::Duration::from_secs(604800)).await;
  match r {
    Ok(_o) => Ok(()),
    Err(e) => {
      redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
      Ok(())
    }
  }
}

#[component]
pub fn TopNav(site_signal: RwSignal<Option<GetSiteResponse>>) -> impl IntoView {
  let i18n = use_i18n();

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();

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
          logging::log!("error decoding error - log and ignore in UI?");
        }
      }
    }
  }

  // // let site_data = expect_context::<RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>>();
  let user = expect_context::<RwSignal<Option<bool>>>();

  // let data = create_resource(
  //   move || (user.get()),
  //   move |(_user)| async move { LemmyClient.get_site().await },
  // );

  // // let data = Signal::derive(move || site_data.get().or(ssr_data.get().or(None)));
  // // let data = site_signal;

  // let my_user = Signal::<Option<Person>>::derive(move || {
  //   data.get().map_or_else(
  //     || None,
  //     |res| res.ok()?.my_user.map(|user| user.local_user_view.person),
  //   )
  // });

  // let instance_name = Signal::derive(move || {
  //   data.get().map_or_else(
  //     || Some(String::from("Lemmy")),
  //     |res| Some(res.ok()?.site_view.site.name),
  //   )
  // });

  let logout_action = create_server_action::<LogoutFn>();

  let on_logout_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        let result = LemmyClient.logout().await;
        match result {
          Ok(_o) => {
            remove_cookie("jwt").await;
            user.set(Some(false));
            // site_data.set(Some(LemmyClient.get_site().await));
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
          set_cookie("theme", &t, &core::time::Duration::from_secs(604800)).await;
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
    <nav class="navbar container mx-auto">
      <div class="navbar-start">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <A href="/" class="text-xl whitespace-nowrap">
              <Transition fallback=|| {
                  view! { "Loading..." }
              }>{move || site_signal.get().map(|m| m.site_view.site.name)/*  instance_name */}</Transition>
              // " "
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
              <span title=t!(i18n, donate)>
                <Heart/>
              </span>
            </a>
          </li>
        </ul>
      </div>
      <div class="navbar-end">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <A href="/search">
              <span title=t!(i18n, search)>
                <MagnifyingGlass/>
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
          <Transition fallback=|| {
              view! { "Loading..." }
          }>
            // { move || data.get().map(|m| {
            //   if let Ok(o) = m {
            //     if let Some(s) = o.my_user {
            //       view! {
            //           <div> {s.local_user_view.person.name} </div>
            //       }
            //     } else {
            //       view! {
            //         <div><A href="/login">{t!(i18n, login)}</A></div>
            //       }
            //     }
            //   } else {
            //     view! {
            //         <div> "Err" </div>
            //     }
            //   }
            // })}
            <Show
              // when=move || true
              when=move || if let Some(GetSiteResponse { my_user: Some(_), .. }) = site_signal.get() { true } else { false }
              // when=move || site_signal.get().map(|m| m.my_user.is_some() /*  with!(| my_user | my_user.is_some() */).unwrap()
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
                    <Bell class="h-6 w-6"/>
                  </span>
                </A>
              </li>
              <li>
                <details>
                  <summary>
                    { move || site_signal.get().map(|m| m.my_user.map(|n| n.clone().local_user_view.person.display_name.unwrap_or(n.local_user_view.person.name))) }
                    // {with!(
                    //     | my_user | { let Person { name, display_name, .. } = my_user.as_ref()
                    //     .unwrap(); display_name.as_ref().unwrap_or(name).to_string() }
                    // )}

                  </summary>
                  <ul class="z-10">
                    <li>
                      <A href=move || format!("/u/{}", if let Some(GetSiteResponse { my_user: Some(m), .. }) = site_signal.get() { m.local_user_view.person.name } else { String::default() }) //.map(|m| m.my_user.map(|n| n.local_user_view.person.name)))
                      // with!(| my_user | format!("/u/{}", my_user.as_ref().unwrap().name)
                      // )
                      >{t!(i18n, profile)}</A>
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
          </Transition>
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
pub fn BottomNav(site_signal: RwSignal<Option<GetSiteResponse>>) -> impl IntoView {
  let i18n = use_i18n();
  // let site_data = expect_context::<RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>>();

  // let ssr_data = create_resource(
  //   move || (),
  //   move |()| async move { LemmyClient.get_site().await },
  // );

  // let data = Signal::derive(move || site_data.get().or(ssr_data.get().or(None)));

  // let instance_api_version = Signal::derive(move || {
  //   data.get().map_or_else(
  //     || Some(String::from("n/a")),
  //     |res| {
  //       Some(if res.clone().ok()?.version.is_empty() {
  //         String::from("empty")
  //       } else {
  //         res.ok()?.version
  //       })
  //     },
  //   )
  // });

  const FE_VERSION: &str = env!("CARGO_PKG_VERSION");

  view! {
    <nav class="container navbar mx-auto">
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
              <Transition fallback=|| {
                  view! { "Loading..." }
              }>{ move || site_signal.get().map(|m| m.version) }</Transition>
              // }>{move || instance_api_version}</Transition>
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
