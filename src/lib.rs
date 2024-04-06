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
  contexts::site_resource_context::provide_site_resource_context,
  i18n::*,
  ui::{
    components::{
      common::with_filter_bar::WithFilterBar,
      communities::communities_activity::CommunitiesActivity,
      home::home_activity::HomeActivity,
      login::login_activity::LoginActivity,
      post::post_activity::PostActivity,
    },
    layouts::base_layout::BaseLayout,
  },
};
use leptos::*;
use leptos_meta::*;
#[cfg(debug_assertions)]
use leptos_router::*;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();
  provide_site_resource_context();

  let ui_theme = RwSignal::new(String::from("retro"));

  let is_routing = RwSignal::new(false);

  view! {
    <Router set_is_routing=is_routing>
      <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
      <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
      <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
      <Meta name="description" content="Lemmy-UI-Leptos."/>
      <Meta name="viewport" content="viewport-fit=cover"/>
      // debug where there is no visible console (mobile/live/desktop)
      // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
      // <Script>eruda.init();</Script>
      <Title text="Brand from env"/>

      <div class="flex flex-col h-screen" data-theme=ui_theme>

        <Routes>
          <Route path="" view=BaseLayout>
            <Route path="/*any" view=NotFound/>

            <Route
              path=""
              view=move || {
                  view! {
                    <WithFilterBar>
                      <HomeActivity/>
                    </WithFilterBar>
                  }
              }

              ssr=SsrMode::Async
            />

            <Route path="create_post" view=CommunitiesActivity/>
            <Route path="post/:id" view=PostActivity/>

            <Route path="search" view=CommunitiesActivity/>
            <Route path="communities" view=CommunitiesActivity/>
            <Route path="create_community" view=CommunitiesActivity/>
            <Route path="c/:id" view=CommunitiesActivity/>

            <Route path="login" view=LoginActivity ssr=SsrMode::Async/>
            <Route path="signup" view=CommunitiesActivity/>

            <Route path="inbox" view=CommunitiesActivity/>
            <Route path="settings" view=CommunitiesActivity/>
            <Route path="u/:id" view=CommunitiesActivity/>

            <Route path="modlog" view=CommunitiesActivity/>
            <Route path="instances" view=CommunitiesActivity/>
          </Route>
        </Routes>
      </div>
    </Router>
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
