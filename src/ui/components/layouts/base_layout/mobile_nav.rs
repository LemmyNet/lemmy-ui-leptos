use crate::ui::components::common::icon::{Icon, IconType};
use leptos::*;
use leptos_router::*;

#[component]
pub fn MobileNav() -> impl IntoView {
  view! {
    <nav aria-label="Mobile nav" class="btm-nav w-full md:hidden border-t border-neutral text-xs">
      <A href="/">
        <Icon icon=IconType::Home/>
        <span class="block">Home</span>
      </A>
      <A href="/communities">
        <Icon icon=IconType::Communities/>
        <span class="block">Communities</span>
      </A>
      <A href="/search">
        <Icon icon=IconType::Search/>
        <span class="block">Search</span>
      </A>
      <A href="/saved">
        <Icon icon=IconType::Saved/>
        <span class="block">Saved</span>
      </A>
      <A href="/">
        <Icon icon=IconType::Profile/>
        <span class="block">Profile</span>
      </A>
    </nav>
  }
}
