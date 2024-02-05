// use actix_web::web;
use crate::ui::components::post::post_listings::PostListings;
use lemmy_api_common::{lemmy_db_views::structs::PaginationCursor, post::GetPosts};
use leptos::*;

#[component]
pub fn HomeActivity() -> impl IntoView {
  let error = create_rw_signal::<Option<String>>(None);

  let page_cursor = create_rw_signal::<Option<PaginationCursor>>(None);
  let cursor_string = create_rw_signal::<Option<String>>(None);

  let prev_cursor_stack = create_rw_signal::<Vec<Option<PaginationCursor>>>(vec![]);

  // let posts = create_resource(
  //   move || cursor_string.get(),
  //   move |_cursor_string| async move {
  //     let form = GetPosts {
  //       type_: None,
  //       sort: None,
  //       community_name: None,
  //       community_id: None,
  //       page: None,
  //       limit: None,
  //       saved_only: None,
  //       disliked_only: None,
  //       liked_only: None,
  //       page_cursor: page_cursor.get(),
  //     };

  //     let result = {
  //       #[cfg(not(feature = "ssr"))]
  //       {
  //         use crate::lemmy_client::*;
  //         Some((Fetch {}).list_posts(form).await)
  //       }
  //       #[cfg(feature = "ssr")]
  //       {
  //         use crate::lemmy_client::LemmyClient;
  //         use actix_web::web;
  //         use leptos_actix::extract;

  //         let client = extract::<web::Data<awc::Client>>()
  //           .await
  //           .map_err(Into::into)?;

  //         client.list_posts(form).await
  //       }
  //     };

  //     result
  //   },
  // );

  view! {
    <div class="w-full flex flex-col sm:flex-row flex-grow overflow-hidden">
      <main role="main" class="w-full h-full flex-grow p-3 overflow-auto">
        {move || {
            error
                .get()
                .map(|err| {
                    view! {
                      <div class="alert alert-error">
                        <span>{err}</span>
                      </div>
                    }
                })
        }}

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
        // <Suspense fallback=|| {
        //     view! { "Loading..." }
        // }>
        //   {move || {
        //       posts
        //           .get()
        //           .map(|res| match res {
        //               None => {
        //                   view! { <div></div> }
        //               }
        //               Some(res) => {
        //                   view! {
        //                     <div>
        //                       <PostListings posts=res.posts.into() error/>

        //                       <button
        //                         class="btn"
        //                         on:click=move |_| {
        //                             let mut p = prev_cursor_stack.get();
        //                             let s = p.pop().unwrap_or(None);
        //                             prev_cursor_stack.set(p);
        //                             page_cursor.set(s.clone());
        //                             cursor_string.set(Some(format!("{:#?}", s)));
        //                         }
        //                       >

        //                         "Prev"
        //                       </button>
        //                       <button
        //                         class="btn"
        //                         on:click=move |_| {
        //                             let mut p = prev_cursor_stack.get();
        //                             p.push(page_cursor.get());
        //                             prev_cursor_stack.set(p);
        //                             page_cursor.set(res.next_page.clone());
        //                             cursor_string
        //                                 .set(Some(format!("{:#?}", res.next_page.clone())));
        //                         }
        //                       >

        //                         "Next"
        //                       </button>

        //                     </div>
        //                   }
        //               }
        //           })
        //   }}

        // </Suspense>
      </main>
      <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 p-4">
        <div class="sticky top-0 p-4 bg-gray-100 rounded-xl w-full"></div>
        <div class="bg-gray-50 rounded-xl border my-3 w-full">
          <div class="max-w-7xl mx-auto py-8 px-4 sm:px-6 lg:py-12 lg:px-8 lg:flex lg:items-center lg:justify-between"></div>
        </div>
      </div>
    </div>
  }
}
