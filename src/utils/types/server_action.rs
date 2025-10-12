use leptos::server_fn::{
  client::browser::BrowserClient,
  codec::PostUrl,
  error::ServerFnError,
  ServerFn,
};
use serde::de::DeserializeOwned;

pub trait ServerActionFn:
  DeserializeOwned
  + Clone
  + Send
  + Sync
  + 'static
  + ServerFn<
    InputEncoding = PostUrl,
    Client = BrowserClient,
    Output = Self::Out,
    Error = ServerFnError,
  >
{
  type Out: Send + Sync + 'static;
}
