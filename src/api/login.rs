use crate::api::{api_wrapper, HttpType};
use anyhow::Result;
use lemmy_api_common::person::{Login, LoginResponse};
use leptos::Scope;

pub async fn login(cx: Scope, form: &Login) -> Result<LoginResponse> {
  api_wrapper::<LoginResponse, Login>(cx, HttpType::Post, "user/login", form).await
}
