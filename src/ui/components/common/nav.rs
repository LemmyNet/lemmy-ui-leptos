use crate::i18n::*;
use leptos::{component, view, IntoAttribute, IntoView};
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();

  view! {
    <nav class="container navbar mx-auto">
      <div class="navbar-start">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <A href="/" class="text-xl whitespace-nowrap">
              "Brand from env"
            </A>
          </li>
          <li>
            <A href="/communities" class="text-md">
              {t!(i18n, nav_communities)}
            </A>
          </li>
          <li>
            <A href="/create_post" class="text-md">
              {t!(i18n, nav_create_post)}
            </A>
          </li>
          <li>
            <A href="/create_community" class="text-md">
              {t!(i18n, nav_create_community)}
            </A>
          </li>
          <li>
            <a href="//join-lemmy.org/donate" title=t!(i18n, nav_donate)>
              <Icon icon=Icon::from(ChIcon::ChHeart) class="h-6 w-6"/>
            </a>
          </li>
        </ul>
      </div>
      <div class="navbar-end">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <A href="/search">
              <span title=t!(i18n, nav_search)>
                <Icon icon=Icon::from(ChIcon::ChSearch) class="h-6 w-6"/>
              </span>
            </A>
          </li>
          <li>
            <A href="/login">{t!(i18n, nav_login)}</A>
          </li>
          <li>
            <A href="/signup">{t!(i18n, nav_signup)}</A>
          </li>
          <li>
            <A href="/inbox">
              <span title=t!(i18n, nav_unread_messages)>
                <Icon icon=Icon::from(ChIcon::ChBell) class="h-6 w-6"/>
              </span>
            </A>
          </li>
          <li>
            <details>
              <summary>"User name"</summary>
              <ul>
                <li>
                  <A href="/u/jimmy90">{t!(i18n, nav_profile)}</A>
                </li>
                <li>
                  <A href="/settings">{t!(i18n, nav_settings)}</A>
                </li>
                <li>
                  <hr/>
                </li>
                <li>
                  <A href="/logout">{t!(i18n, nav_logout)}</A>
                </li>
              </ul>
            </details>
          </li>
        </ul>
      </div>
    </nav>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();

  view! {
    <nav class="container navbar mx-auto">
      <div class="navbar-start"></div>
      <div class="navbar-end ">
        <ul class="menu menu-horizontal flex-nowrap">
          <li>
            <a href="//github.com/LemmyNet/lemmy-ui-leptos/releases" class="text-md">
              "f/e from env"
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
              "b/e from env"
            </a>
          </li>
          <li>
            <A href="/modlog" class="text-md">
              {t!(i18n, nav_modlog)}
            </A>
          </li>
          <li>
            <A href="/instances" class="text-md">
              {t!(i18n, nav_instances)}
            </A>
          </li>
          <li>
            <a href="join-lemmy.org/docs/en/index.html" class="text-md">
              {t!(i18n, nav_docs)}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet" class="text-md">
              {t!(i18n, nav_code)}
            </a>
          </li>
          <li>
            <a href="//join-lemmy.org" class="text-md">
              "join-lemmy.org"
            </a>
          </li>
        </ul>
      </div>
    </nav>
  }
}
