use super::ContentActionType;
use leptos::{html::Dialog, NodeRef};

#[derive(Clone, Default)]
pub struct ReportModalData {
  pub id: i32,
  pub content_type: ContentActionType,
  pub creator_actor_id: String,
}

#[derive(Clone, Copy)]
pub struct ReportModalNode(pub NodeRef<Dialog>);
