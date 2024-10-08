use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::auth::create_logout_action,
  ui::components::common::{
    icon::{Icon, IconSize, IconType},
    unpack::Unpack,
  },
  utils::{derive_query_signal, derive_user_is_logged_in},
};
use leptos::*;
use leptos_fluent::move_tr;
use leptos_router::{ActionForm, A};

#[component]
pub fn AuthDropdown() -> impl IntoView {
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
    <nav class="hidden sm:block">
      <ul aria-label=move_tr!("authentication-nav") class="flex items-center gap-x-2">
        <Show
          when=move || user_is_logged_in.get()

          fallback=move || {
            view! {
              <li>
                <A href="/login" class="btn btn-ghost transition duration-500">
                  {move_tr!("login")}
                </A>
              </li>
              <li>
                <A href="/signup" class="btn btn-primary transition duration-500">

                  {move_tr!("signup")}
                </A>
              </li>
            }
          }
        >

          <Unpack item=names let:names>
            <li>
              <details
                class="dropdown dropdown-end group"
                aria-label=move_tr!("logged-in-user-dropdown")
              >
                <summary class="btn">

                  <span class="text-nowrap leading-loose">

                    {
                      let (name, display_name) = names
                        .as_ref()
                        .expect(
                          "None case for my_user should be handled by ancestor Show component",
                        );
                      display_name.as_ref().unwrap_or(name)
                    } " "
                    <Icon
                      class="align-bottom inline group-open:rotate-180 transition-transform"
                      icon=IconType::DropdownCaret
                      size=IconSize::Small
                    />
                  </span>

                </summary>
                <ul class="*:p-0 p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box">
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
                    }>{move_tr!("profile")}</A>
                  </li>
                  <li>
                    <A href="/settings">{move_tr!("settings")}</A>
                  </li>
                  <div class="divider my-0"></div>
                  <li>
                    <ActionForm action=logout_action class="p-0">
                      <button type="submit" class="p-2.5">
                        {move_tr!("logout")}
                      </button>
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
