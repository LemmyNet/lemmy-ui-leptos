use crate::{
  cookie::get_cookie,
  errors::LemmyAppError,
  ui::components::common::nav::{BottomNav, TopNav},
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;
use web_sys::{js_sys::Function, wasm_bindgen::closure::Closure, Event};

#[component]
pub fn Layout(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>
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


  // let cb = Closure::wrap(Box::new(|e: Event| {
  //   // let input = e
  //   //     .current_target()
  //   //     .unwrap()
  //   //     .dyn_into::<web_sys::HtmlTextAreaElement>()
  //   //     .unwrap();

  //   logging::log!("{:?}", e);
  // }) as Box<dyn Function(_)>);


  // let r = window().add_event_listener_with_callback("scroll", &cb.as_ref());

  // cb.forget();


  // window().set_onscroll(Some(Function::new_no_args(body))) onscroll()  .add_event_listener_with_callback("scroll", &on_scroll);

  view! {
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
    <Meta name="description" content=title/>
    // <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    // <Meta name="viewport" content="viewport-fit=cover"/>
    // debug console where there is no dev tools (mobile/desktop)
    // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
    // <Script>eruda.init();</Script>
    // <Window />
    // <Body on:scroll=on_scroll /* class="min-h-screen h-full" *//>
    <Title text=title/>
    // <Body class="min-h-screen h-full"/>
    <Transition fallback=|| {}>
      {move || {
          theme
              .get()
              .map(|m| {
                  ui_theme.set(Some(m));
                  view! {
                    <div class="flex flex-col min-h-screen"  /* class="flex flex-col flex-row flex-grow justify-between min-h-screen"  */data-theme=move || ui_theme.get()>
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
      // <div class="flex flex-col min-h-screen"  /* class="flex flex-col flex-row flex-grow justify-between min-h-screen"  */data-theme=move || ui_theme.get()>
      //   <TopNav site_signal/>
      //   <div class="w-full flex flex-col flex-grow">
      //     <div class="sm:container sm:mx-auto">
      //       <div class="w-full flex flex-col flex-grow p-6">
      //         <Outlet/>
      //       </div>
      //     </div>
      //   </div>
      //   <BottomNav site_signal/>
      // </div>
    </Transition>
  }
}
