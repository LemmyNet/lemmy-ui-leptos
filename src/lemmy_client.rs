use crate::{
  errors::{LemmyAppError, LemmyAppErrorType, LemmyAppResult},
  host::{get_host, get_https},
  lemmy_errors::LemmyErrorType, cookie::get_cookie,
};
// use async_trait::async_trait;
use cfg_if::cfg_if;
use lemmy_api_common::{comment::*, community::*, person::*, post::*, site::*};
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
  // use async_trait::async_trait;
  use leptos::Serializable;
  use serde::{Deserialize, Serialize};

  // #[async_trait(?Send)]
  pub trait LemmyClient {
    async fn make_request<Response, Form, Request>(
      &self,
      method: HttpType,
      path: &str,
      form: Request,
    ) -> LemmyAppResult<Response>
    where
      Response: Serializable + for<'de> Deserialize<'de>,
      Form: Serialize + std::clone::Clone + 'static + std::fmt::Debug,
      Request: Into<LemmyRequest<Form>>;
  }
}

// #[async_trait(?Send)]
pub trait LemmyClient: private_trait::LemmyClient {
  async fn login(&self, form: Login) -> LemmyAppResult<LoginResponse> {
    // leptos::logging::log!("FORM {:#?}", form);

    let r = self.make_request(HttpType::Post, "user/login", form).await;

    // if let Ok(LoginResponse { jwt: Some(ref s), .. }) = r {
    //   leptos::logging::log!("JW {:#?}", s.clone().into_inner());
    // }

    // leptos::logging::log!("LOGIN {:#?}", r);

    r
  }

  async fn logout(&self, form: ()) -> LemmyAppResult<()> {
    // leptos::logging::log!("FORM {:#?}", form);

    let r = self.make_request(HttpType::Post, "user/logout", form).await;

    // if let Ok(LoginResponse { jwt: Some(ref s), .. }) = r {
    //   leptos::logging::log!("JW {:#?}", s.clone().into_inner());
    // }

    // leptos::logging::log!("LOGIN {:#?}", r);

    r
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
    let r = self.make_request(HttpType::Get, "post/list", form).await;

    // leptos::logging::log!("LIST {:#?}", r);

    r

    // Ok(GetPostsResponse { posts: vec![], next_page: None })
  }

  async fn get_post(&self, form: GetPost) -> LemmyAppResult<GetPostResponse> {
    self.make_request(HttpType::Get, "post", form).await
  }

  async fn get_site(&self, jwt: Option<String>) -> LemmyAppResult<GetSiteResponse> {
    leptos::logging::log!("JWT {:#?}", jwt.clone());

    let r: Result<GetSiteResponse, LemmyAppError> = self
      .make_request(HttpType::Get, "site", LemmyRequest::<()>::from_jwt(jwt))
      .await;

    // leptos::logging::log!("SITE {:#?}", r.clone().ok().unwrap().my_user);

    r
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
    // leptos::logging::log!("FORM {:#?}", form);

    let r = self.make_request(HttpType::Post, "post/like", form).await;

    // if let Ok(LoginResponse { jwt: Some(ref s), .. }) = r {
    //   leptos::logging::log!("JW {:#?}", s.clone().into_inner());
    // }

    // leptos::logging::log!("LIKE {:#?}", r);

    r
  }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {

        pub struct Fetch;

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

        use actix_session::Session;


        // #[async_trait(?Send)]
        impl private_trait::LemmyClient for Fetch {
            async fn make_request<Response, Form, Request>(
                &self,
                method: HttpType,
                path: &str,
                req: Request,
            ) -> LemmyAppResult<Response>
            where
                Response: Serializable + for<'de> Deserialize<'de>,
                Form: Serialize + std::clone::Clone + 'static + std::fmt::Debug,
                Request: Into<LemmyRequest<Form>>,
            {
                let LemmyRequest {body, jwt: _} = req.into();

                let jwt = get_cookie("jwt").await?; // { Ok(o) => o, _ => None };
                    
                // let jwt = extract(|session: Session| async move {
                //   session.get::<String>("jwt")
                // })
                // .await??;

                leptos::logging::log!("make JWT {:#?} ", jwt);

                let route = build_route(path);

                use actix_web::web;
                use awc::Client;
                use leptos_actix::{extract};

                let result = extract(|client: web::Data<Client>| async move {
                  let mut r = match method {
                      HttpType::Get =>
                          client
                              .get(&route)
                              .maybe_bearer_auth(jwt.clone())
                              .query(&body)?
                              .send(),
                      HttpType::Post =>
                          client
                              .post(&route)
                              .maybe_bearer_auth(jwt.clone())
                              .send_json(&body),
                      HttpType::Put =>
                          client
                              .put(&route)
                              .maybe_bearer_auth(jwt.clone())
                              .send_json(&body)
                  }.await?;

                  match r.status().as_u16() {
                    400..=499 | 500..=599 => {
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

                }).await?;

                result
            }
        }

        impl LemmyClient for Fetch {}

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

        // #[async_trait(?Send)]
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

                let jwt = get_cookie("jwt").await?;

                // let jwt = get("jwt").and_then(Result::ok);

                let abort_controller = AbortController::new().ok();
                let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                leptos::on_cleanup( move || {
                    if let Some(abort_controller) = abort_controller {
                        abort_controller.abort()
                    }
                });

                let r = match method {
                  HttpType::Get =>
                      Request::get(&build_fetch_query(path, body))
                          .maybe_bearer_auth(jwt.as_deref())
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
                }.send().await?;


                match r.status() {
                  400..=499 | 500..=599 => {
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
        "//{}/api/v3/{}",
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
