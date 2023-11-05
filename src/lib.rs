#![allow(warnings)]

mod config;
mod errors;
mod host;
mod layout;
mod lemmy_client;
mod queries;
#[cfg(feature = "ssr")]
pub mod server;
mod ui;

use crate::{
  i18n::*,
  layout::Layout,
  ui::components::{
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  }, queries::site_state_query::use_site_state,
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

  // let authenticated_user = create_rw_signal::<Option<Person>>(None);
  // provide_context(authenticated_user);

  // let QueryResult { data, refetch, .. } = use_site_state();

  // let authenticated_user = Signal::derive(move || {
  //   data().map_or_else(
  //     || None,
  //     |res| res.ok()?.my_user.map(|user| user.local_user_view.person),
  //   )
  // });

  // provide_context(authenticated_user);
  // provide_context(refetch);

  let (is_routing, set_is_routing) = create_signal(false);

  view! {
    <Router set_is_routing>
      <Routes>
        <Route path="/" view=move || view! { <Layout is_routing/> } ssr=SsrMode::PartiallyBlocked>
          <Route path="" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="home" view=PostActivity/>

          <Route path="communities" view=PostActivity/>
          <Route path="create_post" view=PostActivity/>
          <Route path="create_community" view=PostActivity/>

          <Route path="search" view=PostActivity/>
          <Route path="login" view=LoginActivity/>
          <Route path="signup" view=PostActivity/>

          <Route path="inbox" view=PostActivity/>
          <Route path="u/:id" view=PostActivity/>
          <Route path="settings" view=PostActivity/>
          <Route path="logout" view=PostActivity/>

          <Route path="modlog" view=PostActivity/>
          <Route path="instances" view=PostActivity/>

          <Route path="post/:id" view=PostActivity/>
        </Route>
      </Routes>
    </Router>
  }
}

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            leptos::mount_to_body(App);
        }
    }
}
