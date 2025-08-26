use leptos::server_fn::{
  actix::ActixServerFnBackend,
  client::browser::BrowserClient,
  codec::PostUrl,
  ServerFn,
};
use serde::de::DeserializeOwned;

pub trait ServerActionFn:
  DeserializeOwned
  + Clone
  + Send
  + Sync
  + 'static
  + ServerFn<Client = BrowserClient, Server = ActixServerFnBackend, Output = Self::Out>
{
  type Out: Send + Sync + 'static;
}
