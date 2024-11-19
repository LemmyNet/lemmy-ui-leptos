use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;

pub struct CommentTree<'a> {
  node: &'a CommentView,
  children: Option<Vec<CommentTree<'a>>>,
}

impl<'a> CommentTree<'a> {
  pub fn new(node: &'a CommentView, children: Option<Vec<CommentTree<'a>>>) -> Self {
    Self { node, children }
  }

  pub fn node(&self) -> &'a CommentView {
    self.node
  }

  pub fn children(&self) -> &Option<Vec<Self>> {
    &self.children
  }
}
