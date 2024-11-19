use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use std::collections::HashMap;

use super::types::CommentTree;

const ROOT_ID: i32 = 0;

pub fn build_comment_tree<'a>(comment_views: Vec<&'a CommentView>) -> Vec<CommentTree<'a>> {
  let child_map = comment_views
    .iter()
    .fold(HashMap::new(), |mut child_map, comment_view| {
      let parent = get_parent_id(&comment_view.comment.path);

      let children: &mut Vec<&CommentView> = child_map.entry(parent).or_default();
      children.push(comment_view);

      child_map
    });

  child_map
    .get(&ROOT_ID)
    .map(|comment_views| {
      comment_views
        .iter()
        .map(|comment_view| build_comment_tree_rec(comment_view, &child_map))
        .collect()
    })
    .unwrap_or_default()
}

fn build_comment_tree_rec<'a>(
  comment_view: &'a CommentView,
  child_map: &HashMap<i32, Vec<&'a CommentView>>,
) -> CommentTree<'a> {
  let children = child_map
    .get(&comment_view.comment.id.0)
    .map(|comment_views| {
      comment_views
        .iter()
        .map(|comment_view| build_comment_tree_rec(comment_view, child_map))
        .collect()
    });

  CommentTree::new(&comment_view, children)
}

fn get_parent_id(path: &str) -> i32 {
  path
    .split('.')
    .nth_back(1)
    .expect(r#"Comments should always have parents; top level comments have "0" as a parent."#)
    .parse()
    .expect("Comment IDs should be valid i32s")
}
