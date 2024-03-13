use crate::{
  i18n::*,
  queries::site_state_query::use_site_state,
  ui::components::common::{
    icon::{
      Icon,
      IconType::{Donate, Notifications, Search},
    },
    unpack::Unpack,
  },
};
use lemmy_client::LemmyRequest;
use leptos::{server_fn::error::NoCustomError, *};
use leptos_query::QueryResult;
use leptos_router::*;

#[server(prefix = "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;
  client
    .logout(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

  session.purge();
  Ok(())
}

// #[server(ChangeThemeFn, "/serverfn")]
// pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
//   // use leptos_actix::redirect;
//   let r = set_cookie("theme", &theme, &core::time::Duration::from_secs(604800)).await;
//   match r {
//     Ok(_o) => Ok(()),
//     Err(_e) => {
//       // redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
//       Ok(())
//     }
//   }
// }

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();

  let QueryResult {
    data: site_response,
    refetch,
    ..
  } = use_site_state().use_query(|| ());

  let user_is_logged_in = Signal::derive(move || {
    with!(
      |site_response| site_response.as_ref().map(|site_response| site_response
        .as_ref()
        .ok()
        .map(|site_response| site_response.my_user.as_ref().is_some()))
    )
    .flatten()
    .unwrap_or_default()
  });

  let names = Signal::derive(move || {
    with!(
      |site_response| site_response.as_ref().map(|site_response| site_response
        .as_ref()
        .map_err(Clone::clone)
        .map(
          |site_response| site_response.my_user.as_ref().map(|my_user| (
            my_user.local_user_view.person.name.clone(),
            my_user.local_user_view.person.display_name.clone()
          ))
        ))
    )
  });

  let instance_name = Signal::derive(move || {
    with!(
      |site_response| site_response.as_ref().map(|site_response| site_response
        .as_ref()
        .ok()
        .map(|site_response| site_response.site_view.site.name.clone()))
    )
    .flatten()
    .unwrap_or_default()
  });

  let logout_action = create_server_action::<Logout>();

  Effect::new_isomorphic(move |_| {
    if logout_action.version().with(|v| *v > 0) {
      refetch();
    }
  });

  // let ui_theme = expect_context::<RwSignal<Option<String>>>();
  // let theme_action = create_server_action::<ChangeThemeFn>();

  // let on_theme_submit = move |theme_name: &'static str| {
  //   move |ev: SubmitEvent| {
  //     ev.prevent_default();
  //     let _res = create_local_resource(
  //       move || theme_name.to_string(),
  //       move |t| async move {
  //         let _ = set_cookie("theme", &t, &core::time::Duration::from_secs(604800)).await;
  //       },
  //     );
  //     ui_theme.set(Some(theme_name.to_string()));
  //   }
  // };

  view! {
    <Transition fallback=|| "Loading">
      <nav class="navbar container mx-auto">
        <div class="navbar-start">
          <ul class="menu menu-horizontal flex-nowrap">
            <li>
              <A href="/" class="text-xl whitespace-nowrap">
                {instance_name}
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
          <ul class="menu menu-horizontal flex-nowrap">
            <li>
              <A href="/search">
                <span title="t!(i18n, search)">
                  <Icon icon=Search/>
                </span>
              </A>
            </li>
            // <li class="z-[1]">
            // <details>
            // <summary>"Theme"</summary>
            // <ul>
            // <li>
            // <ActionForm action=theme_action on:submit=on_theme_submit("dark")>
            // <input type="hidden" name="theme" value="dark"/>
            // <button type="submit">"Dark"</button>
            // </ActionForm>
            // </li>
            // <li>
            // <ActionForm action=theme_action on:submit=on_theme_submit("light")>
            // <input type="hidden" name="theme" value="light"/>
            // <button type="submit">"Light"</button>
            // </ActionForm>
            // </li>
            // <li>
            // <ActionForm action=theme_action on:submit=on_theme_submit("retro")>
            // <input type="hidden" name="theme" value="retro"/>
            // <button type="submit">"Retro"</button>
            // </ActionForm>
            // </li>
            // </ul>
            // </details>
            // </li>
            <Show
              when=move || with!(| user_is_logged_in | { * user_is_logged_in })

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
              <Unpack item=names let:names>
                <li>
                  <details>
                    <summary>

                      {
                          let (name, display_name) = names
                              .as_ref()
                              .expect(
                                  "None case for my_user should be handled by ancestor Show component",
                              );
                          display_name.as_ref().unwrap_or(name)
                      }

                    </summary>
                    <ul class="z-10">
                      <li>
                        <A href={
                            let name = names
                                .as_ref()
                                .expect(
                                    "None case for my_user should be handled by ancestor Show component",
                                )
                                .0
                                .as_str();
                            format!("/u/{name}")
                        }>{t!(i18n, profile)}</A>
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
              </Unpack>
            </Show>
          </ul>
        </div>
      </nav>
    </Transition>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();
  const FE_VERSION: &str = env!("CARGO_PKG_VERSION");

  let QueryResult {
    data: site_response,
    ..
  } = use_site_state().use_query(|| ());

  let version = Signal::derive(move || {
    with!(
      |site_response| site_response.as_ref().map(|site_response| site_response
        .as_ref()
        .map_err(Clone::clone)
        .map(|site_response| format!("BE: {}", site_response.version)))
    )
  });

  view! {
    <Transition fallback=|| "Loading">
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
                <Unpack item=version let:version>
                  {version}
                </Unpack>

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
    </Transition>
  }
}
