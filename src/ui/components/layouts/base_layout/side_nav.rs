use crate::ui::components::common::icon::{Icon, IconType};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_router::components::A;

#[component]
pub fn SideNav() -> impl IntoView {
  view! {
    <section aria-label=move_tr!("main-nav-links")>
      <ul>
        <NavLink href="/create_post" icon=IconType::CreatePost text=move_tr!("create-post") />
        <NavLink
          href="/create_community"
          icon=IconType::CreateCommunity
          text=move_tr!("create-community")
        />
        <NavLink href="/communities" icon=IconType::Communities text=move_tr!("communities") />
        <NavLink href="/search" icon=IconType::Search text=move_tr!("search") />
        <NavLink href="/modlog" icon=IconType::Modlog text=move_tr!("modlog") />
        <NavLink href="/instances" icon=IconType::Instances text=move_tr!("instances") />
        <NavLink href="/legal" icon=IconType::Legal text=move_tr!("legal") />
      </ul>
    </section>
    <section aria-labelledby="lemmy-resources-label" class="mt-auto">
      <div id="lemmy-resources-label" class="font-medium mb-1">
        {move_tr!("lemmy-resources")}
      </div>
      <ul>
        <NavLink
          href="https://join-lemmy.org/docs/en/index.html"
          icon=IconType::Documentation
          text=move_tr!("documentation")
        />
        <NavLink
          href="https://github.com/LemmyNet"
          icon=IconType::Code
          text=move_tr!("source-code")
        />
        <NavLink href="https://join-lemmy.org/" icon=IconType::Info text=move_tr!("about") />
        <NavLink
          href="https://join-lemmy.org/donate"
          icon=IconType::Donate
          text=move_tr!("donate")
        />
      </ul>
    </section>
  }
}

#[component]
fn NavLink(href: &'static str, icon: IconType, text: Signal<String>) -> impl IntoView {
  view! {
    <li>
      <A
        href=href
        attr:class="text-sm block leading-relaxed hover:bg-base-100 p-1.5 rounded-md transition duration-500 my-1 aria-current-page:bg-secondary aria-current-page:text-neutral aria-current-page:hover:bg-secondary/[0.6]"
      >
        <Icon icon=icon class="inline me-1.5" />
        <span class="align-bottom">{text}</span>
      </A>
    </li>
  }
}
