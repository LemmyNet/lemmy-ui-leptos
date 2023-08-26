use crate::api_fn;
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};

api_fn!(
  get_comments,
  GetComments,
  GetCommentsResponse,
  Get,
  "comment/list"
);
