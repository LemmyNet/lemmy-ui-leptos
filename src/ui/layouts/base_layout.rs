use crate::ui::components::common::nav::{BottomNav, TopNav};
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn BaseLayout() -> impl IntoView {
  view! {
    <Transition>
      <TopNav/>
      <Outlet/>
      <BottomNav/>
    </Transition>
  }
}
