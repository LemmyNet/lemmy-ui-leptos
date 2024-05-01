#![allow(clippy::empty_docs)]

mod constants;
mod contexts;
pub mod host;
#[cfg(feature = "ssr")]
pub mod server;
mod serverfns;
mod ui;
mod utils;

use crate::{
  contexts::{
    site_resource_context::provide_site_resource_context,
    theme_resource_context::provide_theme_resource_context,
  },
  i18n::*,
  ui::{
    components::{
      communities::communities_activity::CommunitiesActivity,
      home::home_activity::HomeActivity,
      login::login_activity::LoginActivity,
      post::post_activity::PostActivity,
    },
    layouts::{base_layout::BaseLayout, filter_bar_layout::FilterBarLayout},
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

      {
          #[cfg(any(feature = "ssr", feature = "hydrate"))]
          view! {
            <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
            <Link rel="shortcut icon" href="/favicon.svg"/>
          }
      }

      <Meta name="description" content="Lemmy-UI-Leptos."/>
      <Meta name="viewport" content="viewport-fit=cover"/>
      // debug where there is no visible console (mobile/live/desktop)
      // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
      // <Script>eruda.init();</Script>
      <Title text="Brand from env"/>

      <Routes>
        <Route path="" view=BaseLayout ssr=SsrMode::Async>
          <Route path="/*any" view=NotFound/>

          <Route path="" view=FilterBarLayout>
            <Route path="" view=HomeActivity/>
          </Route>

          <Route path="create_post" view=CommunitiesActivity/>
          <Route path="post/:id" view=PostActivity/>

          <Route path="search" view=CommunitiesActivity/>
          <Route path="communities" view=CommunitiesActivity/>
          <Route path="create_community" view=CommunitiesActivity/>
          <Route path="c/:id" view=CommunitiesActivity/>

          <Route
            path="login"
            view=move || {
                view! {
                  <AnonymousOnlyRouteView>
                    <LoginActivity/>
                  </AnonymousOnlyRouteView>
                }
            }
          />
          <Route
            path="signup"
            view=move || {
                view! {
                  <AnonymousOnlyRouteView>
                    <CommunitiesActivity/>
                  </AnonymousOnlyRouteView>
                }
            }
          />

          <Route path="inbox" view=CommunitiesActivity/>
          <Route path="settings" view=CommunitiesActivity/>
          <Route path="u/:id" view=CommunitiesActivity/>

          <Route path="modlog" view=CommunitiesActivity/>
          <Route path="instances" view=CommunitiesActivity/>
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
        when=move || !user_is_logged_in()
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

        {children()}
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
