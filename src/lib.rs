use crate::{
  i18n::*,
  ui::components::{
    common::nav::{BottomNav, TopNav},
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
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
          <Route path="" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="home" view=HomeActivity ssr=SsrMode::Async/>

          <Route path="communities" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="create_post" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="create_community" view=HomeActivity ssr=SsrMode::Async/>

          <Route path="search" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="login" view=LoginActivity ssr=SsrMode::Async/>
          <Route path="signup" view=LoginActivity ssr=SsrMode::Async/>

          <Route path="inbox" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="u/:id" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="settings" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="logout" view=HomeActivity ssr=SsrMode::Async/>

          <Route path="modlog" view=HomeActivity ssr=SsrMode::Async/>
          <Route path="instances" view=HomeActivity ssr=SsrMode::Async/>

          <Route path="post/:id" view=PostActivity ssr=SsrMode::Async/>
        </Routes>
        <BottomNav/>
      </div>
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
