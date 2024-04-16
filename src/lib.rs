// useful in development to only have errors in compiler output
// #![allow(warnings)]

mod config;
mod cookie;
mod errors;
pub mod host;
mod layout;
mod lemmy_client;
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
  let ui_theme = create_rw_signal::<Option<String>>(None);
  provide_context(ui_theme);

  let site_signal = create_rw_signal::<Option<Result<GetSiteResponse, LemmyAppError>>>(None);

  let ssr_site = create_resource(
    move || (user.get()),
    move |user| async move {
      // fix login cache issue
      let result = if user == Some(false) {
        if let Some(Ok(mut s)) = site_signal.get() {
          s.my_user = None;
          Ok(s)
        } else {
          LemmyClient.get_site().await
        }
      } else {
        LemmyClient.get_site().await
      };

      // let result = LemmyClient.get_site().await;

      match result {
        Ok(o) => Ok(o),
        Err(e) => {
          error.set(Some(e.clone()));
          Err(e)
        }
      }
    },
  );

  view! {
    <Transition fallback=|| {}>
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

          <Route path="" view=move || view! { <HomeActivity site_signal/> }/>

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
