use crate::{
  errors::LemmyAppResult,
  host::{get_host, get_https},
};
use async_trait::async_trait;
use cfg_if::cfg_if;
use lemmy_api_common::{
  comment::*,
  person::*,
  post::*,
  site::*,
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

pub struct LemmyRequest<R: Serialize> {
  pub body: Option<R>,
  pub jwt: Option<String>,
}

impl<R: Serialize> LemmyRequest<R> {
  pub fn from_jwt(jwt: Option<String>) -> Self {
    Self {
      body: None::<R>,
      jwt,
    }
  }
}

impl<R: Serialize> From<R> for LemmyRequest<R> {
  fn from(body: R) -> Self {
    LemmyRequest {
      body: Some(body),
      jwt: None,
    }
  }
}

mod private_trait {
  use super::{HttpType, LemmyRequest};
  use crate::errors::LemmyAppResult;
  use async_trait::async_trait;
  use leptos::Serializable;
  use serde::{Deserialize, Serialize};

  #[async_trait(?Send)]
  pub trait LemmyClient {
    async fn make_request<Response, Form, Request>(
      &self,
      method: HttpType,
      path: &str,
      form: Request,
    ) -> LemmyAppResult<Response>
    where
      Response: Serializable + for<'de> Deserialize<'de>,
      Form: Serialize,
      Request: Into<LemmyRequest<Form>>;
  }
}

#[async_trait(?Send)]
pub trait LemmyClient: private_trait::LemmyClient {
  async fn login<T>(&self, form: T) -> LemmyAppResult<LoginResponse>
  where
    T: Into<LemmyRequest<Login>>,
  {
    self.make_request(HttpType::Post, "user/login", form).await
  }

  async fn get_comments<T>(&self, form: T) -> LemmyAppResult<GetCommentsResponse>
  where
    T: Into<LemmyRequest<GetComments>>,
  {
    self.make_request(HttpType::Get, "comment/list", form).await
  }

  async fn list_posts<T>(&self, form: T) -> LemmyAppResult<GetPostsResponse>
  where
    T: Into<LemmyRequest<GetPosts>>,
  {
    self.make_request(HttpType::Get, "post/list", form).await
  }

  async fn get_post<T>(&self, form: T) -> LemmyAppResult<GetPostResponse>
  where
    T: Into<LemmyRequest<GetPost>>,
  {
    self.make_request(HttpType::Get, "post", form).await
  }
  async fn get_site(&self, jwt: Option<String>) -> LemmyAppResult<GetSiteResponse> {
    self
      .make_request(HttpType::Get, "site", LemmyRequest::<()>::from_jwt(jwt))
      .await
  }
  async fn report_post(&self, form: CreatePostReport) -> LemmyAppResult<PostReportResponse> {
    self.make_request(HttpType::Post, "post/report", form).await
  }
  async fn block_user(&self, form: BlockPerson) -> LemmyAppResult<BlockPersonResponse> {
    self.make_request(HttpType::Post, "user/block", form).await
  }
  async fn save_post(&self, form: SavePost) -> LemmyAppResult<PostResponse> {
    self.make_request(HttpType::Put, "post/save", form).await
  }
  async fn like_post(&self, form: CreatePostLike) -> LemmyAppResult<PostResponse> {
    self.make_request(HttpType::Post, "post/like", form).await
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
            async fn make_request<Response, Form, Request>(
                &self,
                method: HttpType,
                path: &str,
                req: Request,
            ) -> LemmyAppResult<Response>
            where
                Response: Serializable + for<'de> Deserialize<'de>,
                Form: Serialize,
                Request: Into<LemmyRequest<Form>>
            {
                let LemmyRequest {body, jwt} = req.into();
                let route = &build_route(path);

                match method {
                    HttpType::Get =>
                        self
                            .get(route)
                            .maybe_bearer_auth(jwt)
                            .query(&body)?
                            .send(),
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
        use wasm_cookies::get;

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
            async fn make_request<Response, Form, Req>(
                &self,
                method: HttpType,
                path: &str,
                req: Req,
            ) -> LemmyAppResult<Response>
            where
                Response: Serializable + for<'de> Deserialize<'de>,
                Form: Serialize,
                Req: Into<LemmyRequest<Form>>
            {
                let LemmyRequest { body, .. } = req.into();
                let route = &build_route(path);
                let jwt = get("jwt").and_then(Result::ok);

                let abort_controller = AbortController::new().ok();
                let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                leptos::on_cleanup( move || {
                    if let Some(abort_controller) = abort_controller {
                        abort_controller.abort()
                    }
                });

                match method {
                    HttpType::Get =>
                        Request::get(&build_fetch_query(path, body))
                            // .maybe_bearer_auth(jwt.as_deref())
                            .header("Accept", "application/json")
                            .abort_signal(abort_signal.as_ref())
                            .build()
                            .expect_throw("Could not parse query params"),
                    HttpType::Post =>
                        Request::post(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body"),
                    HttpType::Put =>
                        Request::put(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body")
                }.send().await?.json::<Response>().await.map_err(Into::into)
            }
        }

        impl LemmyClient for Fetch {}

        fn build_fetch_query<T: Serialize>(path: &str, form: T) -> String {
            let form_str = serde_urlencoded::to_string(&form).unwrap_or(path.to_string());
            format!("{}?{}", build_route(path), form_str)
        }
    }
}

fn build_route(route: &str) -> String {
  cfg_if! {
    if #[cfg(all(not(feature = "ssr"), not(feature = "bypass_internal_proxy")))] {
      format!(
        "http{}://{}/api/v3/{}",
        "",
        get_host(),
        route
      )
    } else {
      format!(
        "http{}://{}/api/v3/{}",
        if get_https() == "true" { "s" } else { "" },
        get_host(),
        route
      )
    }
  }
}
