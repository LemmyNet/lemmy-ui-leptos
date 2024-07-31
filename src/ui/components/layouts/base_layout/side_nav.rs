use crate::ui::components::common::icon::{Icon, IconType};
use leptos::*;
use leptos_fluent::tr;
use leptos_router::A;

#[component]
pub fn SideNav() -> impl IntoView {
  view! {
    <section aria-label="Main navigation links">
      <ul>
        <NavLink href="/create_post" icon=IconType::CreatePost text=tr!("create-post") />
        <NavLink
          href="/create_community"
          icon=IconType::CreateCommunity
          text=tr!("create-community")
        />
        <NavLink href="/communities" icon=IconType::Communities text=tr!("communities") />
        <NavLink href="/search" icon=IconType::Search text=tr!("search") />
        <NavLink href="/modlog" icon=IconType::Modlog text=tr!("modlog") />
        <NavLink href="/instances" icon=IconType::Instances text=tr!("instances") />
        <NavLink href="/legal" icon=IconType::Legal text=tr!("legal") />
      </ul>
    </section>
    <section aria-labelledby="lemmy-resources-label" class="mt-auto">
      <div id="lemmy-resources-label" class="font-medium mb-1">
        Lemmy Resources
      </div>
      <ul>
        <NavLink
          href="https://join-lemmy.org/docs/en/index.html"
          icon=IconType::Documentation
          text=tr!("documentation")
        />
        <NavLink href="https://github.com/LemmyNet" icon=IconType::Code text=tr!("source-code") />
        <NavLink href="https://join-lemmy.org/" icon=IconType::Info text=tr!("about") />
        <NavLink href="https://join-lemmy.org/donate" icon=IconType::Donate text=tr!("donate") />
      </ul>
    </section>
  }
}

#[component]
fn NavLink(href: &'static str, icon: IconType, #[prop(into)] text: TextProp) -> impl IntoView {
  view! {
    <li>
      <A
        href=href
        class="text-sm block leading-relaxed hover:bg-base-100 p-1.5 rounded-md transition duration-500 my-1 aria-current-page:bg-secondary aria-current-page:text-neutral aria-current-page:hover:bg-secondary/[0.6]"
      >
        <Icon icon=icon class="inline me-1.5" />
        <span class="align-bottom">{text}</span>
      </A>
    </li>
  }
}
