use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{server_fn::codec::GetUrl, *};
use leptos_query::{create_query, QueryOptions, QueryResult, QueryScope, ResourceOption};
use std::{rc::Rc, time::Duration};

#[server(GetSiteResource, "/serverfn", input = GetUrl)]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session::get_client_and_session};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .get_site(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

fn use_site_state() -> QueryScope<(), Result<GetSiteResponse, ServerFnError>> {
  create_query(
    |_| async move { get_site().await },
    QueryOptions {
      resource_option: Some(ResourceOption::Blocking),
      stale_time: Some(Duration::from_secs(1800)),
      gc_time: Some(Duration::from_secs(3600)),
      ..QueryOptions::default()
    },
  )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserLoggedIn(pub bool);

#[derive(Clone)]
pub struct SiteRefetchFn(pub Rc<dyn Fn()>);

pub type SiteStateSignal = Signal<Option<Result<GetSiteResponse, ServerFnError>>>;

pub fn provide_site_state() {
  let QueryResult { data, refetch, .. } = use_site_state().use_query(|| ());
  let user_is_logged_in = Signal::derive(move || {
    with!(|data| UserLoggedIn(
      data
        .as_ref()
        .and_then(|data| data.as_ref().ok())
        .map_or(false, |data| data.my_user.is_some())
    ))
  });

  provide_context(data);
  provide_context(user_is_logged_in);
  provide_context(SiteRefetchFn(Rc::new(refetch)));
}
