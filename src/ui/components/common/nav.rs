use crate::{i18n::*, lemmy_client::*, queries::site_state_query::*, errors::LemmyAppError, cookie::{set_cookie, remove_cookie, get_cookie}};
use chrono::Duration;
// use actix_web::cookie::time::Duration;
use lemmy_api_common::{lemmy_db_schema::{source::person::Person, newtypes::PostId}, site::GetSiteResponse, post::GetPost};
use leptos::*;
// use leptos_icons::*;
use leptos_query::*;
use leptos_router::*;
use phosphor_leptos::{Bell, Heart, MagnifyingGlass};
use web_sys::{SubmitEvent, MouseEvent};

#[server(LogoutFn, "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  // use actix_session::Session;
  use leptos_actix::{extract, redirect};

  let result = Fetch.logout().await;

  match result {
    Ok(()) => {
      let r = remove_cookie("jwt").await;
      
      //  = extract(|session: Session| async move {
      //   // session.purge();
      //   // session.remove("jwt");
      // })
      // .await;

      match r {
        Ok(_o) => {
          redirect("/");
          Ok(())
        }
        Err(e) => Err(e.into()),
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
  logging::log!("{:#?}", lang);

  // provide_i18n_context();
  set_cookie("i18n_pref_locale", &lang.to_lowercase(), &std::time::Duration::from_secs(604800)).await;

  // let i18n = use_i18n();
  // if lang.eq(&"FR".to_string()) {
  //   i18n.set_locale(Locale::fr);
  // }
  // if lang.eq(&"EN".to_string()) {
  //   i18n.set_locale(Locale::en);
  // }
  Ok(())
}

#[server(ChangeThemeFn, "/serverfn")]
pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
  // use actix_session::Session;
  // use leptos_actix::extract;

  let r = set_cookie("theme", &theme, &std::time::Duration::from_secs(604800)).await;
  // set_cookie(path, value, expires) extract(|session: Session| async move { session.insert("theme", theme) }).await;

  match r {
    Ok(_o) => {
      Ok(())
    }
    Err(e) => Err(e.into()),
  }
}

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();

  // let data: Signal<Option<Result<GetSiteResponse, LemmyAppError>>> = Signal::derive(|| None);

  // let QueryResult { data, refetch, .. } = use_site_state();

  let site_data = expect_context::<RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>>();

  let ssr_data = create_resource(move || (), move |()| async move {
    // let jwt = get_cookie("jwt").await?;
    Fetch.get_site(/* jwt */).await
  });

  let data = Signal::derive(move || {
    site_data.get().or(ssr_data.get().or(None))
  });

  let my_user = Signal::<Option<Person>>::derive(move || {
    data.get().map_or_else(
      || None,
      |res| res.ok()?.my_user.map(|user| user.local_user_view.person),
    )
  });

  let instance_name = Signal::derive(move || {
    data.get().map_or_else(
      || Some(String::from("Lemmy")),
      |res| Some(res.ok()?.site_view.site.name),
    )
  });

  let logout_action = create_server_action::<LogoutFn>();
  // let logout_is_success = Signal::derive(move || logout_action.value().get().is_some());

  // create_isomorphic_effect(move |_| {
  //   if logout_is_success.get() {
  //     logging::log!("LOGOUT");
  //     refetch();
  //   }
  // });

  let on_logout_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        let result = Fetch.logout().await;

        match result {
          Ok(_o) => {
            // #[cfg(not(feature = "ssr"))]
            // {
              remove_cookie("jwt");
              // set_cookie("jwt", "value", &std::time::Duration::from_secs(-604800));
              site_data.set(Some( //create_resource(move || (), move |()| async move {
                Fetch.get_site(/* None */).await
              )); //}).get());

              // wasm_cookies::set(
              //   "jwt",
              //   "",
              //   &wasm_cookies::cookies::CookieOptions {
              //     same_site: wasm_cookies::cookies::SameSite::Strict,
              //     secure: true,
              //     expires: Some(std::borrow::Cow::Borrowed("Sat, 01 Jan 2024 19:24:51 GMT")),
              //     domain: None,
              //     path: None,
              //   },
              // );
            // }

            // let QueryResult { refetch, .. } = use_site_state();
            // refetch();
          }
          Err(_e) => {
            logging::log!("logout error {:#?}", _e);
            // error.set(Some(message_from_error(&e)));
// 
            // match e {
            //   _ => {
            //     report_validation.set("".to_string());
            //   }
            // }
          }
        }
      },
    );
  };

  let ui_theme = expect_context::<RwSignal<Option<String>>>();
  let theme_action = create_server_action::<ChangeThemeFn>();

  // let act = create_multi_action(|themer: &String| {
  //   let t = themer.clone();
  //   let r = &t[..];
  //   let d = std::time::Duration::from_secs(604800);
  //   set_cookie("theme", t, d)
  // });




  // #[cfg(not(feature = "ssr"))]
  // let bless = create_local_resource(move || (), move |()| async move {
  //   use std::time::*;
  //   use chrono::offset::Utc;
  //   use chrono::DateTime;

  //   logging::log!("YA");
  //   logging::log!("{:#?}", get_cookie("theme").await);
  //   logging::log!("YA");
  //   remove_cookie("theme").await;
  //   logging::log!("YA");

  //   let mut now = SystemTime::now();
  //   now += std::time::Duration::from_secs(604800);
  //   let datetime: DateTime<Utc> = now.into();

  //   // set_cookie("theme", "theme_name".to_string(), std::time::Duration::from_secs(604800)).await;// "datetime.to_rfc3339()".to_string()).await; 
  // }).get();


  let on_theme_submit = move |theme_name: &'static str| {
    move |ev: SubmitEvent| {
      ev.prevent_default();
      let _res = create_local_resource(move || theme_name.to_string(), move |t| async move {
        set_cookie("theme", &t, &std::time::Duration::from_secs(604800)).await;
      });
      ui_theme.set(Some(theme_name.to_string()));
    }
  };

  // let _res = create_action(|theme: &String| {
  //   let t = theme.clone();
  //   async move {
  //     // let r = set_cookie("theme", t, std::time::Duration::from_secs(604800)).await;
  //   }
  // });

  // use std::time::*;
  // use chrono::offset::Utc;
  // use chrono::DateTime;
  // let q = chrono::offset::Utc::now();

  //     // let now = SystemTime::now();
  // let now = q + std::time::Duration::from_secs(604800);
  // let datetime: DateTime<Utc> = now.into();


  // let on_theme_click = move |theme_name: &'static str| {
  //   move |ev: MouseEvent| {
  //     ev.prevent_default();
  //     spawn_local(async move { 

  //       // let form = GetPost {
  //       //   id: Some(PostId(9513)),
  //       //   comment_id: None,
  //       // };
  //       // Fetch.get_post(form).await;



  // // #[cfg(not(feature = "ssr"))]
  // // set_cookie("theme", theme_name.to_string(), "Sat, 04 Jan 2025 19:24:51 GMT".to_string()/* now.to_rfc3339() */).await; 
  //       set_cookie("theme", theme_name.to_string(), std::time::Duration::from_secs(604800)).await; 

  //     });
  //     // _res.dispatch("input".to_string());
  //   }
  // };




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
              }>{move || instance_name}</Transition>
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
            <A href="/search"/*  on:click=on_theme_click("light") */>
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
            <Show
              when=move || with!(| my_user | my_user.is_some())
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
                    {with!(
                        | my_user | { let Person { name, display_name, .. } = my_user.as_ref()
                        .unwrap(); display_name.as_ref().unwrap_or(name).to_string() }
                    )}
                  </summary>
                  <ul class="z-10">
                    <li>
                      <A href=with!(
                          | my_user | format!("/u/{}", my_user.as_ref().unwrap().name)
                      )>{t!(i18n, profile)}</A>
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
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();

  // let data: Signal<Option<Result<GetSiteResponse, LemmyAppError>>> = Signal::derive(|| None);

  // let QueryResult { data, .. } = use_site_state();

  let site_data = expect_context::<RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>>();

  let ssr_data = create_resource(move || (), move |()| async move {
    // let jwt = get_cookie("jwt").await?;
    Fetch.get_site(/* jwt */).await
  });

  let data = Signal::derive(move || {
    site_data.get().or(ssr_data.get().or(None))
  });

  let instance_api_version = Signal::derive(move || {
    data
      .get()
      .map_or_else(|| Some(String::from("n/a")), |res| Some(if res.clone().ok()?.version.is_empty() { String::from("empty") } else { res.ok()?.version }))
  });

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
              }>{move || instance_api_version}</Transition>
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
