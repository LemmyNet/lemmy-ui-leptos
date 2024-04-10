use crate::{
  contexts::theme_resource_context::ThemeResource,
  ui::components::common::{
    nav::{BottomNav, TopNav},
    unpack::Unpack,
  },
};
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn BaseLayout() -> impl IntoView {
  let theme = expect_context::<ThemeResource>();
  view! {
    <div class="flex flex-col h-screen" data-theme="retro">
      <TopNav/>
      <Outlet/>
      <BottomNav/>
    </div>
  }
}
