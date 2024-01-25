use crate::{
  cookie::get_cookie,
  errors::LemmyAppError,
  lemmy_client::*,
  ui::components::common::nav::{BottomNav, TopNav},
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::{Outlet, RoutingProgress};

#[component]
pub fn Layout(/* is_routing: ReadSignal<bool> */) -> impl IntoView {
  // let site_data = expect_context::<RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>>();
  let title = create_resource(
    move || (),
    move |()| async move { 
      match LemmyClient.get_site().await { 
        Ok(o) => {
          if let Some(s) = o.site_view.site.description {
            format!("{} - {}", o.site_view.site.name, s)
          } else {
            o.site_view.site.name
          }
        }
        _ => { "Lemmy".to_string() }
      }
    },
  );
  // site_data.set(data.get());

  // let site_data = expect_context::<RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>>();

  // let title = Signal::derive(move || site_data.get().or(data.get()).map(|m| match m { 
  //   Ok(o) => {
  //     if let Some(s) = o.site_view.site.description {
  //       format!("{} - {}", o.site_view.site.name, s)
  //     } else {
  //       o.site_view.site.name
  //     }
  //   }
  //   _ => { "Lemmy".to_string() }
  // }).unwrap_or("Lemmy".to_string()));

  let ui_theme = expect_context::<RwSignal<Option<String>>>();
  let theme = create_resource(
    move || (),
    move |()| async move {
      let r = get_cookie("theme").await;
      match r {
        Ok(Some(o)) => o,
        _ => "retro".to_string(),
      }
    },
  );
  // ui_theme.set(theme.get());

  view! {
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
    <Meta name="description" content="Lemmy-UI-Leptos."/>
    <Meta name="viewport" content="viewport-fit=cover"/>
    // debug where there is no visible console (mobile/live/desktop)
    // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
    // <Script>eruda.init();</Script>
    // <Transition fallback=|| { view! { <Title /> } }>
    //   <Title text=move || title.get().unwrap_or("Lemmy".to_string())/>
    // </Transition>
    <Transition fallback=|| {
      view! { <div>"Loading..."</div> }
    }>
      <div
        class="flex flex-col h-screen"
        data-theme=move || ui_theme.get().unwrap_or(theme.get().unwrap_or("retro".to_string()))
      >
        <Title text=move || title.get().unwrap_or("Lemmy".to_string())/>
        <TopNav/>
        <Outlet/>
        <BottomNav/>
      </div>
    </Transition>
  }
}
