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
    <aside class="w-fit px-3.5 whitespace-nowrap pb-5 pt-3 border-e border-neutral hidden md:flex flex-col gap-y-12 bg-base-300 overflow-y-auto">
      <nav aria-label="Pages nav">
        <ul>
          <NavLink href="/create_post" icon=IconType::CreatePost text=t!(i18n, create_post)/>
          <NavLink
            href="/create_community"
            icon=IconType::CreateCommunity
            text=t!(i18n, create_community)
          />
          <NavLink href="/communities" icon=IconType::Communities text=t!(i18n, communities)/>
          <NavLink href="/search" icon=IconType::Search text=t!(i18n, search)/>
          <NavLink href="/modlog" icon=IconType::Modlog text=t!(i18n, modlog)/>
          <NavLink href="/instances" icon=IconType::Instances text=t!(i18n, instances)/>
          <NavLink href="/legal" icon=IconType::Legal text=t!(i18n, legal)/>
        </ul>
      </nav>
      <nav aria-labelledby="lemmy-resources-label" class="mt-auto">
        <div id="lemmy-resources-label" class="font-medium mb-1">
          Lemmy Resources
        </div>
        <ul>
          <NavLink
            href="https://join-lemmy.org/docs/en/index.html"
            icon=IconType::Documentation
            text=t!(i18n, docs)
          />
          <NavLink href="https://github.com/LemmyNet" icon=IconType::Code text=t!(i18n, code)/>
          <NavLink href="https://join-lemmy.org/" icon=IconType::Info text=t!(i18n, about)/>
          <NavLink
            href="https://join-lemmy.org/donate"
            icon=IconType::Donate
            text=t!(i18n, donate)
          />
        </ul>
      </nav>
    </aside>
  }
}

#[component]
fn NavLink(href: &'static str, icon: IconType, #[prop(into)] text: TextProp) -> impl IntoView {
  view! {
    <li>
      <A
        href=href
        class="text-sm block leading-relaxed hover:bg-base-100 p-1.5 rounded-md transition duration-500 my-1"
      >
        <Icon icon=icon class="inline me-1.5"/>
        <span class="align-bottom">{text}</span>
      </A>
    </li>
  }
}
