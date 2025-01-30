#![allow(clippy::empty_docs)]

mod constants;
mod contexts;
#[cfg(feature = "ssr")]
pub mod cookie_middleware;
pub mod host;
mod serverfns;
mod ui;
mod utils;
use crate::{
  contexts::{
    site_resource_context::provide_site_resource_context,
    theme_resource_context::provide_theme_resource_context,
  },
  ui::components::{
    communities::communities_page::CommunitiesPage, home::home_page::HomePage,
    layouts::base_layout::BaseLayout, login::login_page::LoginPage, post::post_page::PostPage,
  },
};
use contexts::site_resource_context::SiteResource;
use fluent_templates::static_loader;
use leptos::{html::Dialog, prelude::*};
use leptos_fluent::leptos_fluent;
use leptos_meta::*;
use leptos_router::{components::*, *};
use ui::components::modals::ReportModal;
use utils::types::{ReportModalData, ReportModalNode};

static_loader! {
  static TRANSLATIONS = {
      locales: "./locales",
      fallback_language: "en",
  };
}

#[component]
fn I18n(children: Children) -> impl IntoView {
  leptos_fluent! {
    children: children(),
    translations: [TRANSLATIONS],
    locales: "./locales",
    check_translations: "./src/**/*.rs",
    sync_html_tag_lang: true,
    initial_language_from_accept_language_header: true,
    cookie_attrs: "SameSite=Strict; Secure;",
    initial_language_from_cookie: true,
    set_language_to_cookie: true,
    initial_language_from_navigator: true
  }
}

#[component]
fn AppRoutes() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = move || {
    site_resource.read().as_ref().map(|response| {
      response
        .as_ref()
        .ok()
        .is_some_and(|response| response.my_user.is_some())
    })
  };

  let is_routing = RwSignal::new(false);

  let (report_modal_data, set_report_modal_data) =
    RwSignal::new(ReportModalData::default()).split();
  let report_modal = ReportModalNode(NodeRef::<Dialog>::new());
  provide_context(set_report_modal_data);
  provide_context(report_modal);

  view! {
    <Router set_is_routing=is_routing>
      <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250) />

      <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css" />
      <Title text="Brand from env" />

      <Routes fallback=NotFound>
        <ParentRoute path=path!("") view=BaseLayout ssr=SsrMode::Async>
          <Route path=path!("") view=HomePage />

          <Route path=path!("create_post") view=CommunitiesPage />
          <Route path=path!("post/:id") view=PostPage />

          <Route path=path!("search") view=CommunitiesPage />
          <Route path=path!("communities") view=CommunitiesPage />
          <Route path=path!("create_community") view=CommunitiesPage />
          <Route path=path!("c/:id") view=CommunitiesPage />
          <ProtectedRoute
            path=path!("login")
            view=LoginPage
            redirect_path=move || ""
            condition=user_is_logged_in
          />

          <ProtectedRoute
            path=path!("signup")
            view=CommunitiesPage
            redirect_path=move || ""
            condition=user_is_logged_in
          />

          <Route path=path!("inbox") view=CommunitiesPage />
          <Route path=path!("settings") view=CommunitiesPage />
          <Route path=path!("u/:id") view=CommunitiesPage />
          <Route path=path!("saved") view=CommunitiesPage />

          <Route path=path!("modlog") view=CommunitiesPage />
          <Route path=path!("instances") view=CommunitiesPage />
          <Route path=path!("legal") view=CommunitiesPage />
        </ParentRoute>
      </Routes>

      <ReportModal dialog_ref=report_modal.0 modal_data=report_modal_data />
    </Router>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_site_resource_context();
  provide_theme_resource_context();

  view! {
    <I18n>
      <AppRoutes />
    </I18n>
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
  leptos::mount::hydrate_body(App);
}
