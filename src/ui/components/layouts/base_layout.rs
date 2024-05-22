use crate::{
  contexts::theme_resource_context::ThemeResource,
  ui::components::{
    common::unpack::Unpack,
    layouts::base_layout::{side_nav::SideNav, top_nav::TopNav},
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
        <Html attr:data-theme=theme/>
        <TopNav/>
        <div class="flex gap-x-4">
          <SideNav/>
          <div class="grow">
            <Outlet/>
          </div>
        </div>
      </Unpack>
    </Transition>
  }
}
