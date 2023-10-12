use crate::{api::get_cookie_wrapper, i18n::*};
#[cfg(feature = "ssr")]
use leptos::IntoAttribute;
use leptos::{
  component,
  create_effect,
  create_resource,
  create_rw_signal,
  create_server_action,
  expect_context,
  server,
  spawn_local,
  use_context,
  view,
  Await,
  ErrorBoundary,
  IntoView,
  RwSignal,
  ServerFnError,
  Show,
  SignalGet,
  SignalSet,
  SignalWith,
  Suspense,
};
use leptos_icons::*;
use leptos_router::*;

#[server(LogoutFormFn, "/serverfn")]
pub async fn logout_form_fn(is_ssr: bool) -> Result<(), ServerFnError> {
  use crate::api::remove_cookie_wrapper;
  use leptos_actix::redirect;

  if is_ssr {
    redirect("/");
  }

  match remove_cookie_wrapper("jwt").await {
    Ok(o) => Ok(o),
    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
  }
}

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();
  let is_ssr_only = create_rw_signal::<bool>(true);

  #[cfg(not(feature = "ssr"))]
  is_ssr_only.set(false);

  let authenticated = expect_context::<RwSignal<bool>>();
  let ui_theme = expect_context::<RwSignal<String>>();
  // let test = create_rw_signal::<bool>(false);

  let auth_resource = create_resource(
    || (),
    move |()| async move {
      match get_cookie_wrapper("jwt").await {
        Ok(Some(_jwt)) => {
          leptos::logging::log!("1");
          authenticated.set(true);
          // test.set(true);
          true
        }
        Ok(None) => {
          leptos::logging::log!("2");
          authenticated.set(false);
          false
        }
        Err(_e) => {
          leptos::logging::log!("3");
          authenticated.set(false);
          false
        }
      }
    },
  );

  // let thing = auth_resource.get();

  create_effect(move |_| match auth_resource.get() {
    Some(true) => authenticated.set(true),
    _ => authenticated.set(false),
  });
  // #[cfg(not(feature = "ssr"))]
  // spawn_local(async move {
  //   match get_cookie_wrapper("jwt").await {
  //     Ok(Some(_jwt)) => {
  //       leptos::logging::log!("ee1");
  //       authenticated.set(true);
  //       // true
  //     }
  //     Ok(None) => {
  //       leptos::logging::log!("ee2");
  //       authenticated.set(false);
  //       // false
  //     }
  //     Err(_e) => {
  //       leptos::logging::log!("ee3");
  //       authenticated.set(false);
  //       // false
  //     }
  //   }
  // });

  let change_theme = move |theme_name: &'static str| move |_| {
    ui_theme.set(theme_name.to_string());
  };

  let logout_form_action = create_server_action::<LogoutFormFn>();

  create_effect(move |_| match logout_form_action.value().get() {
    None => {}
    Some(Ok(_o)) => {
      leptos::logging::log!("yeah");
      authenticated.set(false);
      let navigate = leptos_router::use_navigate();
      navigate("/", Default::default());
    }
    Some(Err(_e)) => {}
  });

  view! {
    // <span>{move || authenticated.get()}</span>
    // <span>{move || test.get()}</span>
    <nav class="container navbar mx-auto">
      <div class="navbar-start">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <A href="/" class="text-xl whitespace-nowrap">
              "Brand from env"
            </A>
          </li>
          <li>
            <A href="/communities" class="text-md">
              {t!(i18n, nav.communities)}
            </A>
          </li>
          <li>
            <A href="/create_post" class="text-md">
              {t!(i18n, nav.create_post)}
            </A>
          </li>
          <li>
            <A href="/create_community" class="text-md">
              {t!(i18n, nav.create_community)}
            </A>
          </li>
          <li>
            <a href="//join-lemmy.org/donate">
              <span title=t!(i18n, nav.donate)>
                <Icon icon=Icon::from(ChIcon::ChHeart) class="h-6 w-6"/>
              </span>
            </a>
          </li>
        </ul>
      </div>
      <div class="navbar-end">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <A href="/search">
              <span title=t!(i18n, nav.search)>
                <Icon icon=Icon::from(ChIcon::ChSearch) class="h-6 w-6"/>
              </span>
            </A>
          </li>
          <Suspense fallback=move || {
              view! {
                <li></li>
                <li></li>
              }
          }>
            // <ErrorBoundary fallback=|_| {
            // view! { <p>"Something went wrong"</p> }
            // }>
            // <li></li>
            // <li></li>
            // {move || {
            // auth_resource
            // .get()
            // .map(move |b| {
            // view! {
            // <li></li>
            // <li></li>
            // }

            // view! {
            <li class="z-[1]">
              <details>
                <summary>"Theme"</summary>
                <ul>
                  <li on:click=change_theme("dark")>
                    "Dark"
                  </li>
                  <li on:click=change_theme("light")>
                    "Light"
                  </li>
                  <li on:click=change_theme("retro")>
                    "Retro"
                  </li>
                </ul>
              </details>
            </li>

            <Show
              when=move || { authenticated.get() }
              fallback=move || {
                  view! {
                    <li>
                      <A href="/login">{t!(i18n, nav.login)}</A>
                    </li>
                    <li>
                      <A href="/signup">{t!(i18n, nav.signup)}</A>
                    </li>
                  }
              }
            >

              // <li></li>
              // <li></li>
              <li>
                <A href="/inbox">
                  <span title=t!(i18n, nav.unread_messages)>
                    <Icon icon=Icon::from(ChIcon::ChBell) class="h-6 w-6"/>
                  </span>
                </A>
              </li>
              <li class="z-[1]">
                <details>
                  <summary>"User name"</summary>
                  <ul>
                    <li>
                      <A href="/u/jimmy90">{t!(i18n, nav.profile)}</A>
                    </li>
                    <li>
                      <A href="/settings">{t!(i18n, nav.settings)}</A>
                    </li>
                    <li>
                      <hr/>
                    </li>
                    <li>
                      <ActionForm action=logout_form_action>
                        <input
                          name="is_ssr"
                          type="hidden"
                          value=move || format!("{}", is_ssr_only.get())
                        />
                        <button type="submit">{t!(i18n, nav.logout)}</button>
                      </ActionForm>
                    </li>
                  </ul>
                </details>
              </li>
            </Show>
          // }
          // })
          // }}

          // {move || {
          // auth_resource
          // .get()
          // .map(move |b| {
          // authenticated.with(|b| {
          // // if !authenticated.get() {
          // if !b {
          // view! {
          // <li>
          // <A href="/login">{t!(i18n, nav.login)}</A>
          // </li>
          // <li>
          // <A href="/signup">{t!(i18n, nav.signup)}</A>
          // </li>
          // }
          // } else {
          // view! {
          // <li>
          // <A href="/inbox">
          // <span title=t!(i18n, nav.unread_messages)>
          // <Icon icon=Icon::from(ChIcon::ChBell) class="h-6 w-6"/>
          // </span>
          // </A>
          // </li>
          // <li>
          // <details>
          // <summary>"User name"</summary>
          // <ul>
          // <li>
          // <A href="/u/jimmy90">{t!(i18n, nav.profile)}</A>
          // </li>
          // <li>
          // <A href="/settings">{t!(i18n, nav.settings)}</A>
          // </li>
          // <li>
          // <hr/>
          // </li>
          // <li>
          // <ActionForm action=logout_form_action>
          // <input
          // name="is_ssr"
          // type="hidden"
          // value=move || format!("{}", is_ssr.get())
          // />
          // <button type="submit">{t!(i18n, nav.logout)}</button>
          // </ActionForm>
          // </li>
          // </ul>
          // </details>
          // </li>
          // }
          // }
          // })
          // })
          // }}

          // </ErrorBoundary>
          </Suspense>

        </ul>
      </div>
    </nav>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();

  view! {
    <nav class="container navbar mx-auto">
      <div class="navbar-start"></div>
      <div class="navbar-end ">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <a href="//github.com/LemmyNet/lemmy-ui-leptos/releases" class="text-md">
              "f/e from env"
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
              "b/e from env"
            </a>
          </li>
          <li>
            <A href="/modlog" class="text-md">
              {t!(i18n, nav.modlog)}
            </A>
          </li>
          <li>
            <A href="/instances" class="text-md">
              {t!(i18n, nav.instances)}
            </A>
          </li>
          <li>
            <a href="//join-lemmy.org/docs/en/index.html" class="text-md">
              {t!(i18n, nav.docs)}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet" class="text-md">
              {t!(i18n, nav.code)}
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
