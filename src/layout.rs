use crate::{ui::components::common::nav::{BottomNav, TopNav}, cookie::{get_cookie}};
use leptos::*;
use leptos_meta::*;
use leptos_router::{Outlet, RoutingProgress};

#[component]
pub fn Layout(is_routing: ReadSignal<bool>) -> impl IntoView {
  let ui_theme = expect_context::<RwSignal<Option<String>>>();

  // // let theme = None::<String>;
  // // let theme = Some("retro".to_string());
  // // {
  // spawn_local(

  let theme = create_resource(
    move || (),
    move |()| async move {
      // #[cfg(feature = "ssr")] {
      // use actix_session::Session;
      // use leptos_actix::extract;
      let cookie_res = get_cookie("theme").await;
      //  extract(|session: Session| async move {
      //   // #[cfg(feature = "ssr")]
      //   // {
      //   //   use cookie::get_cookie;
      //   //   logging::log!("e {:#?}", get_cookie("theme").await);
      //   //   logging::log!("f {:#?}", session.entries());
      //   // }

      //   let r = get_cookie("theme").await;

      //   logging::log!("r {:#?}", r);

      //   match r {
      //     Ok(o) => {
      //       o
      //     }
      //     _ => {
      //       None
      //     },
      //   }
      // }).await;
      match cookie_res {
        Ok(Some(o)) => {
          // logging::log!("none {:#?}", o);
          o
        }
        _ => {
          // logging::log!("none");
          "retro".to_string()
        },
      }
      // }

      // #[cfg(not(feature = "ssr"))]
      // if let Some(r) = wasm_cookies::get("theme") {
      //   match r {
      //     Ok(o) => o,
      //     _ => "light".to_string(),
      //   }
      // } else {
      //   "light".to_string()
      // }

    }
  //   // })
  //   // }.get();
  //   );
  );


  // #[cfg(not(feature = "ssr"))]
  // // let theme = None::<String>;
  // // let theme = {
  //   if let Some(r) = wasm_cookies::get("theme") {
  //     match r {
  //       Ok(o) => ui_theme.set(o),
  //       _ => ui_theme.set("retro".to_string()),
  //     }
  //   } else {
  //     ui_theme.set("retro".to_string());
  //   }
  // // };

  // // if let Some(t) = theme {
  ui_theme.set(theme.get());
  // // }

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
        <div class="flex flex-col h-screen" data-theme=move || ui_theme.get().unwrap_or(theme.get().unwrap_or("".to_string()))>
          <TopNav/>
          <Outlet/>
          <BottomNav/>
        </div>
      </Transition>
    </ErrorBoundary>
  }
}
