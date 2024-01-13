use crate::ui::components::common::nav::{BottomNav, TopNav};
use leptos::*;
use leptos_meta::*;
use leptos_router::{Outlet, RoutingProgress};

#[component]
pub fn Layout(is_routing: ReadSignal<bool>) -> impl IntoView {
  let ui_theme = expect_context::<RwSignal<String>>();

  #[cfg(feature = "ssr")]
  let theme = None::<String>; 
  // let theme = {

  //   create_resource(|| (), |_| async move {

  //   use actix_session::Session;
  //   use leptos_actix::{extract, redirect};

  //   let cookie_res =
  //   extract(|session: Session| async move {
  //     session.get::<String>("theme")
  //   })
  //   .await;

  //   match cookie_res {
  //     Ok(Ok(Some(o))) => {
  //       o
  //       // redirect("/");
  //       // Ok(())
  //     }
  //     _ => {
  //       "retro".to_string()
  //     },
  //   }

  //   })
  // }.get();

  #[cfg(not(feature = "ssr"))]
  // let theme = None::<String>; 
  let theme = {
    if let Some(r) = wasm_cookies::get("theme") {
      match r {
        Ok(o) => {
          Some(o)
        }
        _ => {
          None
        },
      }
    } else {
      None
    }
  };

  if let Some(t) = theme {
    ui_theme.set(t);
  }

  view! {
    <ErrorBoundary fallback=|_| view! { Error! }>
      <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
      <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
      <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
      <Meta name="description" content="Lemmy-UI-Leptos."/>
      <Meta name="viewport" content="viewport-fit=cover"/>
      // debug where there is no visible console (mobile/live/desktop)
      // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
      // <Script>eruda.init();</Script>
      <Title text="Brand from env"/>

      <div class="flex flex-col h-screen" data-theme=ui_theme>
        <TopNav/>
        <Outlet/>
        <BottomNav/>
      </div>
    </ErrorBoundary>
  }
}
