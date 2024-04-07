use crate::{
  cookie::get_cookie,
  errors::{LemmyAppError, LemmyAppErrorType, LemmyAppResult},
  host::{get_host, get_https},
};
use cfg_if::cfg_if;
use lemmy_api_common::{comment::*, community::*, person::*, post::*, site::*, LemmyErrorType};
use leptos::Serializable;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
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
  use leptos::Serializable;
  use serde::{Deserialize, Serialize};

  pub trait PrivateFetch {
    async fn make_request<Response, Form, Request>(
      &self,
      method: HttpType,
      path: &str,
      form: Request,
    ) -> LemmyAppResult<Response>
    where
      Response: Serializable + for<'de> Deserialize<'de> + 'static,
      Form: Serialize + core::clone::Clone + 'static + core::fmt::Debug,
      Request: Into<LemmyRequest<Form>>;
  }
}

pub trait PublicFetch: private_trait::PrivateFetch {
  async fn login(&self, form: Login) -> LemmyAppResult<LoginResponse> {
    self.make_request(HttpType::Post, "user/login", form).await
  }

  async fn logout(&self) -> LemmyAppResult<()> {
    let _ = self
      .make_request::<(), (), ()>(HttpType::Post, "user/logout", ())
      .await;
    // TODO: do not ignore error due to not being able to decode enpty http response cleanly
    Ok(())
  }

  async fn list_communities(
    &self,
    form: ListCommunities,
  ) -> LemmyAppResult<ListCommunitiesResponse> {
    self
      .make_request(HttpType::Get, "community/list", form)
      .await
  }

  async fn get_comments(&self, form: GetComments) -> LemmyAppResult<GetCommentsResponse> {
    self.make_request(HttpType::Get, "comment/list", form).await
  }

  async fn list_posts(&self, form: GetPosts) -> LemmyAppResult<GetPostsResponse> {
    self.make_request(HttpType::Get, "post/list", form).await
  }

  async fn get_post(&self, form: GetPost) -> LemmyAppResult<GetPostResponse> {
    self.make_request(HttpType::Get, "post", form).await
  }

  async fn get_site(&self) -> LemmyAppResult<GetSiteResponse> {
    self.make_request(HttpType::Get, "site", ()).await
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

        use actix_web::web;
        use actix_web::cookie::time::OffsetDateTime;
        use awc::{Client, ClientRequest};
        use leptos_actix::{extract};

        pub struct LemmyClient;

        trait MaybeBearerAuth {
            fn maybe_bearer_auth(self, token: Option<impl core::fmt::Display>) -> Self;
        }

        impl MaybeBearerAuth for ClientRequest {
            fn maybe_bearer_auth(self, token: Option<impl core::fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.bearer_auth(token)
                } else {
                    self
                }
            }
        }

        impl private_trait::PrivateFetch for LemmyClient {
            async fn make_request<Response, Form, Request>(
                &self,
                method: HttpType,
                path: &str,
                req: Request,
            ) -> LemmyAppResult<Response>
            where
                Response: Serializable + for<'de> Deserialize<'de> + 'static,
                Form: Serialize + core::clone::Clone + 'static + core::fmt::Debug,
                Request: Into<LemmyRequest<Form>>,
            {
                let LemmyRequest {body, ..} = req.into();

                let jwt = get_cookie("jwt").await?;

                let route = build_route(path);

                // cache busting code
                let query = serde_urlencoded::to_string(&body).unwrap_or("".to_string());
                // let query = format!("{}?{}&cache_bust={}", route, query, OffsetDateTime::now_utc().unix_timestamp());
                let query = format!("{}?{}", route, query);

                leptos::logging::log!("{}", query);

                let client = extract::<web::Data<Client>>().await?;

                let mut r = match method {
                    HttpType::Get => client
                        // normal request code
                        // .get(&route)
                        .get(&query)
                        .maybe_bearer_auth(jwt.clone())
                        // normal request code
                        // .query(&body)?
                        .send(),
                    HttpType::Post => client
                        .post(&route)
                        .maybe_bearer_auth(jwt.clone())
                        .send_json(&body),
                    HttpType::Put => client
                        .put(&route)
                        .maybe_bearer_auth(jwt.clone())
                        .send_json(&body)
                }.await?;

                match r.status().as_u16() {
                    400..=599 => {
                        let api_result = r.json::<LemmyErrorType>().await;

                        match api_result {
                            Ok(le) => {
                              return Err(LemmyAppError{ error_type: LemmyAppErrorType::ApiError(le.clone()), content: format!("{:#?}", le) })
                            },
                            Err(e) => {
                              return Err(LemmyAppError{ error_type: LemmyAppErrorType::Unknown, content: format!("{:#?}", e) })
                            },
                        }
                    },
                    _ => {
                    },
                };

                r.json::<Response>().await.map_err(Into::into)
            }
        }

        impl PublicFetch for LemmyClient {}

    } else {

        use leptos::wasm_bindgen::UnwrapThrowExt;
        use web_sys::AbortController;
        use gloo_net::{http, http::RequestBuilder};

        pub struct LemmyClient;

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

        impl private_trait::PrivateFetch for LemmyClient {
            async fn make_request<Response, Form, Request>(
                &self,
                method: HttpType,
                path: &str,
                req: Request,
            ) -> LemmyAppResult<Response>
            where
                Response: Serializable + for<'de> Deserialize<'de> + 'static,
                Form: Serialize + core::clone::Clone + 'static + core::fmt::Debug,
                Request: Into<LemmyRequest<Form>>,
            {
                let LemmyRequest { body, .. } = req.into();
                let route = &build_route(path);

                let jwt = get_cookie("jwt").await?;

                let abort_controller = AbortController::new().ok();
                let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                leptos::on_cleanup( move || {
                    if let Some(abort_controller) = abort_controller {
                        abort_controller.abort()
                    }
                });

                let r = match method {
                    HttpType::Get => http::Request::
                        // cache busting code
                        // get(&format!("{}&cache_bust={}", build_fetch_query(path, body), chrono::offset::Utc::now().timestamp()))
                        get(&format!("{}", build_fetch_query(path, body)))
                        // normal request code
                        // get(&build_fetch_query(path, body))
                        .maybe_bearer_auth(jwt.as_deref())
                        .abort_signal(abort_signal.as_ref())
                        .build()
                        .expect_throw("Could not parse query params"),
                    HttpType::Post => http::Request::post(route)
                        .maybe_bearer_auth(jwt.as_deref())
                        .abort_signal(abort_signal.as_ref())
                        .json(&body)
                        .expect_throw("Could not parse json body"),
                    HttpType::Put => http::Request::put(route)
                        .maybe_bearer_auth(jwt.as_deref())
                        .abort_signal(abort_signal.as_ref())
                        .json(&body)
                        .expect_throw("Could not parse json body")
                }.send().await?;

                match r.status() {
                    400..=599 => {
                        let api_result = r.json::<LemmyErrorType>().await;
                        match api_result {
                            Ok(le) => {
                                return Err(LemmyAppError{ error_type: LemmyAppErrorType::ApiError(le.clone()), content: format!("{:#?}", le) })
                            },
                            Err(e) => {
                                return Err(LemmyAppError{ error_type: LemmyAppErrorType::Unknown, content: format!("{:#?}", e) })
                            },
                        }
                    },
                    _ => {
                    },
                };

                r.json::<Response>().await.map_err(Into::into)
            }
        }

        impl PublicFetch for LemmyClient {}

        fn build_fetch_query<T: Serialize>(path: &str, form: T) -> String {
            let form_str = serde_urlencoded::to_string(&form).unwrap_or("".to_string());
            format!("{}?{}", build_route(path), form_str)
        }

    }
}

fn build_route(route: &str) -> String {
  format!(
    "http{}://{}/api/v3/{}",
    if get_https() == "true" { "s" } else { "" },
    get_host(),
    route
  )
}
