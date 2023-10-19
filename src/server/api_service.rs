use cfg_if::cfg_if;
use leptos::leptos_dom::logging;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::host::get_host;
        use crate::config::LEMMY_UI_HTTPS;
        use actix_proxy::{IntoHttpResponse, SendRequestError as ProxyError};
        use actix_web::{
        http::uri::{InvalidUri, InvalidUriParts, Uri},
        web,
        HttpRequest,
        HttpResponse,
        };
        use awc::{
        error::{HttpError, InvalidUrl, SendRequestError},
        Client,
        };

        pub async fn route_to_api(
        request: HttpRequest,
        client: web::Data<Client>,
        payload: web::Payload,
        uri: Uri,
        ) -> Result<HttpResponse, ProxyError> {
        let mut parts = uri.into_parts();
        parts.authority = Some(get_host().try_into().map_err(map_uri_err::<InvalidUri>)?);
        parts.scheme = Some(
            format!(
            "http{}",
            if std::env::var("LEMMY_UI_HTTPS").unwrap_or(format!("{LEMMY_UI_HTTPS}")) == "true" {
                "s"
            } else {
                ""
            }
            )
            .as_str()
            .try_into()
            .map_err(map_uri_err::<InvalidUri>)?,
        );

        let uri = Uri::from_parts(parts).map_err(map_uri_err::<InvalidUriParts>)?;

        leptos::logging::log!("proxy {:#?}", uri);

        client
            .request_from(uri, request.head())
            .no_decompress()
            .send_stream(payload)
            .await?
            .into_wrapped_http_response()
        }

        fn map_uri_err<E: Into<HttpError>>(e: E) -> SendRequestError {
        SendRequestError::Url(InvalidUrl::HttpError(e.into()))
        }
    }
}
