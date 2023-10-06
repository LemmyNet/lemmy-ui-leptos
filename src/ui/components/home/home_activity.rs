// use actix_web::web;
use crate::ui::components::post::post_listings::PostListings;
use lemmy_api_common::post::{GetPosts, GetPostsResponse};
use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn HomeActivity() -> impl IntoView {
  let query = use_query_map();
  let page = move || {
    query
      .with(|q| q.get("page").and_then(|page| page.parse::<i64>().ok()))
      .unwrap_or(1)
  };

  let authenticated = use_context::<RwSignal<bool>>().unwrap_or(create_rw_signal(false));

  let posts = create_resource(
    move || (page(), authenticated()),
    move |(page, _authenticated)| async move {
      let form = GetPosts {
        type_: None,
        sort: None,
        community_name: None,
        community_id: None,
        page: Some(page),
        limit: None,
        saved_only: None,
        disliked_only: None,
        liked_only: None,
        // moderator_view: None,
        // auth: None,
        page_cursor: None,
      };

      // cfg_if! {
      //   if #[cfg(feature = "ssr")] {
      //     use crate::{api::set_cookie_wrapper, lemmy_client::LemmyClient};
      //     use awc::Client;
      //     awc::Client::new().list_posts(&form).await.ok()
      //   } else {
      //     use crate::lemmy_client::Fetch;
      //     use crate::lemmy_client::LemmyClient;
      //     // let c = Fetch::new();
      //     // c.list_posts(&form).await.ok()
      //     let v: Vec<PostView> = vec![];
      //     Some(GetPostsResponse { next_page: None, posts: v } )
      //   }
      // }
    },
  );

  let err_msg = " Error loading this post.";

  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Home activity"</h2>
    // <Suspense fallback=|| {
    // view! { "Loading..." }
    // }>
    // {move || {
    // posts
    // .get()
    // .map(|res| match res {
    // None => {
    // view! { <div>{err_msg}</div> }
    // }
    // Some(res) => {
    // view! {
    // <div>
    // <PostListings posts=res.posts.into()/>
    // </div>
    // }
    // }
    // })
    // }}

    // </Suspense>
    </main>
  }
}
