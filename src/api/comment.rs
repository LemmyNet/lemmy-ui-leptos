use super::HttpType;
use crate::{api::api_wrapper, errors::LemmyAppError};
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};

pub async fn get_comments(form: &GetComments) -> Result<GetCommentsResponse, LemmyAppError> {
  api_wrapper::<GetCommentsResponse, GetComments>(HttpType::Get, "comment/list", form).await
}
