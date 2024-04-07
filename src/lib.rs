// useful in development to only have errors in compiler output
#![allow(warnings)]

mod config;
mod cookie;
mod errors;
pub mod host;
mod layout;
mod lemmy_client;
mod ui;

use crate::{
  errors::LemmyAppError,
  i18n::*,
  layout::Layout,
  lemmy_client::*,
  ui::components::{
    communities::communities_activity::CommunitiesActivity,
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  },
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();

  let error = create_rw_signal::<Option<LemmyAppError>>(None);
  provide_context(error);
  let user = create_rw_signal::<Option<bool>>(None);
  provide_context(user);
  let ui_theme = create_rw_signal::<Option<String>>(None);
  provide_context(ui_theme);

  let site_signal = create_rw_signal::<Option<Result<GetSiteResponse, LemmyAppError>>>(None);

  let ssr_site = create_resource(
    move || (user.get()),
    move |_user| async move {
      let result = if _user == Some(true) {
        LemmyClient.get_site().await
      } else {
        if let Some(Ok(mut s)) = site_signal.get() {
          s.my_user = None;
          Ok(s)
        } else {
          LemmyClient.get_site().await
        }
      };

      match result {
        Ok(o) => Ok(o),
        Err(e) => {
          error.set(Some(e.clone()));
          Err(e)
        }
      }
    },
  );


  view! {
    <Transition fallback=|| {}>
      {move || {
          ssr_site
              .get()
              .map(|m| {
                  site_signal.set(Some(m));
              });
      }}

    </Transition>
    <Router>
      <Routes>
        <Route path="/" view=move || view! { <Layout site_signal/> } ssr=SsrMode::Async>
          <Route path="/*any" view=NotFound/>

          <Route path="" view=move || view! { <HomeActivity site_signal/> }/>

          <Route path="create_post" view=CommunitiesActivity/>
          <Route path="post/:id" view=PostActivity/>

          <Route path="search" view=CommunitiesActivity/>
          <Route path="communities" view=CommunitiesActivity/>
          <Route path="create_community" view=CommunitiesActivity/>
          <Route path="c/:id" view=CommunitiesActivity/>

          <Route path="login" view=LoginActivity/>
          <Route path="logout" view=CommunitiesActivity/>
          <Route path="signup" view=CommunitiesActivity/>

          <Route path="inbox" view=CommunitiesActivity/>
          <Route path="settings" view=CommunitiesActivity/>
          <Route path="u/:id" view=CommunitiesActivity/>

          <Route path="modlog" view=CommunitiesActivity/>
          <Route path="instances" view=CommunitiesActivity/>
        </Route>
      </Routes>
    </Router>
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
  mount_to_body(App);
}

// use anyhow::Result;

// use headless_chrome::{Browser, LaunchOptions};

// use std::error::Error;

// use headless_chrome::Browser;
// use headless_chrome::protocol::cdp::Page;

// fn browse_wikipedia() -> Result<(), Box<dyn Error>> {
//     Ok(())
// }

// fn query(input: &str) -> Result<()> {
//   let browser = Browser::new(
//       LaunchOptions::default_builder()
//           .build()
//           .expect("Could not find chrome-executable"),
//   )?;
//   let tab = browser.new_tab()?;
//   tab.navigate_to("https://en.wikipedia.org")?
//       .wait_for_element("input#searchInput")?
//       .click()?;
//   tab.type_str(input)?.press_key("Enter")?;
//   match tab.wait_for_element("div.shortdescription") {
//       Err(e) => eprintln!("Query failed: {e:?}"),
//       Ok(e) => match e.get_description()?.find(|n| n.node_name == "#text") {
//           Some(n) => println!("Result for `{}`: {}", &input, n.node_value),
//           None => eprintln!("No shortdescription-node found on page"),
//       },
//   }
//   Ok(())
// }

// fn main() -> Result<()> {
//   let input = "Elvis Aaron Presley";
//   query(input)
// }

// #[cfg(test)]
// mod tests {
//     use std::error::Error;

//     use headless_chrome::Browser;
//     use headless_chrome::protocol::cdp::Page;
  
//     #[test]
//     fn browse_wikipedia()-> Result<(), Box<dyn Error>> {

//       let browser = Browser::default()?;

//       let tab = browser.new_tab()?;
  
//       // Navigate to wikipedia
//       tab.navigate_to("https://www.wikipedia.org")?;
  
//       // Wait for network/javascript/dom to make the search-box available
//       // and click it.
//       tab.wait_for_element("input#searchInput")?.click()?;

//       tab.
  
//       // Type in a query and press `Enter`
//       tab.type_str("WebKit")?.press_key("Enter")?;
  
//       // We should end up on the WebKit-page once navigated
//       let elem = tab.wait_for_element("#firstHeading")?;
//       assert!(tab.get_url().ends_with("WebKit"));
  
//       // Take a screenshot of the entire browser window
//       let _jpeg_data = tab.capture_screenshot(
//           Page::CaptureScreenshotFormatOption::Jpeg,
//           None,
//           None,
//           true)?;
  
//       // Take a screenshot of just the WebKit-Infobox
//       let _png_data = tab
//           .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
//           .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
  
//       // Run JavaScript in the page
//       let remote_object = elem.call_js_fn(r#"
//           function getIdTwice () {
//               // `this` is always the element that you called `call_js_fn` on
//               const id = this.id;
//               return id + id;
//           }
//       "#, vec![], false)?;
//       match remote_object.value {
//           Some(returned_string) => {
//               dbg!(&returned_string);
//               assert_eq!(returned_string, "firstHeadingfirstHeading".to_string());
//           }
//           _ => {
//               assert!(remote_object.value.is_some());
//           }
//       };

//       Ok(())

//     }
// }