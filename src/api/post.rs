use crate::api_fn;
use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};

api_fn!(list_posts, GetPosts, GetPostsResponse, Get, "post/list");
api_fn!(get_post, GetPost, GetPostResponse, Get, "post");
