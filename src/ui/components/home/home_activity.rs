use crate::{api::post::list_posts, ui::components::post::post_listings::PostListings};
use lemmy_api_common::post::GetPosts;
use leptos::*;
use leptos_router::use_query_map;

// This is helpful:
// https://github.com/leptos-rs/leptos/blob/main/examples/hackernews/src/routes/stories.rs

#[component]
pub fn HomeActivity(cx: Scope) -> impl IntoView {
  let query = use_query_map(cx);
  let page = move || {
    query
      .with(|q| q.get("page").and_then(|page| page.parse::<i64>().ok()))
      .unwrap_or(1)
  };

  let posts = create_resource(cx, page, move |page| async move {
    let form = GetPosts {
      type_: None,
      sort: None,
      community_name: None,
      community_id: None,
      page: Some(page),
      limit: None,
      saved_only: None,
      auth: None,
    };
    list_posts(cx, &form).await.ok()
  });

  let err_msg = " Error loading this post.";

  view! { cx,
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Home activity"</h2>
      <Suspense fallback=|| {
          view! { cx, "Loading..." }
      }>
        {move || {
            posts
                .read(cx)
                .map(|res| match res {
                    None => {
                        view! { cx, <div>{err_msg}</div> }
                    }
                    Some(res) => {

                        view! { cx,
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
