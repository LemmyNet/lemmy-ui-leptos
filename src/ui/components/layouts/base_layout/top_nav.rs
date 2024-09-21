use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::{
    common::{
      icon::{Icon, IconType},
      unpack::Unpack,
    },
    layouts::base_layout::top_nav::{
      auth_dropdown::AuthDropdown,
      notification_bell::NotificationBell,
      theme_select::ThemeSelect,
    },
  },
  utils::derive_query_signal,
};
use leptos::*;
use leptos_router::*;

mod auth_dropdown;
mod notification_bell;
mod theme_select;

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
pub fn TopNav() -> impl IntoView {
  view! {
    <nav class="navbar bg-gradient-to-br from-base-100 to-base-200 to-90% shadow-lg sm:px-7 z-20">
      <div class="navbar-start sm:hidden">
        <label for="mobile-drawer" aria-label="Open mobile drawer" class="btn btn-square btn-ghost">
          <Icon icon=IconType::Hamburger />
        </label>
      </div>
      <div class="navbar-center sm:navbar-start">
        <Transition>
          <InstanceName />
        </Transition>
      </div>
      <div class="navbar-end gap-x-3">
        <Transition>
          <ThemeSelect />
          <NotificationBell />
          <AuthDropdown />
        </Transition>
      </div>
    </nav>
  }
}
