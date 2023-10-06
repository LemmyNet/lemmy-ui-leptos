// use actix_web::web;
use lemmy_api_common::lemmy_db_views::structs::PostView;
use lemmy_api_common::post::{GetPosts, GetPostsResponse};
use leptos::*;
use leptos_router::use_query_map;
use cfg_if::cfg_if;
use async_trait::async_trait;


// use crate::lemmy_client::LemmyClient;
use crate::ui::components::post::post_listings::PostListings;
// use crate::{api::set_cookie_wrapper, lemmy_client::LemmyClient};


// This is helpful:
// https://github.com/leptos-rs/leptos/blob/main/examples/hackernews/src/routes/stories.rs

#[component]
pub fn HomeActivity() -> impl IntoView {
  let query = use_query_map();
  let page = move || {
    query
      .with(|q| q.get("page").and_then(|page| page.parse::<i64>().ok()))
      .unwrap_or(1)
  };

  let posts = create_resource(page, move |page| async move {
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

    // let c = Client::new();

    // LemmyClient::list_posts(&form).await.ok()
    cfg_if! {
    if #[cfg(feature = "ssr")] {
      use crate::{api::set_cookie_wrapper, lemmy_client::LemmyClient};
      use awc::Client;
    
      // let c: LemmyClient = awc::Client::new().into();

      awc::Client::new().list_posts(&form).await.ok()
    } else {
      use crate::lemmy_client::Fetch;
      use crate::lemmy_client::LemmyClient;
      // use crate::{api::set_cookie_wrapper, lemmy_client::LemmyClient};
      // use reqwest::Client;
      // let c = Fetch::new();
      // c.list_posts(&form).await.ok()
      let v: Vec<PostView> = vec![];
      Some(GetPostsResponse { next_page: None, posts: v } )
    }
    }

    // let l = LemmyClient
    // ::list_posts(&form).await.ok()
    // let v: Vec<PostView> = vec![];
    // Some(v)
  });

  let err_msg = " Error loading this post.";

  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Home activity"</h2>
      <Suspense fallback=|| {
          view! { "Loading..." }
      }>
        {move || {
            posts.get()
                .map(|res| match res {
                    None => {
                        view! { <div>{err_msg}</div> }
                    }
                    Some(res) => {
                        view! {
                          <div>
                            <PostListings posts=res.posts.into()/>
                          </div>
                        }
                    }
                })
        }}

      </Suspense>
    </main>
  }
}
