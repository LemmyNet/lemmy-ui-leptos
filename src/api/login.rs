use crate::{
  api::{api_wrapper, HttpType},
  errors::LemmyAppError,
};
use lemmy_api_common::person::{Login, LoginResponse};

pub async fn login(form: &Login) -> Result<LoginResponse, LemmyAppError> {
  api_wrapper::<LoginResponse, Login>(HttpType::Post, "user/login", form).await
}
