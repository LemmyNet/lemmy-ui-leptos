use crate::{
  ui::components::common::icon::{Icon, IconType},
  use_i18n,
};
use leptos::*;
use leptos_i18n::t;
use leptos_router::A;

#[component]
pub fn SideNav() -> impl IntoView {
  let i18n = use_i18n();

  view! {
    <aside class="w-fit px-5 whitespace-nowrap pt-8 border-e border-neutral">
      <nav>
        <menu class="flex flex-col gap-2">
          <NavLink href="/create_post" icon=IconType::CreatePost text=t!(i18n, create_post)/>
          <NavLink
            href="/create_community"
            icon=IconType::CreateCommunity
            text=t!(i18n, create_community)
          />
          <NavLink href="/communities" icon=IconType::Communities text=t!(i18n, communities)/>
        // <li>
        // <a href="//join-lemmy.org/donate">
        // <span title="t!(i18n, donate)">
        // <Icon icon=IconType::Donate/>
        // </span>
        // </a>
        // </li>
        </menu>
      </nav>
    </aside>
  }
}

#[component]
fn NavLink(href: &'static str, icon: IconType, #[prop(into)] text: TextProp) -> impl IntoView {
  view! {
    <li>
      <A href=href class="block text-md leading-relaxed hover:bg-base-200 p-1.5 rounded-md">
        <Icon icon=icon class="inline me-1.5"/>
        <span class="align-bottom">{text}</span>
      </A>
    </li>
  }
}
