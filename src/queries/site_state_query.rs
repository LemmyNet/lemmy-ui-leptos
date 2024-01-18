use crate::{errors::LemmyAppError, cookie::get_cookie};
use crate::lemmy_client::*;
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_query::{use_query, QueryOptions, QueryResult, RefetchFn, ResourceOption};

#[server(GetSiteResource, "/serverfn", "GetJson")]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  // use actix_session::Session;
  // use leptos_actix::extract;

  let jwt = get_cookie("jwt").await?;
  // extract(|session: Session| async move { session.get::<String>("jwt") }).await??;

  // logging::log!("SITE JWT {:#?}", jwt);

  let result = Fetch.get_site(/* jwt */).await?;

  // logging::log!("coop {:#?}", result);

  Ok(result)
}

pub fn use_site_state() -> QueryResult<Result<GetSiteResponse, LemmyAppError>, impl RefetchFn> {
  use_query(
    || (),
    |_| async move {
      // use crate::lemmy_client::*;

      // let jwt = get_cookie("jwt").await?;

      // #[cfg(feature = "ssr")]
      // let jwt = None //{
      //   // use actix_session::Session;
      //   // use leptos_actix::extract;
      //   // extract(|session: Session| async move { session.get::<String>("jwt") }).await??
      // // }
      // ;

      // #[cfg(not(feature = "ssr"))]
      // let jwt = {
      //   use wasm_cookies::get;
      //   get("jwt").and_then(Result::ok)
      // };

      Fetch.get_site(/* jwt */).await
      // get_site().await
    },
    QueryOptions {
      resource_option: ResourceOption::Blocking,
      ..QueryOptions::default()
    },
  )
}
