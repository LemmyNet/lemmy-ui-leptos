use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::newtypes::CommentId, lemmy_db_views::structs::CommentView,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct CommentList(Vec<Rc<RefCell<CommentNode>>>);

#[derive(Debug)]
pub struct CommentNode {
  data: CommentView,
  children: Vec<Rc<RefCell<CommentNode>>>,
}

fn parse_comment_id(id: Option<&str>) -> CommentId {
  CommentId(
    id.expect("All comments have at least 2 path segments.")
      .parse::<i32>()
      .expect("Comment IDs are always valid 32-bit integers."),
  )
}

/// Only used during construction of `CommentList`
struct CommentListAccumulator {
  node_map: HashMap<CommentId, Rc<RefCell<CommentNode>>>,
  orphan_map: HashMap<CommentId, Vec<Rc<RefCell<CommentNode>>>>,
  top_level_comments: Vec<Rc<RefCell<CommentNode>>>,
}

impl CommentList {
  pub fn new(comment_views: impl IntoIterator<Item = CommentView>) -> Self {
    // TODO: Make unit tests for this. Lemmy API common types not implementing `Default` make this
    // much harder to do than expected.

    Self(
      comment_views
        .into_iter()
        .fold(
          CommentListAccumulator {
            node_map: HashMap::new(),
            orphan_map: HashMap::new(),
            top_level_comments: Vec::new(),
          },
          |mut acc, comment_view| {
            let mut path_segments = comment_view.comment.path.split('.').rev();

            let (own_id, parent_id) = (
              parse_comment_id(path_segments.next()),
              parse_comment_id(path_segments.next()),
            );

            let children = acc.orphan_map.remove(&own_id).unwrap_or_else(|| Vec::new());
            let node = Rc::new(RefCell::new(CommentNode {
              data: comment_view,
              children,
            }));
            acc.node_map.insert(own_id, Rc::clone(&node));

            if let Some(parent_node) = acc.node_map.get(&parent_id) {
              (*parent_node.borrow_mut()).children.push(Rc::clone(&node));
            }
            // Parent ID is 0 if it is a top level comment (i.e. it cannot be any other comment's child).
            else if parent_id.0 == 0 {
              acc.top_level_comments.push(Rc::clone(&node));
            } else {
              let orphans = acc.orphan_map.entry(parent_id).or_default();
              orphans.push(Rc::clone(&node));
            }

            acc
          },
        )
        .top_level_comments,
    )
  }
}

impl IntoIterator for CommentList {
  type Item = Rc<RefCell<CommentNode>>;

  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}
