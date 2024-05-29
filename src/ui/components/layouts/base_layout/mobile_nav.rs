use crate::ui::components::common::icon::{Icon, IconType};
use leptos::*;
use leptos_router::*;

#[component]
pub fn MobileNav() -> impl IntoView {
  view! {
    <nav aria-label="Mobile nav" class="btm-nav w-full md:hidden border-t border-neutral">
      <A href="/search">
        <Icon icon=IconType::Search/>
      </A>
      <A href="/communities">
        <Icon icon=IconType::Communities/>
      </A>
      <A href="/">
        <Icon icon=IconType::Home/>
      </A>
      <A href="/saved">
        <Icon icon=IconType::Saved/>
      </A>
      <A href="/">
        <Icon icon=IconType::Profile/>
      </A>
    </nav>
  }
}
