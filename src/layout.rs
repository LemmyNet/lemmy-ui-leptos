use crate::{ui::components::common::nav::{BottomNav, TopNav}, cookie::{get_cookie}, lemmy_client::*, errors::LemmyAppError};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::{Outlet, RoutingProgress};

#[component]
pub fn Layout(is_routing: ReadSignal<bool>) -> impl IntoView {
  let ui_theme = expect_context::<RwSignal<Option<String>>>();
  let theme = create_resource(move || (), move |()| async move {
    let r = get_cookie("theme").await;
    match r {
      Ok(Some(o)) => {
        o
      }
      _ => {
        "retro".to_string()
      },
    }
  });
  ui_theme.set(theme.get());

  view! {
    <ErrorBoundary fallback=|_| view! { Error! }>
      // <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
      <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
      <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
      <Meta name="description" content="Lemmy-UI-Leptos."/>
      <Meta name="viewport" content="viewport-fit=cover"/>
      // debug where there is no visible console (mobile/live/desktop)
      // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
      // <Script>eruda.init();</Script>
      <Title text="Brand ite info"/>
      <Transition fallback=|| { view! { <div> "Loading..."  </div> } }>
        <div class="flex flex-col h-screen" data-theme=move || ui_theme.get().unwrap_or(theme.get().unwrap_or("retro".to_string()))>
          <TopNav/>
          <Outlet/>
          <BottomNav/>
        </div>
      </Transition>
    </ErrorBoundary>
  }
}
