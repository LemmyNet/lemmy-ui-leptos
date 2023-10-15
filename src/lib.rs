use crate::{
  api::get_cookie_wrapper,
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
mod api;
pub mod api_service;
mod config;
mod errors;
mod host;
mod lemmy_client;
mod ui;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();

  let ui_theme = create_rw_signal::<String>(String::from("retro"));
  provide_context(ui_theme);

  let authenticated = create_rw_signal::<bool>(false);
  provide_context(authenticated);

  let auth_resource = create_resource(
    || (),
    move |()| async move {
      match get_cookie_wrapper("jwt").await {
        Ok(Some(_jwt)) => {
          authenticated.set(true);
          leptos::logging::log!("ROOT jwt");
          true
        }
        Ok(None) => {
          authenticated.set(false);
          leptos::logging::log!("NONE jwt");
          false
        }
        Err(_e) => {
          authenticated.set(false);
          false
        }
      }
    },
  );

  // #[cfg(feature = "ssr")]
  // spawn_local(async move {
  //   match get_cookie_wrapper("jwt").await {
  //     Ok(Some(_jwt)) => {
  //       authenticated.set(true);
  //       leptos::logging::log!("TONG jwt");
  //       // true
  //     }
  //     Ok(None) => {
  //       authenticated.set(false);
  //       leptos::logging::log!("TONG NONE jwt");
  //       // false
  //     }
  //     Err(_e) => {
  //       authenticated.set(false);
  //       // false
  //     }
  //   }
  // });

  let (is_routing, set_is_routing) = create_signal(false);

  view! {
    <Suspense>
      <div>{ move || auth_resource.get() }</div>
      <div>{ move || authenticated.get() }</div>
    </Suspense>
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
    <Link rel="shortcut icon" type_="image/svg" href="/favicon.svg"/>
    <Meta name="description" content="Lemmy-UI-Leptos."/>
    <Meta name="viewport" content="viewport-fit=cover"/>
    // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
    // <Script>eruda.init();</Script>
    <Title text="Brand from env"/>

    // adding `set_is_routing` causes the router to wait for async data to load on new pages
    <Router set_is_routing>
      <div class="flex flex-col h-screen" data-theme=move || ui_theme()>
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
