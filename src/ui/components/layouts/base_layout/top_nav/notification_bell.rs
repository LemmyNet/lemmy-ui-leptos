use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::icon::{Icon, IconType},
  use_i18n,
  utils::derive_user_is_logged_in,
};
use leptos::*;
use leptos_i18n::t;
use leptos_router::A;

#[component]
pub fn NotificationBell() -> impl IntoView {
  let i18n = use_i18n();
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);

  view! {
    <Show when=move || user_is_logged_in.get()>
      <A href="/inbox" class="me-2">
        <span title=t!(i18n, unread_messages)>
          <Icon icon=IconType::Notifications />
        </span>
      </A>
    </Show>
  }
}
