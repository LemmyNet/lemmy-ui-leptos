use crate::{
  contexts::theme_resource_context::ThemeResource,
  ui::components::{
    common::unpack::Unpack,
    layouts::base_layout::{mobile_nav::MobileNav, side_nav::SideNav, top_nav::TopNav},
  },
};
use leptos::*;
use leptos_meta::Html;
use leptos_router::Outlet;

mod mobile_nav;
mod side_nav;
mod top_nav;

#[component]
pub fn BaseLayout() -> impl IntoView {
  let theme = expect_context::<ThemeResource>();

  view! {
    <div class="drawer h-full">
      <input id="mobile-drawer" type="checkbox" class="drawer-toggle"/>
      <div class="drawer-content h-full min-h-screen md:max-h-screen">
        <Transition>
          <Unpack item=theme let:theme>
            <Html attr:data-theme=theme class="h-full max-h-screen overflow-y-hidden"/>
            <TopNav/>
            <div class="flex gap-x-4 h-full min-h-0 overflow-y-auto">
              <SideNav/>
              <div class="grow h-fit">
                <Outlet/>
              </div>
            </div>
          </Unpack>
        </Transition>
        <MobileNav/>
      </div>
      <div class="drawer-side">
        <label for="mobile-drawer" aria-label="Close mobile drawer" class="drawer-overlay"></label>
        <p>Drawer stuff</p>
      </div>
    </div>
  }
}
