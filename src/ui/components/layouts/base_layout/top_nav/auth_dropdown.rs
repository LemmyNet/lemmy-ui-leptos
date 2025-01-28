use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::auth::create_logout_action,
  ui::components::common::icon::{Icon, IconSize, IconType},
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::{either::Either, prelude::*};
use leptos_fluent::move_tr;
use leptos_router::components::A;

#[component]
pub fn AuthDropdown() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();

  let logout_action = create_logout_action();

  Effect::new(move |_| {
    if logout_action.version().get() > 0 {
      site_resource.refetch();
    }
  });

  view! {
    <nav class="hidden sm:block">
      <ul aria-label=move_tr!("authentication-nav") class="flex items-center gap-x-2">
        {move || Suspend::new(async move {
          site_resource
            .await
            .map(|site| {
              site
                .my_user
                .map(|my_user| {
                  let Person { ref name, display_name, .. } = my_user.local_user_view.person;
                  Either::Left(

                    view! {
                      <li>
                        <details
                          class="dropdown dropdown-end group"
                          aria-label=move_tr!("logged-in-user-dropdown")
                        >
                          <summary class="btn">

                            <span class="text-nowrap leading-loose">

                              {
                              display_name.unwrap_or(name.clone())} " "
                              <Icon
                                class="align-bottom inline group-open:rotate-180 transition-transform"
                                icon=IconType::DropdownCaret
                                size=IconSize::Small
                              />
                            </span>

                          </summary>
                          <ul class="*:p-0 p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box">
                            <li>
                              <A href=
                              format!("/u/{name}")>{move_tr!("profile")}</A>
                            </li>
                            <li>
                              <A href="/settings">{move_tr!("settings")}</A>
                            </li>
                            <div class="divider my-0"></div>
                            <li>
                              <ActionForm action=logout_action attr:class="p-0">
                                <button type="submit" class="p-2.5">
                                  {move_tr!("logout")}
                                </button>
                              </ActionForm>
                            </li>
                          </ul>
                        </details>
                      </li>
                    },
                  )
                })
                .unwrap_or_else(|| Either::Right(
                  view! {
                    <li>
                      <A href="/login" attr:class="btn btn-ghost transition duration-500">
                        {move_tr!("login")}
                      </A>
                    </li>
                    <li>
                      <A href="/signup" attr:class="btn btn-primary transition duration-500">

                        {move_tr!("signup")}
                      </A>
                    </li>
                  },
                ))
            })
        })}
      </ul>
    </nav>
  }
}
