use crate::ui::components::common::icon::{Icon, IconType};
use leptos::*;
use leptos_router::*;

#[component]
pub fn MobileNav() -> impl IntoView {
  view! {
    <nav aria-label="Mobile nav" class="btm-nav w-full md:hidden border-t border-neutral text-xs">
      <NavLink href="/" icon=IconType::Home text="Home"/>
      <NavLink href="/communities" icon=IconType::Communities text="Communities"/>
      <NavLink href="/search" icon=IconType::Search text="Search"/>
      <NavLink href="/saved" icon=IconType::Saved text="Saved"/>
      <NavLink href="/" icon=IconType::Profile text="Profile"/>
    </nav>
  }
}

#[component]
fn NavLink(href: &'static str, icon: IconType, text: &'static str) -> impl IntoView {
  // TODO: Apply active to aria-current=page once the unusual cargo-leptos bug is resolved
  view! {
    <A href=href>
      <Icon icon=icon/>
      <span class="block">{text}</span>
    </A>
  }
}
