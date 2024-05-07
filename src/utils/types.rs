use leptos::{
  server_fn::{client::browser::BrowserClient, codec::PostUrl, ServerFn},
  Action,
  ServerFnError,
};
use serde::de::DeserializeOwned;
use trait_set::trait_set;

trait_set! {
    pub trait ServerActionFn = DeserializeOwned + ServerFn<InputEncoding = PostUrl, Client = BrowserClient> + 'static;
}

pub type ServerAction<T: ServerActionFn> = Action<T, Result<T::Output, ServerFnError<T::Error>>>;
