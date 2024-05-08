use crate::{
  contexts::theme_resource_context::ThemeResource,
  ui::components::common::{
    nav::{BottomNav, TopNav},
    unpack::Unpack,
  },
};
use leptos::*;
use leptos_meta::Html;
use leptos_router::Outlet;

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
