use crate::{
  api::{api_wrapper, HttpType},
  errors::LemmyAppError,
};
use lemmy_api_common::person::{Login, LoginResponse};
use leptos::Scope;

pub async fn login(cx: Option<Scope>, form: &Login) -> Result<LoginResponse, LemmyAppError> {
  api_wrapper::<LoginResponse, Login>(cx, HttpType::Post, "user/login", form).await
}
