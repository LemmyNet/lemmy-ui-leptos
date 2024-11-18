use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use std::collections::HashMap;

struct CommentNode {
  pub node: CommentView,
  pub children: Option<Vec<CommentNode>>,
}

fn build_tree_comments(comment_views: Vec<CommentView>) -> Vec<CommentNode> {
  let parent_tree = comment_views.iter().fold(HashMap::new(), |mut m, c| {
    let parent = get_parent(&c.comment.path);

    let v = m.entry(parent).or_insert_with(|| Vec::new());
    v.push(c);

    m
  });

  parent_tree
    .get(&0)
    .cloned()
    .unwrap_or_default()
    .into_iter()
    .map(|c| build_children(c, &parent_tree))
    .collect()
}

fn build_children(c: &CommentView, m: &HashMap<i32, Vec<&CommentView>>) -> CommentNode {
  let children = m
    .get(&c.comment.id.0)
    .map(|v| v.iter().map(|c| build_children(c, m)).collect());

  CommentNode {
    node: c.clone(),
    children,
  }
}

fn get_parent(path: &str) -> i32 {
  path
    .split('.')
    .rev()
    .skip(1)
    .next()
    .expect(r#"Comments should always have parents; top level comments have "0" as a parent."#)
    .parse()
    .expect("Comment IDs should be valid i32s")
}
