use crate::ui::components::{
  common::nav::{BottomNav, TopNav},
  home::home_activity::HomeActivity,
  login::login_activity::LoginActivity,
  post::post_activity::PostActivity,
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod api;
mod errors;
mod ui;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  let (is_routing, set_is_routing) = create_signal(false);

  view! {
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
    <Meta name="description" content="Lemmy-UI-Leptos."/>
    <Meta name="viewport" content="viewport-fit=cover"/>
    <Script src="//cdn.jsdelivr.net/npm/eruda"/>
    <Script>eruda.init();</Script>

    // adding `set_is_routing` causes the router to wait for async data to load on new pages
    <Router set_is_routing>
      // shows a progress bar while async data are loading
      <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
      <TopNav/>
        <Routes>
          <Route path="home" view=HomeActivity/>
          <Route path="" view=HomeActivity/>
          <Route path="login" view=LoginActivity/>
          <Route path="post/:id" view=PostActivity/>
        // <Route path="stories/:id" view=Story/>
        // <Route path=":stories?" view=Stories/>
        </Routes>
      <BottomNav/>
    </Router>
  }
}

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            leptos::mount_to_body(move || {
                view! { <App/> }
            });
        }
    }
}
