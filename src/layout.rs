use crate::ui::components::common::nav::{BottomNav, TopNav};
use leptos::*;
use leptos_meta::*;
use leptos_router::{Outlet, RoutingProgress};

#[component]
pub fn Layout(is_routing: ReadSignal<bool>) -> impl IntoView {
  let ui_theme = expect_context::<RwSignal<String>>();

  view! {
    // <Suspense>
    //   <ErrorBoundary fallback=|_| view! { Error! }>
        <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
        <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
        <Meta name="description" content="Lemmy-UI-Leptos."/>
        <Meta name="viewport" content="viewport-fit=cover"/>
        // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
        // <Script>eruda.init();</Script>
        <Title text="Brand from env"/>

        <div class="flex flex-col h-screen" data-theme=move || ui_theme()>
          <TopNav/>
          <Outlet/>
          <BottomNav/>
        </div>
    //   </ErrorBoundary>
    // </Suspense>
  }
}
