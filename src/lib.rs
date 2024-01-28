// #![allow(warnings)]

mod config;
mod cookie;
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
  errors::LemmyAppError,
  i18n::*,
  layout::Layout,
  lemmy_client::*,
  ui::components::{
    communities::communities_activity::CommunitiesActivity,
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  },
};
use cfg_if::cfg_if;
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();

  let error = create_rw_signal::<Option<LemmyAppError>>(None);
  provide_context(error);
  let user = create_rw_signal::<Option<bool>>(None);
  provide_context(user);
  let site_data = create_rw_signal::<Option<Result<GetSiteResponse, LemmyAppError>>>(None);
  provide_context(site_data);
  let ui_theme = create_rw_signal::<Option<String>>(None);
  provide_context(ui_theme);

  let ssr_site = create_resource(
    move || (user.get()),
    move |_user| async move { LemmyClient.get_site().await },
  );

  let site_signal = create_rw_signal::<Option<GetSiteResponse>>(None);

  view! {
    <Transition fallback=|| {}>
      // the only way i can find to force a signal to initialize in SSR mode
      {move || {
          ssr_site
              .get()
              .map(|m| match m {
                  Ok(o) => {
                      site_signal.set(Some(o));
                      view! {}
                  }
                  _ => {
                      view! {}
                  }
              })
      }}

    </Transition>
    <Router>
      <Routes>
        <Route path="/" view=move || view! { <Layout site_signal/> }>
          <Route path="" view=move || view! { <HomeActivity site_signal/> } ssr=SsrMode::Async/>
          <Route path="communities" view=CommunitiesActivity/>
          <Route path="create_post" view=CommunitiesActivity/>
          <Route path="create_community" view=CommunitiesActivity/>
          <Route path="c/:id" view=CommunitiesActivity/>

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
