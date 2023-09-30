use leptos::*;

// This is helpful:
// https://github.com/leptos-rs/leptos/blob/main/examples/hackernews/src/routes/stories.rs

#[component]
pub fn HomeActivity() -> impl IntoView {
  // let query = use_query_map();
  // let page = move || {
  //   query
  //     .with(|q| q.get("page").and_then(|page| page.parse::<i64>().ok()))
  //     .unwrap_or(1)
  // };

  // let posts = create_resource(page, move |page| async move {
  //   let form = GetPosts {
  //     type_: None,
  //     sort: None,
  //     community_name: None,
  //     community_id: None,
  //     page: Some(page),
  //     limit: None,
  //     saved_only: None,
  //     disliked_only: None,
  //     liked_only: None,
  //     // moderator_view: None,
  //     auth: None,
  //   };
  //   list_posts(&form).await.ok()
  // });

  // let err_msg = " Error loading this post.";

  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Home activity"</h2>
    // <Suspense fallback=|| {
    // view! { "Loading..." }
    // }>
    // {move || {
    // posts()
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
