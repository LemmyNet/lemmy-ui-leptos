use crate::{
  contexts::theme_resource_context::ThemeResource,
  ui::components::layouts::base_layout::{
    mobile_nav::MobileNav,
    side_nav::SideNav,
    top_nav::TopNav,
  },
};
use leptos::prelude::*;
use leptos_meta::Html;
use leptos_router::components::Outlet;

mod mobile_nav;
mod side_nav;
mod top_nav;

#[component]
pub fn BaseLayout() -> impl IntoView {
  let theme = expect_context::<ThemeResource>();

  view! {
    <div class="drawer h-full">
      <input id="mobile-drawer" type="checkbox" class="drawer-toggle" />
      <div class="drawer-content h-full min-h-screen sm:min-h-0 sm:max-h-screen flex flex-col">
        <Transition>
          {move || Suspend::new(async move {
            theme
              .await
              .ok()
              .map(|theme| {
                view! {
                  <Html attr:data-theme=theme attr:class="h-full max-h-screen overflow-y-hidden" />
                }
              })
          })}
        </Transition>
        <TopNav />
        <div class="flex gap-x-4 h-full overflow-y-auto sm:overflow-y-hidden grow">
          <nav
            aria-label="Navigation Sidebar"
            class="min-w-fit px-3.5 whitespace-nowrap pb-5 pt-3 border-e border-neutral hidden sm:flex flex-col bg-base-300 overflow-y-auto h-full"
          >
            <SideNav />
          </nav>
          <div class="flex-auto overflow-y-auto">
            <Outlet />
          </div>
        </div>
        <MobileNav />
      </div>
      <div class="drawer-side sm:hidden">
        <label for="mobile-drawer" aria-label="Close mobile drawer" class="drawer-overlay"></label>
        <aside class="w-2/3 whitespace-nowrap px-3.5 pb-5 pt-3 flex flex-col bg-base-300 overflow-y-auto h-full border-e border-neutral">
          <SideNav />
        </aside>
      </div>
    </div>
  }
}
