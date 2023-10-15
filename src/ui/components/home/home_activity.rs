// use actix_web::web;
use crate::ui::components::post::post_listings::PostListings;
use lemmy_api_common::post::{GetPosts, GetPostsResponse};
use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn HomeActivity() -> impl IntoView {
  let _query = use_query_map();

  let error = create_rw_signal::<Option<String>>(None);

  let authenticated = expect_context::<RwSignal<bool>>();

  let auth_resource = create_resource(
    || (),
    move |()| async move {
      match get_cookie_wrapper("jwt").await {
        Ok(Some(_jwt)) => {
          authenticated.set(true);
          leptos::logging::log!("home jwt");
          true
        }
        Ok(None) => {
          authenticated.set(false);
          leptos::logging::log!("home NONE jwt");
          false
        }
        Err(_e) => {
          authenticated.set(false);
          false
        }
      }
    },
  );


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

  let posts = create_resource(
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

      match list_posts(&form).await {
        Ok(o) => {
          next_page_cursor.set(o.next_page.clone());
          Some(o)
        }
        _ => None,
      }
    },
  );

  let err_msg = " Error loading this post.";

  view! {
    <Suspense>
    <div>{ move || auth_resource.get() }</div>
    <div>{ move || authenticated.get() }</div>
    </Suspense>

    <main class="container mx-auto">
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
                            <PostListings posts=res.posts.into() error/>
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
  }
}
