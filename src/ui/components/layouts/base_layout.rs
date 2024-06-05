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
      <div class="drawer-content h-full min-h-screen md:min-h-0 md:max-h-screen flex flex-col">
        <Transition>
          <Unpack item=theme let:theme>
            <Html attr:data-theme=theme class="h-full max-h-screen overflow-y-hidden"/>
            <TopNav/>
            <div class="flex gap-x-4 h-full overflow-y-auto md:overflow-y-hidden grow">
              <aside class="w-fit px-3.5 whitespace-nowrap pb-5 pt-3 border-e border-neutral hidden md:flex flex-col gap-y-12 bg-base-300 overflow-y-auto h-full">
                <SideNav/>

              </aside>
              <div class="grow h-fit md:h-auto">
                <Outlet/>
              </div>
            </div>
          </Unpack>
          <MobileNav/>
        </Transition>
      </div>
      <div class="drawer-side md:hidden">
        <label for="mobile-drawer" aria-label="Close mobile drawer" class="drawer-overlay"></label>
        <aside class="w-2/3 h-full whitespace-nowrap bg-base-300 overflow-y-auto flex flex-col">
          <SideNav/>
        </aside>
      </div>
    </div>
  }
}
