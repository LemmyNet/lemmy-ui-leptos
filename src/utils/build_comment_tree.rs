use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::newtypes::CommentId,
  lemmy_db_views::structs::CommentView,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct CommentNode {
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

pub fn build_comment_tree(comment_views: Vec<CommentView>) {
  let comments = comment_views
    .into_iter()
    .fold(
      (HashMap::new(), HashMap::new(), Vec::new()),
      |(mut node_map, mut orphan_map, mut top_level_comments), comment_view| {
        let mut path_segments = comment_view.comment.path.split('.').rev();

        let (own_id, parent_id) = (
          parse_comment_id(path_segments.next()),
          parse_comment_id(path_segments.next()),
        );

        let children = orphan_map.remove(&own_id).unwrap_or_else(|| Vec::new());
        let node = Rc::new(RefCell::new(CommentNode {
          data: comment_view,
          children,
        }));
        node_map.insert(own_id, Rc::clone(&node));

        if let Some(parent_node) = node_map.get(&parent_id) {
          (*parent_node.borrow_mut()).children.push(Rc::clone(&node));
        }
        // Parent ID is 0 if it is a top level comment (i.e. it cannot be any other comment's child).
        else if parent_id.0 == 0 {
          top_level_comments.push(Rc::clone(&node));
        } else {
          let orphans = orphan_map.entry(parent_id).or_default();
          orphans.push(Rc::clone(&node));
        }

        (node_map, orphan_map, top_level_comments)
      },
    )
    .2;
}
