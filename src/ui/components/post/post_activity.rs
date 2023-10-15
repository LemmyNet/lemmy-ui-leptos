use leptos::*;

#[component]
pub fn PostActivity() -> impl IntoView {
  // let params = use_params_map();
  // let post = create_resource(
  //   move || post_id_from_params(params),
  //   move |id| async move {
  //     match id {
  //       Err(e) => Err(LemmyAppError::from(e)),
  //       Ok(id) => {
  //         let form = GetPost {
  //           id: Some(PostId(id)),
  //           comment_id: None,
  //           auth: None,
  //         };
  //         get_post(&form).await
  //       }
  //     }
  //   },
  // );

  // let comments = create_resource(
  //   move || post_id_from_params(params),
  //   move |id| async move {
  //     match id {
  //       Err(e) => Err(LemmyAppError::from(e)),
  //       Ok(id) => {
  //         let form = GetComments {
  //           post_id: Some(PostId(id)),
  //           community_id: None,
  //           type_: None,
  //           sort: None,
  //           max_depth: Some(8),
  //           page: None,
  //           limit: None,
  //           community_name: None,
  //           parent_id: None,
  //           saved_only: None,
  //           disliked_only: None,
  //           liked_only: None,
  //           auth: None,
  //         };
  //         get_comments(&form).await
  //       }
  //     }
  //   },
  // );

  view! {
    // <main class="mx-auto">
    //   <h2 class="p-6 text-4xl">"Post page"</h2>
    // // <Suspense fallback=|| {
    // // view! { "Loading..." }
    // // }>
    // // {move || {
    // // post()
    // // .map(|res| match res {
    // // Err(e) => {
    // // view! { <div>{e.to_string()}</div> }
    // // }
    // // Ok(res) => {
    // // view! {
    // // <div>
    // // <PostListing post_view=res.post_view.into()/>
    // // </div>
    // // }
    // // }
    // // })
    // // }}
    // // {move || {
    // // comments()
    // // .map(|res| match res {
    // // Err(e) => {
    // // view! { <div>{e.to_string()}</div> }
    // // }
    // // Ok(res) => {
    // // view! {
    // // <div>
    // // <CommentNodes comments=res.comments.into()/>
    // // </div>
    // // }
    // // }
    // // })
    // // }}

    // // </Suspense>
    // </main>
  }
}

// fn post_id_from_params(params: Memo<ParamsMap>) -> Result<i32, ParseIntError> {
//   params()
//     .get("id")
//     .cloned()
//     .unwrap_or_default()
//     .parse::<i32>()
// }
