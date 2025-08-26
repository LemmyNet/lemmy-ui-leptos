use lemmy_client::lemmy_api_common::{comment::CommentId, post::PostId};

#[derive(Clone, Copy)]
pub enum PostOrCommentId {
  Post(PostId),
  #[allow(dead_code)]
  Comment(CommentId),
}

impl PostOrCommentId {
  pub fn get_id(&self) -> i32 {
    match self {
      Self::Post(id) => id.0,
      Self::Comment(id) => id.0,
    }
  }
}

impl Default for PostOrCommentId {
  fn default() -> Self {
    Self::Post(PostId(0))
  }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Hidden(pub bool);
