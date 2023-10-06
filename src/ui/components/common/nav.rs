use crate::i18n::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[server(LogoutAction, "serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use actix_session::Session;
  use leptos_actix::extract;

  extract(|session: Session| async move {
    // TODO: Will have to make API call to delete session stored in DB once that feature is implemented on the server
    session.purge();
  })
  .await
}

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();

  let logout_action = create_server_action::<LogoutAction>();

  view! {
    <nav class="navbar container mx-auto">
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
            <a href="join-lemmy.org/donate">
              <span title=t!(i18n, nav_donate)>
                <Icon icon=Icon::from(ChIcon::ChHeart) class="h-6 w-6"/>
              </span>
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
            <ActionForm action=logout_action>
              <button type="submit">{t!(i18n, nav_logout)}</button>
            </ActionForm>
          </li>
        </ul>
      </div>
    </nav>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  view! {
    <footer class="sticky bottom-0">
      <div class="btm-nav btm-nav-lg">
        <A href="/" class="active">
          // TODO put svg's here
          <span class="btm-nav-label">"Home"</span>
        </A>
        <button>
          <span class="btm-nav-label">"TODO 1"</span>
        </button>
        <button>
          <span class="btm-nav-label">"TODO 2"</span>
        </button>
      </div>
    </footer>
  }
}
