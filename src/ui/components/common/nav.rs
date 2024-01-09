use crate::{i18n::*, queries::site_state_query::*};
use lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::*;
// use leptos_icons::*;
use leptos_query::*;
use leptos_router::*;
use phosphor_leptos::{Bell, Heart, MagnifyingGlass};

#[server(LogoutAction, "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use actix_session::Session;
  use leptos_actix::extract;

  extract(|session: Session| async move {
    // TODO: Will have to make API call to delete session stored in DB once that feature is implemented on the server
    session.purge();
  })
  .await
}

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();

  let QueryResult { data, refetch, .. } = use_site_state();

  // let data = create_resource(|| (), move |()| async move {
  //   let result = {
  //       #[cfg(not(feature = "ssr"))]
  //       {
  //         logging::log!("yo 2");
  //         use crate::lemmy_client::*;
  //         Some((Fetch {}).get_site(None).await)
  //       }
  //       #[cfg(feature = "ssr")]
  //       {
  //         use crate::lemmy_client::LemmyClient;
  //         use actix_web::web;
  //         use leptos_actix::extract;

  //         logging::log!("here 1");

  //         extract(|client: web::Data<awc::Client>| async move { client.get_site(None).await })
  //           .await
  //           .ok()
  //       }
  //   };
  //   logging::log!("here 2");

  //   result
  // });

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

  let logout_action = create_server_action::<LogoutAction>();
  let logout_is_success =
    Signal::derive(move || logout_action.value().get().is_some());

  create_isomorphic_effect(move |_| {
    if logout_is_success.get() {
      logging::log!("LOGOUT");
      refetch();
    }
  });

  let ui_theme = expect_context::<RwSignal<String>>();

  let change_theme = move |theme_name: &'static str| {
    move |_| {
      ui_theme.set(theme_name.to_string());
    }
  };

  view! {
    // <Transition>
      <nav class="navbar container mx-auto">
        <div class="navbar-start">
          <ul class="menu menu-horizontal flex-nowrap">
            <li>
              <A href="/" class="text-xl whitespace-nowrap">
              <Transition fallback=|| { view! { "Loading..." } }>
              // <Suspense fallback=|| { view! { "Loading..." } }>
                {move || instance_name}
              // {move || {
              //   data
              //       .get()
              //       .map(|res| {
              //         logging::log!("result {:#?}", res);
              //         match res {
              //           Some(Ok(o)) => {
              //             view! { <div> {o.site_view.site.name} </div> }
              //           },
              //           _ => {
              //               view! { <div> "Lemmy" </div> }
              //           },
              //         }
              //       })
              // }}
              // </Suspense>
              </Transition>
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
          <ul class="menu menu-horizontal flex-nowrap">
            <li>
              <A href="/search">
                <span title=t!(i18n, search)>
                  <MagnifyingGlass/>
                </span>
              </A>
            </li>
            <li class="z-[1]">
              <details>
                <summary>"Theme"</summary>
                <ul>
                  <li on:click=change_theme("dark")>
                    <span>"Dark"</span>
                  </li>
                  <li on:click=change_theme("light")>
                    <span>"Light"</span>
                  </li>
                  <li on:click=change_theme("retro")>
                    <span>"Retro"</span>
                  </li>
                </ul>
              </details>
            </li>
            <Transition fallback=|| { view! { "Loading..." } }>
            // <Suspense fallback=|| { view! { "Loading..." } }>
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
                      <Bell class="h-6 w-6" />
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
                        <A href=with!(|my_user| format!("/u/{}", my_user.as_ref().unwrap().name))>
                          {t!(i18n, profile)}
                        </A>
                      </li>
                      <li>
                        <A href="/settings">{t!(i18n, settings)}</A>
                      </li>
                      <div class="divider my-0"></div>
                      <li>
                        <ActionForm action=logout_action>
                          <button type="submit">{t!(i18n, logout)}</button>
                        </ActionForm>
                      </li>
                    </ul>
                  </details>
                </li>
              </Show>
            // </Suspense>
            </Transition>
          </ul>
        </div>
      </nav>
    // </Transition>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();

  let QueryResult { data, .. } = use_site_state();

  // let data = create_resource(|| (), move |()| async move {
  //   let result = {
  //       #[cfg(not(feature = "ssr"))]
  //       {
  //         logging::log!("yo 1");
  //         use crate::lemmy_client::*;
  //         Some((Fetch {}).get_site(None).await)
  //       }
  //       #[cfg(feature = "ssr")]
  //       {
  //         use crate::lemmy_client::LemmyClient;
  //         use actix_web::web;
  //         use leptos_actix::extract;

  //         logging::log!("here 3");

  //         extract(|client: web::Data<awc::Client>| async move { client.get_site(None).await })
  //           .await
  //           .ok()
  //       }
  //   };
  //   logging::log!("here 4");

  //   result
  // });

  let instance_api_version = Signal::derive(move || {
    data.get().map_or_else(|| Some(String::from("n/a")), |res| Some(res.ok()?.version))
  });

  const FE_VERSION: &str = env!("CARGO_PKG_VERSION");

  view! {
    <nav class="container navbar mx-auto">
      <div class="navbar-start"></div>
      <div class="navbar-end ">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <a href="//github.com/LemmyNet/lemmy-ui-leptos/releases" class="text-md">
              "FE: "
              {FE_VERSION}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
              "BE: "
              <Transition fallback=|| { view! { "Loading..." } }>
              // <Suspense fallback=|| { view! { "Loading..." } }>
              // {move || {
              //   data
              //       .get()
              //       .map(|res| match res {
              //           Some(Ok(o)) => {
              //               view! { <span> {o.version} </span> }
                          {move || instance_api_version}
              //           },
              //           _ => {
              //               view! { <span> "n/a" </span> }
              //           },
              //       })
              // }}
              // </Suspense>
              </Transition>
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
