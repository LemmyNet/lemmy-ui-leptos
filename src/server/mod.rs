use cfg_if::cfg_if;

mod api_service;
mod cookie_middleware;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use api_service::route_to_api;
        pub use cookie_middleware::cookie_middleware;
    }
}
