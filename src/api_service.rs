use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_web::{HttpRequest, HttpResponse, web, http::uri::{Uri, Authority, InvalidUri, InvalidUriParts}};
        use actix_proxy::{IntoHttpResponse, SendRequestError as ProxyError};
        use awc::{Client, ClientRequest, error::{InvalidUrl, SendRequestError, HttpError}};
        use serde::{Deserialize, Serialize};
        use crate::host::get_host;

        pub async fn route_to_api(
            request: HttpRequest,
            client: web::Data<Client>,
            payload: web::Payload,
            uri: Uri
        ) -> Result<HttpResponse, ProxyError> {
            let mut parts = uri.into_parts();
            parts.authority = Some(get_host().try_into().map_err(map_uri_err::<InvalidUri>)?);
            parts.scheme = Some(format!("http{}",
                                        if std::env::var("LEMMY_UI_HTTPS")
                                            .unwrap_or(String::from("false")) == "true"
                                        {"s"}
                                        else
                                        {""}
                            )
                                .as_str()
                                .try_into()
                                .map_err(map_uri_err::<InvalidUri>)?
            );

            let uri = Uri::from_parts(parts)
                                .map_err(map_uri_err::<InvalidUriParts>)?;

            client.request_from(uri, &request.head())
                                .no_decompress()
                                .send_stream(payload).await?
                                .into_wrapped_http_response()
        }

        fn map_uri_err<E: Into<HttpError>>(e: E) -> SendRequestError {
            SendRequestError::Url(InvalidUrl::HttpError(e.into()))
        }
    }
}
