use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::{
    common::icon::{Icon, IconType},
    layouts::base_layout::top_nav::{
      auth_dropdown::AuthDropdown,
      notification_bell::NotificationBell,
      theme_select::ThemeSelect,
    },
  },
};
use leptos::prelude::*;
use leptos_router::components::A;

mod auth_dropdown;
mod notification_bell;
mod theme_select;

#[component]
fn InstanceName() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();

  move || {
    Suspend::new(async move {
      site_resource.await.map(|site_response| {
        view! {
          <A href="/" attr:class="block navbar-start text-xl whitespace-nowrap">
            {site_response.site_view.site.name}
          </A>
        }
      })
    })
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
