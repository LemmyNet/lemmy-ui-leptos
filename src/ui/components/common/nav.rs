use crate::{i18n::*, queries::site_state_query::use_site_state};
use lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::*;
use leptos_icons::*;
use leptos_query::QueryResult;
use leptos_router::*;

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

  let my_user = Signal::<Option<Person>>::derive(move || {
    data().map_or_else(
      || None,
      |res| res.ok()?.my_user.map(|user| user.local_user_view.person),
    )
  });

  let instance_name =
    Signal::derive(move || data().map_or_else(|| None, |res| Some(res.ok()?.site_view.site.name)));

  let logout_action = create_server_action::<LogoutAction>();
  let logout_is_success =
    Signal::derive(move || logout_action.value()().is_some_and(|res| res.is_ok()));

  create_isomorphic_effect(move |_| {
    if logout_is_success() {
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
    <Transition>
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
                <span title=t!(i18n, donate)>
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
                <span title=t!(i18n, search)>
                  <Icon icon=Icon::from(ChIcon::ChSearch) class="h-6 w-6"/>
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
                    <Icon icon=Icon::from(ChIcon::ChBell) class="h-6 w-6"/>
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
                      <ActionForm action=logout_action>
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
    </Transition>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();

  let QueryResult { data, .. } = use_site_state();

  let instance_api_version =
    Signal::derive(move || data().map(|res| res.ok().map(|res| res.version)));

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
              {instance_api_version}
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
