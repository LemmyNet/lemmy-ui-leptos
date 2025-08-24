use reactive_stores::Store;

#[derive(Debug, Default, Store)]
struct CommentStore {
  creator_banned_from_community: bool,
  banned_from_community: bool,
}
