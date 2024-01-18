// use crate::{errors::LemmyAppError, cookie::get_cookie};
// use crate::lemmy_client::*;
// use lemmy_api_common::site::GetSiteResponse;
// use leptos::*;
// use leptos_query::{use_query, QueryOptions, QueryResult, RefetchFn, ResourceOption};

// #[server(GetSiteResource, "/serverfn", "GetJson")]
// async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
//   Ok(LemmyClient.get_site().await?)
// }

// pub fn use_site_state() -> QueryResult<Result<GetSiteResponse, LemmyAppError>, impl RefetchFn> {
//   use_query(
//     || (),
//     |_| async move {
//       LemmyClient.get_site().await
//     },
//     QueryOptions {
//       resource_option: ResourceOption::Blocking,
//       ..QueryOptions::default()
//     },
//   )
// }
