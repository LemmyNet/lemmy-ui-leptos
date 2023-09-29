use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_web::{HttpRequest, HttpResponse, web, http::uri::{Uri, Authority}};
        use actix_proxy::{IntoHttpResponse, SendRequestError as ProxyError};
        use awc::{Client, ClientRequest, error::{InvalidUrl, SendRequestError}};
        use serde::{Deserialize, Serialize};

        pub async fn route_to_api(
            request: HttpRequest,
            client: web::Data<Client>,
            body: web::Json<dyn Serialize>,
            uri: Uri
        ) -> Result<HttpResponse, ProxyError> {
            let mut parts = uri.into_parts();
            parts.authority = Some(Authority::from_static("localhost:8536"));

            client.request_from(Uri::from_parts(parts)
                                .map_err(|e| SendRequestError::Url(InvalidUrl::HttpError(e.into())))?, &request.head())
                                .send_json(&body).await?
                                .into_wrapped_http_response()
        }
    }
}
