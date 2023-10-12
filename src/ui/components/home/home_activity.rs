// use actix_web::web;
use crate::{
  api::{api_wrapper, HttpType},
  errors::LemmyAppError,
  ui::components::post::post_listings::PostListings,
};
use lemmy_api_common::post::{GetPosts, GetPostsResponse};
use leptos::*;
use leptos_router::use_query_map;

pub async fn list_posts(form: &GetPosts) -> Result<GetPostsResponse, LemmyAppError> {
  api_wrapper::<GetPostsResponse, GetPosts>(HttpType::Get, "post/list", form).await
}

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

      list_posts(&form).await.ok()
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
    <main class="container mx-auto">
      <div class="join">
        <button class="btn join-item">"Posts"</button>
        <button class="btn join-item">"Comments"</button>
      </div>
      <div class="join">
        <button class="btn join-item">"Subscribed"</button>
        <button class="btn join-item">"Local"</button>
        <button class="btn join-item">"All"</button>
      </div>
      <div class="dropdown">
        <label tabindex="0" class="btn">
          "Sort type"
        </label>
        <ul
          tabindex="0"
          class="menu dropdown-content z-[1]"
        >
          <li>
            <a>Item 1</a>
          </li>
          <li>
            <a>Item 2</a>
          </li>
        </ul>
      </div>
      <Suspense fallback=|| {
          view! { "Loading..." }
      }>
        {move || {
            posts
                .get()
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
