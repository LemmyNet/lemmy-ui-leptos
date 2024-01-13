#![allow(warnings)]

mod config;
mod errors;
mod host;
mod layout;
mod lemmy_client;
mod lemmy_errors;
mod queries;
#[cfg(feature = "ssr")]
pub mod server;
mod ui;

use crate::{
  i18n::*,
  layout::Layout,
  queries::site_state_query::use_site_state,
  ui::components::{
    communities::communities_activity::CommunitiesActivity,
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  },
};
use cfg_if::cfg_if;
use lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::*;
use leptos_meta::*;
use leptos_query::{provide_query_client, QueryResult};
use leptos_router::*;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();
  provide_query_client();

  let ui_theme = create_rw_signal::<String>(String::from("retro"));
  provide_context(ui_theme);

  let (is_routing, set_is_routing) = create_signal(false);

  view! {
    <Router set_is_routing>
      <Routes>
        <Route path="/" view=move || view! { <Layout is_routing/> } ssr=SsrMode::PartiallyBlocked>
          <Route path="" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="home" view=HomeActivity ssr=SsrMode::Async/>

          <Route path="communities" view=CommunitiesActivity/>
          <Route path="create_post" view=CommunitiesActivity/>
          <Route path="create_community" view=CommunitiesActivity/>

          <Route path="search" view=CommunitiesActivity/>
          <Route path="login" view=LoginActivity ssr=SsrMode::Async/>
          <Route path="signup" view=CommunitiesActivity/>

          <Route path="inbox" view=CommunitiesActivity/>
          <Route path="u/:id" view=CommunitiesActivity/>
          <Route path="settings" view=CommunitiesActivity/>
          <Route path="logout" view=CommunitiesActivity/>

          <Route path="modlog" view=CommunitiesActivity/>
          <Route path="instances" view=CommunitiesActivity/>

          <Route path="post/:id" view=PostActivity ssr=SsrMode::Async/>
        </Route>
      </Routes>
    </Router>
  }
}

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            leptos::mount_to_body(App);
        }
    }
}
