use crate::ui::components::common::icon::{Icon, IconType};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_router::components::A;

#[component]
pub fn MobileNav() -> impl IntoView {
  view! {
    <nav
      aria-label=move_tr!("mobile-nav")
      class="btm-nav w-full sm:hidden border-t border-neutral text-xs"
    >
      <NavLink href="/" icon=IconType::Home text=move_tr!("home") />
      <NavLink href="/communities" icon=IconType::Communities text=move_tr!("communities") />
      <NavLink href="/search" icon=IconType::Search text=move_tr!("search") />
      <NavLink href="/saved" icon=IconType::Saved text=move_tr!("saved") />
      <NavLink href="/" icon=IconType::Profile text=move_tr!("profile") />
    </nav>
  }
}

#[component]
fn NavLink(href: &'static str, icon: IconType, text: Signal<String>) -> impl IntoView {
  // TODO: Apply active to aria-current=page once the relevant Daisy UI issue is fixed: https://github.com/saadeghi/daisyui/issues/3170
  view! {
    <A href=href>
      <Icon icon=icon />
      <span class="block">{text}</span>
    </A>
  }
}
