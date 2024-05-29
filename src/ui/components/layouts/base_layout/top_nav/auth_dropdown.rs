use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::auth::create_logout_action,
  ui::components::common::{
    icon::{Icon, IconType},
    unpack::Unpack,
  },
  use_i18n,
  utils::{derive_query_signal, derive_user_is_logged_in},
};
use leptos::*;
use leptos_i18n::t;
use leptos_router::{ActionForm, A};

#[component]
pub fn AuthDropdown() -> impl IntoView {
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
