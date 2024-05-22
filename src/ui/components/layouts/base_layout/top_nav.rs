use crate::{
  contexts::{site_resource_context::SiteResource, theme_resource_context::ThemeResource},
  i18n::*,
  serverfns::auth::create_logout_action,
  ui::components::common::{
    icon::{Icon, IconType},
    unpack::Unpack,
  },
  utils::{derive_query_signal, derive_user_is_logged_in, types::Theme},
};
use leptos::*;
use leptos_router::*;

#[component]
fn InstanceName() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();
  let instance_name = derive_query_signal(site_resource, |site_response| {
    site_response.site_view.site.name.clone()
  });

  view! {
    <Unpack item=instance_name let:instance_name>
      <A href="/" class="text-xl whitespace-nowrap">
        {instance_name}
      </A>
    </Unpack>
  }
}

#[component]
fn LoggedInUserActionDropdown() -> impl IntoView {
  let i18n = use_i18n();
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let names = derive_query_signal(site_resource, |site_response| {
    site_response.my_user.as_ref().map(|my_user| {
      (
        my_user.local_user_view.person.name.clone(),
        my_user.local_user_view.person.display_name.clone(),
      )
    })
  });

  let logout_action = create_logout_action();

  Effect::new(move |_| {
    if logout_action.version().get() > 0 {
      site_resource.refetch();
    }
  });

  view! {
    <Show
      when=move || user_is_logged_in.get()

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
            <Icon icon=IconType::Notifications/>
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
                      .expect("None case for my_user should be handled by ancestor Show component");
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
  }
}

#[server(prefix = "/serverfn")]
pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
  use actix_web::{
    cookie::{Cookie, SameSite},
    http::{header, header::HeaderValue},
  };
  use leptos_actix::ResponseOptions;

  let response = expect_context::<ResponseOptions>();

  let cookie = Cookie::build("theme", theme)
    .path("/")
    .secure(!cfg!(debug_assertions))
    .same_site(SameSite::Strict)
    .finish();

  if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
    response.insert_header(header::SET_COOKIE, cookie);
  }

  Ok(())
}

#[component]
fn ThemeSelect() -> impl IntoView {
  let theme_action = Action::<ChangeTheme, _>::server();
  let theme = expect_context::<ThemeResource>();

  Effect::new(move |_| {
    if theme_action.version().get() > 0 {
      theme.refetch();
    }
  });

  view! {
    <li class="z-[1]">
      <details>
        <summary>"Theme"</summary>
        <ul>
          <li>
            <ActionForm action=theme_action>
              <input type="hidden" name="theme" value=Theme::Dark/>
              <button type="submit">"Dark"</button>
            </ActionForm>
          </li>
          <li>
            <ActionForm action=theme_action>
              <input type="hidden" name="theme" value=Theme::Light/>
              <button type="submit">"Light"</button>
            </ActionForm>
          </li>
          <li>
            <ActionForm action=theme_action>
              <input type="hidden" name="theme" value=Theme::Retro/>
              <button type="submit">"Retro"</button>
            </ActionForm>
          </li>
        </ul>
      </details>
    </li>
  }
}

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();

  view! {
    <nav class="navbar bg-gradient-to-br from-base-100 to-base-200 to-90% shadow-lg">
      <div class="navbar-start">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <Transition>
              <InstanceName/>
            </Transition>
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
                <Icon icon=IconType::Donate/>
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
                <Icon icon=IconType::Search/>
              </span>
            </A>
          </li>
          <Transition>
            <ThemeSelect/>
            <LoggedInUserActionDropdown/>
          </Transition>
        </ul>
      </div>
    </nav>
  }
}
