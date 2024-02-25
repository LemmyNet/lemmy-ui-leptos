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
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
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
    <Meta name="description" content=title/>
    // debug console when there is no dev tools (mobile/desktop)
    // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
    // <Script>eruda.init();</Script>
    <Title text=title/>
    <Transition fallback=|| {}>
      {move || {
          theme
              .get()
              .map(|m| {
                  ui_theme.set(Some(m));
                  view! {
                    <div class="flex flex-col min-h-screen" data-theme=move || ui_theme.get()>
                      <TopNav site_signal/>
                      <div class="w-full flex flex-col flex-grow">
                        <div class="sm:container sm:mx-auto">
                          <div class="w-full flex flex-col flex-grow p-6">
                            <Outlet/>
                          </div>
                        </div>
                      </div>
                      <BottomNav site_signal/>
                    </div>
                  }
              })
      }}

    </Transition>
  }
}
