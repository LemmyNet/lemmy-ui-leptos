use crate::ui::components::common::icon::{Icon, IconType};
use leptos::*;
use leptos_fluent::tr;
use leptos_router::*;

#[component]
pub fn MobileNav() -> impl IntoView {
  view! {
    <nav aria-label=tr!("mobile-nav") class="btm-nav w-full md:hidden border-t border-neutral text-xs">
      <NavLink href="/" icon=IconType::Home text=tr!("home") />
      <NavLink href="/communities" icon=IconType::Communities text=tr!("communities") />
      <NavLink href="/search" icon=IconType::Search text=tr!("search") />
      <NavLink href="/saved" icon=IconType::Saved text=tr!("saved") />
      <NavLink href="/" icon=IconType::Profile text=tr!("profile") />
    </nav>
  }
}

#[component]
fn NavLink(href: &'static str, icon: IconType, text: String) -> impl IntoView {
  // TODO: Apply active to aria-current=page once the unusual cargo-leptos bug is resolved
  view! {
    <A href=href>
      <Icon icon=icon />
      <span class="block">{text}</span>
    </A>
  }
}
