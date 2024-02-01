// #![allow(warnings)]

mod config;
mod cookie;
mod errors;
mod host;
mod layout;
mod lemmy_client;
mod lemmy_errors;
// mod queries;
#[cfg(feature = "ssr")]
pub mod server;
mod ui;

use crate::{
  errors::LemmyAppError,
  // i18n::*,
  layout::Layout,
  lemmy_client::*,
  ui::components::{
    communities::communities_activity::CommunitiesActivity,
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  },
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  // crashes web server currently (v0.2)
  // provide_i18n_context();

  let error = create_rw_signal::<Option<LemmyAppError>>(None);
  provide_context(error);
  let user = create_rw_signal::<Option<bool>>(None);
  provide_context(user);
  let ui_theme = create_rw_signal::<Option<String>>(None);
  provide_context(ui_theme);

  let ssr_site = create_resource(
    move || (user.get()),
    move |_user| async move {
      let result = LemmyClient.get_site().await;

      match result {
        Ok(o) => Ok(o),
        Err(e) => {
          error.set(Some(e.clone()));
          Err(e)
        }
      }
    },
  );

  let site_signal = create_rw_signal::<Option<Result<GetSiteResponse, LemmyAppError>>>(None);

  view! {
    <Transition fallback=|| {}>
      // the only way i can find to force a signal to initialize in SSR mode
      {move || {
          ssr_site
              .get()
              .map(|m| {
                  site_signal.set(Some(m));
              });
      }}

    </Transition>

    <Router>
      <Routes>
        <Route path="/" view=move || view! { <Layout site_signal/> } ssr=SsrMode::Async>
          <Route path="/*any" view=NotFound/>

          <Route path="" view=move || view! { <HomeActivity site_signal_1=site_signal/> }/>

          <Route path="create_post" view=CommunitiesActivity/>
          <Route path="post/:id" view=PostActivity/>

          <Route path="search" view=CommunitiesActivity/>
          <Route path="communities" view=CommunitiesActivity/>
          <Route path="create_community" view=CommunitiesActivity/>
          <Route path="c/:id" view=CommunitiesActivity/>

          <Route path="login" view=LoginActivity/>
          <Route path="logout" view=CommunitiesActivity/>
          <Route path="signup" view=CommunitiesActivity/>

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
fn NotFound() -> impl IntoView {
  // set an HTTP status code 404
  // this is feature gated because it can only be done during
  // initial server-side rendering
  // if you navigate to the 404 page subsequently, the status
  // code will not be set because there is not a new HTTP request
  // to the server
  #[cfg(feature = "ssr")]
  {
    // this can be done inline because it's synchronous
    // if it were async, we'd use a server function
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! { <h1>"Not Found"</h1> }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  use leptos::*;
  console_error_panic_hook::set_once();
  mount_to_body(App);
}
