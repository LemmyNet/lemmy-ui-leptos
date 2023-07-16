use crate::ui::components::{common::nav::Nav, home::home_activity::HomeActivity};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod api;
mod ui;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  provide_meta_context(cx);
  let (is_routing, set_is_routing) = create_signal(cx, false);

  view! { cx,
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
    <Meta name="description" content="Lemmy-UI-Leptos."/>
    // adding `set_is_routing` causes the router to wait for async data to load on new pages
    <Router set_is_routing>
      // shows a progress bar while async data are loading
      <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
      <Nav/>
      <main>
        <Routes>
          <Route path="home" view=HomeActivity/>
          <Route path="" view=HomeActivity/>
        // <Route path="stories/:id" view=Story/>
        // <Route path=":stories?" view=Stories/>
        </Routes>
      </main>
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
            leptos::mount_to_body(move |cx| {
                view! { cx, <App/> }
            });
        }
    }
}
