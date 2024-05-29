use crate::{
  contexts::theme_resource_context::ThemeResource,
  ui::components::{
    common::unpack::Unpack,
    layouts::base_layout::{bottom_nav::MobileNav, side_nav::SideNav, top_nav::TopNav},
  },
};
use leptos::*;
use leptos_meta::Html;
use leptos_router::Outlet;

mod bottom_nav;
mod side_nav;
mod top_nav;

#[component]
pub fn BaseLayout() -> impl IntoView {
  let theme = expect_context::<ThemeResource>();

  view! {
    <Transition>
      <Unpack item=theme let:theme>
        <Html attr:data-theme=theme class="h-full max-h-screen overflow-y-hidden"/>
        <TopNav/>
        <div class="flex gap-x-4 h-full min-h-0">
          <SideNav/>
          <div class="grow">
            <Outlet/>
          </div>
        </div>
        <MobileNav />
      </Unpack>
    </Transition>
  }
}
