use crate::{
  cookie::*,
  errors::{message_from_error, LemmyAppError, LemmyAppErrorType},
  i18n::*,
  lemmy_client::*,
  ui::components::post::post_listings::PostListings,
};
use lemmy_api_common::{
  community::*,
  lemmy_db_schema::{source::person::Person, ListingType, SortType},
  lemmy_db_views::structs::PaginationCursor,
  lemmy_db_views_actor::structs::CommunityView,
  post::{GetPosts, GetPostsResponse},
  site::GetSiteResponse,
};
use leptos::*;
use leptos_router::*;
use web_sys::*;

// impl From<PaginationCursor> for String {
//   fn from(value: PaginationCursor) -> Self {
//     Self::APIError {
//       error: value.to_string(),
//     }
//   }
// }

// impl fmt::Display for PaginationCursor {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "{}", self.)
//   }
// }

#[component]
pub fn HomeActivity() -> impl IntoView {
  let i18n = use_i18n();

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();

  let site_data = expect_context::<RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>>();
  let data = create_resource(
    move || (),
    move |()| async move { LemmyClient.get_site().await },
  );
  site_data.set(data.get());

  let page_cursor = create_rw_signal::<Option<PaginationCursor>>(None);
  let cursor_string = create_rw_signal::<Option<String>>(None);
  let prev_cursor_stack = create_rw_signal::<Vec<Option<PaginationCursor>>>(vec![]);

  let list_signal = create_rw_signal::<Option<ListingType>>(None);
  let sort_signal = create_rw_signal::<Option<SortType>>(None);

  let query = use_query_map();

  // let _list = create_rw_signal::<Option<ListingType>>(None);
  let ssr_list = move || query.with(|params| params.get("list").cloned());
  // let _sort = create_rw_signal::<Option<SortType>>(None);
  let ssr_sort = move || query.with(|params| params.get("sort").cloned());

  if let Some(t) = ssr_list() {
    let r = serde_json::from_str::<ListingType>(&t[..]);

    match r {
      Ok(o) => {
        list_signal.set(Some(o));
      }
      Err(_e) => {
        logging::log!("error decoding error - log and ignore in UI?");
      }
    }
  }

  let on_list_click = move |lt: ListingType| {
    move |_me: MouseEvent| {
      let r = serde_json::to_string::<ListingType>(&lt);

      match r {
        Ok(o) => {
          let navigate = leptos_router::use_navigate();
          navigate(
            &format!("/?list={}&sort={}", o, ssr_sort().unwrap_or("".to_string()))[..],
            Default::default(),
          );
        }
        Err(e) => {
          error.set(Some(e.into()));
        }
      }
    }
  };

  if let Some(s) = ssr_sort() {
    let r = serde_json::from_str::<SortType>(&s[..]);

    match r {
      Ok(o) => {
        sort_signal.set(Some(o));
      }
      Err(e) => {
        error.set(Some(e.into()));
      }
    }
  }

  let on_sort_click = move |lt: SortType| {
    move |_me: MouseEvent| {
      let r = serde_json::to_string::<SortType>(&lt);

      match r {
        Ok(o) => {
          let navigate = leptos_router::use_navigate();
          navigate(
            &format!("/?list={}&sort={}", ssr_list().unwrap_or("".to_string()), o)[..],
            Default::default(),
          );
        }
        Err(e) => {
          error.set(Some(e.into()));
        }
      }
    }
  };

  let posts = create_resource(
    move || (cursor_string.get(), ssr_list(), ssr_sort()),
    move |(_cursor_string, list, sort)| async move {
      let l = {
        if let Some(t) = list.clone() {
          if !t.is_empty() {
            let r = serde_json::from_str::<ListingType>(&t[..]);

            match r {
              Ok(o) => {
                list_signal.set(Some(o));
                Some(o)
              }
              Err(e) => {
                error.set(Some(e.into()));
                None
              }
            }
          } else {
            None
          }
        } else {
          None
        }
      };

      let s = {
        if let Some(t) = sort.clone() {
          if !t.is_empty() {
            let r = serde_json::from_str::<SortType>(&t[..]);

            match r {
              Ok(o) => {
                sort_signal.set(Some(o));
                Some(o)
              }
              Err(e) => {
                error.set(Some(e.into()));
                None
              }
            }
          } else {
            None
          }
        } else {
          None
        }
      };

      let form = GetPosts {
        type_: l,
        sort: s,
        community_name: None,
        community_id: None,
        page: None,
        limit: None,
        saved_only: None,
        disliked_only: None,
        liked_only: None,
        page_cursor: page_cursor.get(),
      };

      let result: Option<Result<GetPostsResponse, LemmyAppError>> = {
        use crate::lemmy_client::*;
        Some(LemmyClient.list_posts(form).await)
      };

      match result {
        Some(Ok(o)) => Some(o),
        Some(Err(e)) => {
          leptos::logging::log!("Err {:#?}", e);
          error.set(Some(e));
          None
        }
        None => {
          leptos::logging::log!("Nun");
          error.set(Some(LemmyAppError {
            error_type: LemmyAppErrorType::Unknown,
            content: String::default(),
          }));
          None
        }
      }
    },
  );

  let trending = create_resource(
    move || (),
    move |()| async move {
      let form = ListCommunities {
        type_: Some(ListingType::Local),
        sort: Some(SortType::Hot),
        limit: Some(6),
        show_nsfw: None,
        page: None,
      };

      let result: Option<Result<ListCommunitiesResponse, LemmyAppError>> =
        { Some(LemmyClient.list_communities(form).await) };

      match result {
        Some(Ok(o)) => Some(o),
        Some(Err(e)) => {
          leptos::logging::log!("Err {:#?}", e);
          error.set(Some(e));
          None
        }
        None => {
          leptos::logging::log!("Nun");
          error.set(Some(LemmyAppError {
            error_type: LemmyAppErrorType::Unknown,
            content: String::default(),
          }));
          None
        }
      }
    },
  );

  view! {
    <div class="w-full flex flex-col sm:flex-row flex-grow overflow-hidden">
      <div class="container mx-auto overflow-auto">
        <div class="w-full flex flex-col sm:flex-row flex-grow">
          <Transition fallback=|| {
              view! { <div>"Loading..."</div> }
          }>
            <main role="main" class="w-full h-full flex-grow p-3">
              <div class="join mr-3">
                <button class="btn join-item">"Posts"</button>
                <button class="btn join-item">"Comments"</button>
              </div>
              <div class="join mr-3">
                <A
                  href=format!(
                      "/?list={}&sort={}",
                      "\"Subscribed\"",
                      if Some(SortType::Active) == sort_signal.get() { "\"Active\"" } else { "" },
                  )

                  class=move || {
                      format!(
                          "btn join-item {}",
                          if Some(ListingType::Subscribed) == list_signal.get() {
                              "btn-active"
                          } else {
                              ""
                          },
                      )
                  }

                  on:click=on_list_click(ListingType::Subscribed)
                >
                  "Subscribed"
                </A>
                <A
                  href=format!(
                      "/?list={}&sort={}",
                      "\"Local\"",
                      if Some(SortType::Active) == sort_signal.get() { "\"Hot\"" } else { "" },
                  )

                  class=move || {
                      format!(
                          "btn join-item {}",
                          if Some(ListingType::Local) == list_signal.get() {
                              "btn-active"
                          } else {
                              ""
                          },
                      )
                  }

                  on:click=on_list_click(ListingType::Local)
                >
                  "Local"
                </A>
                <A
                  href=format!(
                      "/?list={}&sort={}",
                      "\"All\"",
                      if Some(SortType::Active) == sort_signal.get() { "\"New\"" } else { "" },
                  )

                  class=move || {
                      format!(
                          "btn join-item {}",
                          if Some(ListingType::All) == list_signal.get() {
                              "btn-active"
                          } else {
                              ""
                          },
                      )
                  }

                  on:click=on_list_click(ListingType::All)
                >
                  "All"
                </A>
              </div>
              <div class="dropdown">
                <label tabindex="0" class="btn">
                  "Sort type"
                </label>
                <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
                  <li
                    class=move || {
                        (if Some(SortType::Active) == sort_signal.get() {
                            "btn-active"
                        } else {
                            ""
                        })
                            .to_string()
                    }

                    on:click=on_sort_click(SortType::Active)
                  >
                    <span>{t!(i18n, active)}</span>
                  </li>
                  <li
                    class=move || {
                        (if Some(SortType::Hot) == sort_signal.get() { "btn-active" } else { "" })
                            .to_string()
                    }

                    on:click=on_sort_click(SortType::Hot)
                  >
                    <span>{t!(i18n, hot)}</span>
                  </li>
                  <li
                    class=move || {
                        (if Some(SortType::New) == sort_signal.get() { "btn-active" } else { "" })
                            .to_string()
                    }

                    on:click=on_sort_click(SortType::New)
                  >
                    <span>{t!(i18n, new)}</span>
                  </li>
                </ul>
              </div>
              {move || {
                  posts
                      .get()
                      .map(|res| match res {
                          None => {
                              view! { <div>"No posts for this type of query at the moment"</div> }
                          }
                          Some(res) => {
                              view! {
                                <div>
                                  <PostListings posts=res.posts.into()/>
                                  <button
                                    class="btn"
                                    on:click=move |_| {
                                        let mut p = prev_cursor_stack.get();
                                        let s = p.pop().unwrap_or(None);
                                        prev_cursor_stack.set(p);
                                        page_cursor.set(s.clone());
                                        cursor_string.set(Some(format!("{:#?}", s)));
                                    }
                                  >

                                    "Prev"
                                  </button>
                                  <button
                                    class="btn"
                                    on:click=move |_| {
                                        let mut p = prev_cursor_stack.get();
                                        p.push(page_cursor.get());
                                        prev_cursor_stack.set(p);
                                        page_cursor.set(res.next_page.clone());
                                        cursor_string
                                            .set(Some(format!("{:#?}", res.next_page.clone())));
                                    }
                                  >

                                    "Next"
                                  </button>
                                </div>
                              }
                          }
                      })
              }}

            </main>
          </Transition>
          <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 p-4">
            <Transition fallback=|| {
                view! { "Loading..." }
            }>
              {move || {
                  trending
                      .get()
                      .map(|r| match r {
                          None => {
                              view! { <div class="hidden"></div> }
                          }
                          Some(c) => {
                              let c_signal = create_rw_signal(c.communities);
                              view! {
                                <div class="card w-full bg-base-300 text-base-content mb-3">
                                  <figure>
                                    <div class="card-body bg-info">
                                      <h2 class="card-title text-info-content">
                                        "Trending Communities"
                                      </h2>
                                    </div>
                                  </figure>
                                  <div class="card-body">
                                    <p>"Description"</p>
                                    <p>
                                      <For
                                        each=move || c_signal.get()
                                        key=|community| community.community.id
                                        children=move |cv: CommunityView| {
                                            view! {
                                              <span class="badge badge-neutral inline-block whitespace-nowrap">
                                                {cv.community.title}
                                              </span>
                                              " "
                                            }
                                        }
                                      />

                                    </p>
                                  </div>
                                </div>
                              }
                          }
                      })
              }}

            </Transition>
            <div class="card w-full bg-base-300 text-base-content mb-3">
              {move || {
                  site_data
                      .get()
                      .map(|m| match m {
                          Ok(o) => {
                              view! {
                                <div>
                                  <figure>
                                    <div class="card-body bg-neutral">
                                      <h2 class="card-title text-neutral-content">
                                        {o.site_view.site.name}
                                      </h2>
                                    </div>
                                  </figure>
                                  <div class="card-body">
                                    <p>{o.site_view.site.description}</p>
                                    <p>
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.users_active_day} " user / day"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.users_active_week} " users / week"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.users_active_month} " users / month"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.users_active_half_year}
                                        " users / 6 months"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.users} " users"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.communities} " Communities"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.posts} " Posts"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        {o.site_view.counts.comments} " Comments"
                                      </span>
                                      " "
                                      <span class="badge badge-neutral inline-block whitespace-nowrap">
                                        "Modlog"
                                      </span>
                                    </p>
                                    <h3 class="card-title">"Admins"</h3>
                                    <p>
                                      <For
                                        each=move || o.admins.clone()
                                        key=|admin| admin.person.id
                                        children=move |a| {
                                            view! {
                                              <span class="badge badge-neutral inline-block whitespace-nowrap">
                                                {a.person.name}
                                              </span>
                                              " "
                                            }
                                        }
                                      />

                                    </p>
                                  </div>
                                </div>
                              }
                          }
                          _ => {
                              view! { <div class="hidden"></div> }
                          }
                      })
              }}

            </div>
          </div>
        </div>
      </div>
    </div>
  }
}
