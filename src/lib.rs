use crate::{
  i18n::*,
  ui::components::{
    common::nav::{BottomNav, TopNav},
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
    site_state_provider::SiteStateProvider,
  },
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod config;
mod errors;
mod host;
mod lemmy_client;
pub mod server;
mod ui;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();

  let authenticated = create_rw_signal::<bool>(false);
  provide_context(authenticated);

  let (is_routing, set_is_routing) = create_signal(false);

  view! {
    <SiteStateProvider>
      <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
      <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
      <Meta name="description" content="Lemmy-UI-Leptos."/>
      <Meta name="viewport" content="viewport-fit=cover"/>
      <Script src="//cdn.jsdelivr.net/npm/eruda"/>
      <Script>eruda.init();</Script>
      <Title text="Brand from env"/>

      // adding `set_is_routing` causes the router to wait for async data to load on new pages
      <Router set_is_routing>
        <div class="flex flex-col h-screen">
          <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
          <TopNav/>
          <Routes>
            <Route path="" view=HomeActivity/>
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
          </Routes>
          <BottomNav/>
        </div>
      </Router>
    </SiteStateProvider>
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
