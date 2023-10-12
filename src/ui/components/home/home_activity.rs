// use actix_web::web;
use crate::{
  api::{api_wrapper, HttpType},
  errors::LemmyAppError,
  ui::components::post::post_listings::PostListings,
};
use lemmy_api_common::{
  lemmy_db_views::structs::PaginationCursor,
  post::{GetPosts, GetPostsResponse},
};
use leptos::*;
use leptos_router::use_query_map;

pub async fn list_posts(form: &GetPosts) -> Result<GetPostsResponse, LemmyAppError> {
  api_wrapper::<GetPostsResponse, GetPosts>(HttpType::Get, "post/list", form).await
}

// let page_navigate = move |page_cursor: | move |_| {
//   let navigate = leptos_router::use_navigate();
//   navigate("/", Default::default());
// };

#[component]
pub fn HomeActivity() -> impl IntoView {
  let query = use_query_map();
  let page = move || {
    query
      .with(|q| q.get("page").and_then(|page| page.parse::<String>().ok()))
      .unwrap_or("".to_string())
  };

  let authenticated = expect_context::<RwSignal<bool>>();

  let _next_page = create_rw_signal::<Option<PaginationCursor>>(None);

  let posts = create_resource(
    move || (page(), authenticated()),
    move |(_page, _authenticated)| async move {
      let form = GetPosts {
        type_: None,
        sort: None,
        community_name: None,
        community_id: None,
        page: None,
        limit: None,
        saved_only: None,
        disliked_only: None,
        liked_only: None,
        // page_cursor: Some(PaginationCursor(page)),
        page_cursor: None,
      };

      list_posts(&form).await.ok()
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
        <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
          <li>
            <span>Item 1</span>
          </li>
          <li>
            <span>Item 2</span>
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
      <button class="btn">"Prev"</button>
      <button class="btn">"Next"</button>
    </main>
  }
}
