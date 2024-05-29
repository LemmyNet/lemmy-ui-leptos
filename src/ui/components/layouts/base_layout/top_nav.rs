use crate::{
  contexts::{site_resource_context::SiteResource, theme_resource_context::ThemeResource},
  i18n::*,
  serverfns::auth::create_logout_action,
  ui::components::common::{
    icon::{Icon, IconSize, IconType},
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
      <A href="/" class="block navbar-start text-xl whitespace-nowrap">
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
    <nav>
      <ul aria-label="Authentication nav" class="flex gap-x-2">
        <Show
          when=move || user_is_logged_in.get()

          fallback=move || {
              view! {
                <li>
                  <A href="/login" class="btn btn-ghost transition duration-500">
                    {t!(i18n, login)}
                  </A>
                </li>
                <li>
                  <A href="/signup" class="btn btn-primary transition duration-500">

                    {t!(i18n, signup)}
                  </A>
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
    </nav>
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
    <details class="dropdown dropdown-end group">
      <summary class="btn btn-circle btn-ghost relative" aria-label="Theme">
        <Icon class="absolute left-1 inset-y-auto" icon=IconType::Theme/>
        <Icon
          class="absolute right-2.5 bottom-1 group-open:rotate-180 transition-transform"
          icon=IconType::DropdownCaret
          size=IconSize::Small
        />
      </summary>
      <ul class="p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box">
        <li>
          <ActionForm action=theme_action class="p-0">
            <input type="hidden" name="theme" value=Theme::Dark/>
            <button type="submit" class="p-2.5">
              "Dark"
            </button>
          </ActionForm>
        </li>
        <li>
          <ActionForm action=theme_action class="p-0">
            <input type="hidden" name="theme" value=Theme::Light/>
            <button type="submit" class="p-2.5">
              "Light"
            </button>
          </ActionForm>
        </li>
        <li>
          <ActionForm action=theme_action class="p-0">
            <input type="hidden" name="theme" value=Theme::Retro/>
            <button type="submit" class="p-2.5">
              "Retro"
            </button>
          </ActionForm>
        </li>
      </ul>
    </details>
  }
}

#[component]
pub fn TopNav() -> impl IntoView {
  view! {
    <nav class="navbar bg-gradient-to-br from-base-100 to-base-200 to-90% shadow-lg px-7">
      <Transition>
        <InstanceName/>
      </Transition>
      <div class="navbar-end gap-x-3">
        <Transition>
          <ThemeSelect/>
          <LoggedInUserActionDropdown/>
        </Transition>
      </div>
    </nav>
  }
}
