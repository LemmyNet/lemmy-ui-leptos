use crate::{errors::LemmyAppError, host::get_host};
use async_trait::async_trait;
use cfg_if::cfg_if;
use lemmy_api_common::{
  comment::{GetComments, GetCommentsResponse},
  person::{Login, LoginResponse},
  post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse},
};
use leptos::Serializable;
use serde::{Deserialize, Serialize};

pub enum HttpType {
  #[allow(dead_code)]
  Get,
  #[allow(dead_code)]
  Post,
  #[allow(dead_code)]
  Put,
}

pub struct LemmyRequest<'a, R: Serialize> {
  pub body: R,
  pub jwt: Option<&'a str>,
}

mod private_trait {
  use super::{HttpType, LemmyRequest};
  use crate::errors::LemmyAppError;
  use async_trait::async_trait;
  use leptos::Serializable;
  use serde::{Deserialize, Serialize};

  #[async_trait(?Send)]
  pub trait LemmyClient {
    async fn make_request<'a, Response: Serializable + for<'de> Deserialize<'de>, Form: Serialize>(
      &self,
      method: HttpType,
      path: &str,
      form: LemmyRequest<'a, Form>,
    ) -> Result<Response, LemmyAppError>;
  }
}

#[async_trait(?Send)]
pub trait LemmyClient: private_trait::LemmyClient {
  async fn login<'a>(&self, form: LemmyRequest<'a, Login>) -> Result<LoginResponse, LemmyAppError> {
    self.make_request(HttpType::Post, "user/login", form).await
  }

  async fn get_comments<'a>(
    &self,
    form: LemmyRequest<'a, GetComments>,
  ) -> Result<GetCommentsResponse, LemmyAppError> {
    self.make_request(HttpType::Get, "comment/list", form).await
  }

  async fn list_posts<'a>(
    &self,
    form: LemmyRequest<'a, GetPosts>,
  ) -> Result<GetPostsResponse, LemmyAppError> {
    self.make_request(HttpType::Get, "post/list", form).await
  }

  async fn get_post<'a>(
    &self,
    form: LemmyRequest<'a, GetPost>,
  ) -> Result<GetPostResponse, LemmyAppError> {
    self.make_request(HttpType::Get, "post", form).await
  }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        trait MaybeBearerAuth {
            fn maybe_bearer_auth(self, token: Option<impl std::fmt::Display>) -> Self;
        }

        impl MaybeBearerAuth for awc::ClientRequest {
            fn maybe_bearer_auth(self, token: Option<impl std::fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.bearer_auth(token)
                } else {
                    self
                }
            }
        }

        #[async_trait(?Send)]
        impl private_trait::LemmyClient for awc::Client {
            async fn make_request<'a, Response: Serializable + for<'de> Deserialize<'de>, Form: Serialize>(
               &self,
               method: HttpType,
               path: &str,
               LemmyRequest{body, jwt}: LemmyRequest<'a, Form>,
           ) -> Result<Response, LemmyAppError> {
                let route = &build_route(path);

                match method {
                    HttpType::Get => {
                        self
                            .get(route)
                            .maybe_bearer_auth(jwt)
                            .query(&body)?
                            .send()
                    }
                    HttpType::Post =>
                        self
                            .post(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    HttpType::Put =>
                        self
                            .put(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body)
                }.await?.json::<Response>().await.map_err(Into::into)
            }
        }

        impl LemmyClient for awc::Client {}
    } else {
        use leptos::wasm_bindgen::UnwrapThrowExt;
        use web_sys::AbortController;
        use gloo_net::http::{Request, RequestBuilder};

        pub struct Fetch;

        trait MaybeBearerAuth {
            fn maybe_bearer_auth(self, token: Option<&str>) -> Self;
        }

        impl MaybeBearerAuth for RequestBuilder {
           fn maybe_bearer_auth(self, token: Option<&str>) -> Self {
                if let Some(token) = token {
                    self.header("Authorization", format!("Bearer {token}").as_str())
                } else {
                    self
                }
            }
        }

        #[async_trait(?Send)]
       impl private_trait::LemmyClient for Fetch {
           async fn make_request<'a, Response: Serializable + for<'de> Deserialize<'de>, Form: Serialize>(
               &self,
               method: HttpType,
               path: &str,
               LemmyRequest{body, jwt}: LemmyRequest<'a, Form>,
           ) -> Result<Response, LemmyAppError> {
               let route = &build_route(path);
               let abort_controller = AbortController::new().ok();
               let abort_signal = abort_controller.as_ref().map(AbortController::signal);

               // abort in-flight requests if the Scope is disposed
               // i.e., if we've navigated away from this page
               leptos::on_cleanup( move || {
                   if let Some(abort_controller) = abort_controller {
                       abort_controller.abort()
                   }
               });

               match method {
                   HttpType::Get => {
                       Request::get(&build_fetch_query(path, body))
                           .maybe_bearer_auth(jwt)
                           .abort_signal(abort_signal.as_ref())
                           .build()
                           .expect_throw("Could not parse query params")
                   }
                   HttpType::Post => {
                       Request::post(route)
                           .maybe_bearer_auth(jwt)
                           .abort_signal(abort_signal.as_ref())
                           .json(&body)
                           .expect_throw("Could not parse json body")
                   }
                   HttpType::Put => {
                       Request::put(route)
                           .maybe_bearer_auth(jwt)
                           .abort_signal(abort_signal.as_ref())
                           .json(&body)
                           .expect_throw("Could not parse json body")
                   }
               }.send().await?.json::<Response>().await.map_err(Into::into)
           }
       }

        impl LemmyClient for Fetch {}

        fn build_fetch_query<T: Serialize>(path: &str, form: T) -> String {
            let form_str = serde_urlencoded::to_string(&form).unwrap_or(path.to_string());
            format!("{path}?{form_str}")
        }
    }
}

fn build_route(route: &str) -> String {
  format!("http://{}/api/v3/{route}", get_host())
}
