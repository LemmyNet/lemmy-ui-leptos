use crate::{
  i18n::*,
  layout::Layout,
  ui::components::{
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  },
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_query::provide_query_client;
use leptos_router::*;
mod config;
mod errors;
mod host;
mod layout;
mod lemmy_client;
mod queries;
pub mod server;
mod ui;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();
  provide_query_client();

  let (is_routing, set_is_routing) = create_signal(false);

  view! {
    <Router set_is_routing>
      <Routes>
        <Route path="/" view=move || view! { <Layout is_routing/> } ssr=SsrMode::PartiallyBlocked>
          <Route path="/" view=HomeActivity/>
          <Route path="home" view=HomeActivity/>

          <Route path="communities" view=HomeActivity/>
          <Route path="create_post" view=HomeActivity/>
          <Route path="create_community" view=HomeActivity/>

          <Route path="search" view=HomeActivity/>
          <Route path="login" view=LoginActivity/>
          <Route path="signup" view=LoginActivity/>

          <Route path="inbox" view=HomeActivity/>
          <Route path="u/:id" view=HomeActivity/>
          <Route path="settings" view=HomeActivity/>
          <Route path="logout" view=HomeActivity/>

          <Route path="modlog" view=HomeActivity/>
          <Route path="instances" view=HomeActivity/>

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
