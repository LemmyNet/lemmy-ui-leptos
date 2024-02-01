use crate::{
  cookie::get_cookie,
  errors::LemmyAppError,
  ui::components::common::nav::{BottomNav, TopNav},
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;

#[component]
pub fn Layout(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>, /* , ui_theme: RwSignal<Option<String>> *//* Option<GetSiteResponse> */
) -> impl IntoView {
  let title = move || match site_signal.get() {
    Some(Ok(o)) => {
      if let Some(s) = o.site_view.site.description {
        format!("{} - {}", o.site_view.site.name, s)
      } else {
        o.site_view.site.name
      }
    }
    _ => "Lemmy".to_string(),
  };

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

  view! {
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
    <Meta name="description" content="Lemmy-UI-Leptos."/>
    <Meta name="viewport" content="viewport-fit=cover"/>
    // debug console where there is no dev tools (mobile/desktop)
    // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
    // <Script>eruda.init();</Script>
    <Transition fallback=|| {}>
      {move || {
          theme
              .get()
              .map(|m| {
                  ui_theme.set(Some(m));
              })
      }}
      <div class="flex flex-col h-screen" data-theme=move || ui_theme.get()>
        <Title text=title/>
        <TopNav site_signal_1=site_signal/>
        <Outlet/>
        <BottomNav site_signal_1=site_signal/>
      </div>
    </Transition>
  }
}
