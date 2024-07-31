use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::icon::{Icon, IconType},
  utils::derive_user_is_logged_in,
};
use leptos::*;
use leptos_fluent::tr;
use leptos_router::A;

#[component]
pub fn NotificationBell() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);

  view! {
    <Show when=move || user_is_logged_in.get()>
      <A href="/inbox" class="me-2">
        <span title=tr!("unread-messages")>
          <Icon icon=IconType::Notifications />
        </span>
      </A>
    </Show>
  }
}
