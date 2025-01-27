use leptos::server_fn::{
  client::browser::BrowserClient, codec::PostUrl, error::NoCustomError, ServerFn,
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
    Error = NoCustomError,
  >
{
  type Out: Send + Sync + 'static;
}
