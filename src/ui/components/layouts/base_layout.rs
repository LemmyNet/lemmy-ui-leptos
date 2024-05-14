use crate::{
  contexts::theme_resource_context::ThemeResource,
  ui::components::{
    common::unpack::Unpack,
    layouts::base_layout::{bottom_nav::BottomNav, top_nav::TopNav},
  },
};
use leptos::*;
use leptos_meta::Html;
use leptos_router::Outlet;

mod bottom_nav;
mod top_nav;

#[component]
pub fn BaseLayout() -> impl IntoView {
  let theme = expect_context::<ThemeResource>();

  view! {
    <Transition>
      <Unpack item=theme let:theme>
        <Html attr:data-theme=theme/>
        <TopNav/>
        <Outlet/>
        <BottomNav/>
      </Unpack>
    </Transition>
  }
}
