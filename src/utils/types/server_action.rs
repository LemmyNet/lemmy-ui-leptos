use leptos::server_fn::{client::browser::BrowserClient, codec::PostUrl, ServerFn};
use serde::de::DeserializeOwned;
use trait_set::trait_set;

trait_set! {
    pub trait ServerActionFn = DeserializeOwned + ServerFn<InputEncoding = PostUrl, Client = BrowserClient> + 'static;
}
