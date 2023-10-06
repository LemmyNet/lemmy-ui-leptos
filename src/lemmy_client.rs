use crate::{errors::LemmyAppError, host::get_host};
use async_trait::async_trait;
use cfg_if::cfg_if;
use lemmy_api_common::{
  comment::{GetComments, GetCommentsResponse},
  person::{Login, LoginResponse},
  post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse},
};
use leptos::Serializable;
use log::logger;
use serde::{Deserialize, Serialize};

pub enum HttpType {
  #[allow(dead_code)]
  Get,
  #[allow(dead_code)]
  Post,
  #[allow(dead_code)]
  Put,
}

mod private_trait {
  use super::HttpType;
  use crate::errors::LemmyAppError;
  use async_trait::async_trait;
  use leptos::Serializable;
  use serde::{Deserialize, Serialize};

  #[async_trait(?Send)]
  pub trait LemmyClient {
    async fn make_request<Response: Serializable + for<'de> Deserialize<'de>, Form: Serialize>(
      &self,
      method: HttpType,
      path: &str,
      form: &Form,
    ) -> Result<Response, LemmyAppError>;
  }
}

#[async_trait(?Send)]
pub trait LemmyClient: private_trait::LemmyClient {
  async fn login(&self, form: &Login) -> Result<LoginResponse, LemmyAppError> {
    self.make_request(HttpType::Post, "user/login", form).await
  }

  async fn get_comments(&self, form: &GetComments) -> Result<GetCommentsResponse, LemmyAppError> {
    self.make_request(HttpType::Get, "comment/list", form).await
  }

  async fn list_posts(&self, form: &GetPosts) -> Result<GetPostsResponse, LemmyAppError> {
    self.make_request(HttpType::Get, "post/list", form).await
  }

  async fn get_post(&self, form: &GetPost) -> Result<GetPostResponse, LemmyAppError> {
    self.make_request(HttpType::Get, "post", form).await
  }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[async_trait(?Send)]
        impl private_trait::LemmyClient for awc::Client {
            async fn make_request<Response: Serializable + for<'de> Deserialize<'de>, Form: Serialize>(
               &self,
               method: HttpType,
               path: &str,
               form: &Form,
            ) -> Result<Response, LemmyAppError> {
                use crate::api::get_cookie_wrapper;

                let route = &build_route(path);

                let mut request_builder = match method {
                    HttpType::Get => self.get(route).query(form)?,
                    HttpType::Post => self.post(route),
                    HttpType::Put => self.put(route),
                };

                match get_cookie_wrapper("jwt").await {
                  Ok(Some(jwt)) => {
                    request_builder = request_builder.insert_header(("Authorization", &format!("Bearer {}", jwt)[..]));
                  },
                  _ => {
                  },
                };

                leptos::logging::log!("{:#?}", route);

                match method {
                  HttpType::Get => request_builder.send(),
                  HttpType::Post => request_builder.send_json(form),
                  HttpType::Put => request_builder.send_json(form)
                }.await?.json::<Response>().await.map_err(Into::into)

                // match method {
                //   HttpType::Get => {
                //       self
                //           .get(route)
                //           .query(form)?
                //           .send()
                //   }
                //   HttpType::Post => self.post(route).send_json(form),
                //   HttpType::Put => self.put(route).send_json(form)
                // }.await?.json::<Response>().await.map_err(Into::into)

          }
        }

        impl LemmyClient for awc::Client {}
    } else {
        use crate::wasm_bindgen::UnwrapThrowExt;
        use web_sys::AbortController;
        use gloo_net::http::Request;

        pub struct Fetch;

        #[async_trait(?Send)]
        impl private_trait::LemmyClient for Fetch {
           async fn make_request<Response: Serializable + for<'de> Deserialize<'de>, Form: Serialize>(
               &self,
               method: HttpType,
               path: &str,
               form: &Form,
           ) -> Result<Response, LemmyAppError> {
              use crate::api::get_cookie_wrapper;

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

              //  let mut request_builder = match method {
              //     HttpType::Get => {
              //         Request::get(&build_fetch_query(path, form))
              //           .abort_signal(abort_signal.as_ref())
              //     }
              //     HttpType::Post => {
              //         Request::post(route)
              //         .abort_signal(abort_signal.as_ref())
              //     }
              //     HttpType::Put => {
              //         Request::put(route)
              //         .abort_signal(abort_signal.as_ref())
              //     }
              //   };

              //   match get_cookie_wrapper("jwt").await {
              //     Ok(Some(jwt)) => {
              //       request_builder = request_builder.header("Authorization", &format!("Bearer {}", jwt)[..]);
              //     },
              //     _ => {
              //     },
              //   };

              //   match method {
              //     HttpType::Get => {
              //         request_builder.send().await
              //     }
              //     HttpType::Post => {
              //         request_builder.json(form)
              //         .expect_throw("Could not parse json body").send().await
              //     }
              //     HttpType::Put => {
              //         request_builder.json(form)
              //         .expect_throw("Could not parse json body").send().await
              //     }
              //   }?.json::<Response>().await.map_err(Into::into)

               match method {
                   HttpType::Get => {
                       Request::get(&build_fetch_query(path, form))
                         .abort_signal(abort_signal.as_ref())
                         .send().await
                   }
                   HttpType::Post => {
                       Request::post(route)
                        .abort_signal(abort_signal.as_ref())
                        .json(form)
                           .expect_throw("Could not parse json body")
                        .send().await
                   }
                   HttpType::Put => {
                       Request::put(route)
                        .abort_signal(abort_signal.as_ref())
                        .json(form)
                           .expect_throw("Could not parse json body")
                        .send().await
                   }
               }?.json::<Response>().await.map_err(Into::into)
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
