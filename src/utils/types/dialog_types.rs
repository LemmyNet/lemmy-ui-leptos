use super::PostOrCommentId;
use leptos::{html::Dialog, NodeRef};

#[derive(Clone, Default)]
pub struct ReportModalData {
  pub content_id: PostOrCommentId,
  pub creator_actor_id: String,
}

#[derive(Clone, Copy)]
pub struct ReportModalNode(pub NodeRef<Dialog>);
