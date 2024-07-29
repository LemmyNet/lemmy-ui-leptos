use super::ContentId;
use leptos::{html::Dialog, NodeRef};

#[derive(Clone, Default)]
pub struct ReportModalData {
  pub content_id: ContentId,
  pub creator_actor_id: String,
}

#[derive(Clone, Copy)]
pub struct ReportModalNode(pub NodeRef<Dialog>);
