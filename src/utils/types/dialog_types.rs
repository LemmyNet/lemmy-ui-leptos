use super::PostOrCommentId;
use leptos::{html::Dialog, prelude::NodeRef};

#[derive(Clone, Default)]
pub struct ReportModalData {
  pub post_or_comment_id: PostOrCommentId,
  pub creator_actor_id: String,
  pub creator_name: String,
}

#[derive(Clone, Copy)]
pub struct ReportModalNode(pub NodeRef<Dialog>);
