#![allow(clippy::empty_docs)]

mod constants;
mod contexts;
#[cfg(feature = "ssr")]
pub mod cookie_middleware;
pub mod host;
mod serverfns;
mod ui;
mod utils;

use crate::{
  contexts::{
    site_resource_context::provide_site_resource_context,
    theme_resource_context::provide_theme_resource_context,
  },
  i18n::*,
  ui::components::{
    communities::communities_page::CommunitiesPage,
    home::home_page::HomePage,
    layouts::{base_layout::BaseLayout, filter_bar_layout::FilterBarLayout},
    login::login_page::LoginPage,
    post::post_page::PostPage,
  },
};
use contexts::site_resource_context::SiteResource;
use leptos::*;
use leptos_meta::*;
#[cfg(debug_assertions)]
use leptos_router::*;
use utils::derive_user_is_logged_in;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();
  provide_site_resource_context();
  provide_theme_resource_context();

  let is_routing = RwSignal::new(false);

  view! {
    <Router set_is_routing=is_routing>
      <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>

      <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
      <Link rel="shortcut icon" href="/favicon.svg"/>

      <Meta name="description" content="Lemmy-UI-Leptos."/>
      <Meta name="viewport" content="viewport-fit=cover"/>
      // debug where there is no visible console (mobile/live/desktop)
      // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
      // <Script>eruda.init();</Script>
      <Title text="Brand from env"/>
      <Body class="h-full max-h-screen flex flex-col overflow-y-hidden"/>

      <Routes>
        <Route path="" view=BaseLayout ssr=SsrMode::Async>
          <Route path="/*any" view=NotFound/>

          <Route path="" view=FilterBarLayout>
            <Route path="" view=HomePage/>
          </Route>

          <Route path="create_post" view=CommunitiesPage/>
          <Route path="post/:id" view=PostPage/>

          <Route path="search" view=CommunitiesPage/>
          <Route path="communities" view=CommunitiesPage/>
          <Route path="create_community" view=CommunitiesPage/>
          <Route path="c/:id" view=CommunitiesPage/>

          <Route
            path="login"
            view=move || {
                view! {
                  <AnonymousOnlyRouteView>
                    <LoginPage/>
                  </AnonymousOnlyRouteView>
                }
            }
          />

          <Route
            path="signup"
            view=move || {
                view! {
                  <AnonymousOnlyRouteView>
                    <CommunitiesPage/>
                  </AnonymousOnlyRouteView>
                }
            }
          />

          <Route path="inbox" view=CommunitiesPage/>
          <Route path="settings" view=CommunitiesPage/>
          <Route path="u/:id" view=CommunitiesPage/>

          <Route path="modlog" view=CommunitiesPage/>
          <Route path="instances" view=CommunitiesPage/>
          <Route path="legal" view=CommunitiesPage/>
        </Route>
      </Routes>
    </Router>
  }
}

#[component]
fn AnonymousOnlyRouteView(children: ChildrenFn) -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let children = StoredValue::new(children);

  view! {
    <Transition>
      <Show
        when=move || !user_is_logged_in.get()
        fallback=move || {
            view! {
              <Redirect
                path="/"
                options=NavigateOptions {
                    replace: true,
                    ..Default::default()
                }
              />
            }
        }
      >

        {children.get_value()}
      </Show>
    </Transition>
  }
}

#[component]
fn NotFound() -> impl IntoView {
  #[cfg(feature = "ssr")]
  {
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! { <h1>"Not Found"</h1> }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}
