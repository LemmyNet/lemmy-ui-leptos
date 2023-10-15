// use actix_web::web;
use crate::{api::api_wrapper, error::*, ui::components::post::post_listings::PostListings};
use lemmy_api_common::{
  lemmy_db_views::structs::PaginationCursor,
  post::{GetPosts, GetPostsResponse},
};
use leptos::*;
use leptos_router::use_query_map;

// pub async fn list_posts(form: &GetPosts) -> Result<GetPostsResponse, LemmyAppError> {
//   api_wrapper::<GetPostsResponse, GetPosts>(HttpType::Get, "post/list", form).await
// }

#[component]
pub fn HomeActivity() -> impl IntoView {
  // cfg_if! {
  // }

  let _query = use_query_map();

  let error = create_rw_signal::<Option<String>>(None);

  let authenticated = expect_context::<RwSignal<bool>>();

  // let auth_resource = create_resource(
  //   || (),
  //   move |()| async move {
  //     match get_cookie_wrapper("jwt").await {
  //       Ok(Some(_jwt)) => {
  //         authenticated.set(true);
  //         leptos::logging::log!("home jwt");
  //         true
  //       }
  //       Ok(None) => {
  //         authenticated.set(false);
  //         leptos::logging::log!("home NONE jwt");
  //         false
  //       }
  //       Err(_e) => {
  //         authenticated.set(false);
  //         false
  //       }
  //     }
  //   },
  // );

  // #[cfg(feature = "ssr")]
  // spawn_local(async move {
  //   match get_cookie_wrapper("jwt").await {
  //     Ok(Some(_jwt)) => {
  //       authenticated.set(true);
  //       leptos::logging::log!("HOME jwt");
  //       // true
  //     }
  //     Ok(None) => {
  //       authenticated.set(false);
  //       leptos::logging::log!("HOME jwt");
  //       // false
  //     }
  //     Err(_e) => {
  //       authenticated.set(false);
  //       // false
  //     }
  //   }
  // });

  let next_page_cursor = create_rw_signal::<Option<PaginationCursor>>(None);
  let page_cursor = create_rw_signal::<Option<PaginationCursor>>(None);

  let prev_cursor_stack = create_rw_signal::<Vec<Option<PaginationCursor>>>(vec![]);

  let refresh = create_rw_signal(true);

  let posts: Resource<(bool, bool), Option<GetPostsResponse>> = create_resource(
    move || (refresh(), authenticated()),
    move |(_refresh, _authenticated)| async move {
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
        page_cursor: page_cursor(),
      };

      let result = {
        #[cfg(not(feature = "ssr"))]
        {
          use crate::lemmy_client::*;

          let fetch = Fetch {};
          let omg = fetch.list_posts(form).await;
          Some(omg)
        }
        #[cfg(feature = "ssr")]
        {
          use crate::lemmy_client::{LemmyClient, LemmyRequest};
          use actix_session::Session;
          use actix_web::web;
          use leptos_actix::extract;

          let x = extract(
            |/* session: Session,  */ client: web::Data<awc::Client>| async move {
              // let jwt = session.get::<String>("jwt")?;
              let res = client.list_posts(form).await;
              res
            },
          )
          .await
          .ok();

          // logging::log!("{:#?}", x);
          x
        }
      };
      match result {
        Some(Ok(o)) => {
          next_page_cursor.set(o.next_page.clone());
          Some(o)
        }
        _ => None,
      }
    },
  );

  let err_msg = " Error loading this post.";

  view! {
    <div class="w-full flex flex-col sm:flex-row flex-grow overflow-hidden">
      <main role="main" class="w-full h-full flex-grow p-3 overflow-auto">
      // <main class="container mx-auto">
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
      </div> <div class="join">
        <button class="btn join-item">"Subscribed"</button>
        <button class="btn join-item">"Local"</button>
        <button class="btn join-item">"All"</button>
      </div> <div class="dropdown">
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
                            // <div class="w-3/4 inline-block align-top">
                              <PostListings posts=res.posts.into() error/>
                            // </div>
                            // <div class="w-1/4 inline-block align-top">
                            //   <div class="card shadow-xl">
                            //     <div class="card-body">
                            //       <h2 class="card-title">Card title!</h2>
                            //       <p>If a dog chews shoes whose shoes does he choose?</p>
                            //       <div class="justify-end card-actions">
                            //         <button class="btn btn-primary">Buy Now</button>
                            //       </div>
                            //     </div>
                            //   </div>
                        
                            //   <div class="card shadow-xl">
                            //     <div class="card-body">
                            //       <h2 class="card-title">Card title!</h2>
                            //       <p>If a dog chews shoes whose shoes does he choose?</p>
                            //       <div class="justify-end card-actions">
                            //         <button class="btn btn-primary">Buy Now</button>
                            //       </div>
                            //     </div>
                            //   </div>
                            // </div>
                          </div>
                        }
                    }
                })
        }}

      </Suspense>
      <button
        class="btn"
        on:click=move |_| {
            let mut p = prev_cursor_stack();
            let s = p.pop().unwrap_or(None);
            prev_cursor_stack.set(p);
            page_cursor.set(s);
            refresh.set(!refresh());
        }
      >

        "Prev"
      </button>
      <button
        class="btn"
        on:click=move |_| {
            let mut p = prev_cursor_stack();
            p.push(page_cursor());
            prev_cursor_stack.set(p);
            page_cursor.set(next_page_cursor());
            refresh.set(!refresh());
        }
      >

        "Next"
      </button>
      </main>
      <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 p-4">
          <div class="sticky top-0 p-4 bg-gray-100 rounded-xl w-full">
          </div>
          <div class="bg-gray-50 rounded-xl border my-3 w-full">
              <div class="max-w-7xl mx-auto py-8 px-4 sm:px-6 lg:py-12 lg:px-8 lg:flex lg:items-center lg:justify-between">
              </div>
          </div>
      </div>
    </div>
  }
}
