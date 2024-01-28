use crate::{
  errors::{LemmyAppError, LemmyAppErrorType},
  i18n::*,
  lemmy_client::*,
  ui::components::post::post_listings::PostListings,
};
use lemmy_api_common::{
  community::*,
  lemmy_db_schema::{ListingType, SortType},
  lemmy_db_views::structs::PaginationCursor,
  lemmy_db_views_actor::structs::CommunityView,
  post::{GetPosts, GetPostsResponse},
  site::GetSiteResponse,
};
use leptos::*;
use leptos_router::*;
use web_sys::*;

#[component]
pub fn HomeActivity(site_signal: RwSignal<Option<GetSiteResponse>>) -> impl IntoView {
  let i18n = use_i18n();

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();
  let user = expect_context::<RwSignal<Option<bool>>>();

  let query = use_query_map();

  let list_func = move || {
    serde_json::from_str::<ListingType>(
      &query
        .get()
        .get("list")
        .cloned()
        .unwrap_or("\"Local\"".to_string()),
    )
    .ok()
  };

  let sort_func = move || {
    serde_json::from_str::<SortType>(
      &query
        .get()
        .get("sort")
        .cloned()
        .unwrap_or("\"Active\"".to_string()),
    )
    .ok()
  };

  let ssr_list = move || query.get().get("list").cloned();
  let ssr_sort = move || query.get().get("sort").cloned();
  let ssr_prev = move || query.get().get("prev").cloned();
  let ssr_from = move || query.get().get("from").cloned();

  let on_sort_click = move |lt: SortType| {
    move |_me: MouseEvent| {
      let r = serde_json::to_string::<SortType>(&lt);

      match r {
        Ok(o) => {
          let mut query_params = query.get();
          query_params.insert("sort".into(), o);

          let navigate = leptos_router::use_navigate();
          navigate(
            &format!("{}", query_params.to_query_string()),
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
    move || (user.get(), ssr_list(), ssr_sort(), ssr_from()),
    move |(_user, list, sort, from)| async move {
      let l = {
        if let Some(t) = list.clone() {
          if !t.is_empty() {
            let r = serde_json::from_str::<ListingType>(&t[..]);

            match r {
              Ok(o) => Some(o),
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
              Ok(o) => Some(o),
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

      let f = {
        if let Some(t) = from.clone() {
          if !t.is_empty() {
            Some(PaginationCursor(t))
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
        page_cursor: f,
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
          error.set(Some(e));
          None
        }
        None => {
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
          <main role="main" class="w-full h-full flex-grow p-3">
            <div class="join mr-3">
              <button class="btn join-item">"Posts"</button>
              <button class="btn join-item">"Comments"</button>
            </div>
            <div class="join mr-3">
              {move || {
                  let mut query_params = query.get();
                  query_params.insert("list".into(), "\"Subscribed\"".into());
                  view! {
                    <A
                      href=move || format!("{}", query_params.to_query_string())
                      class=move || {
                          format!(
                              "btn join-item {}",
                              if Some(ListingType::Subscribed) == list_func() {
                                  "btn-active"
                              } else {
                                  ""
                              },
                          )
                      }
                    >

                      "Subscribed"
                    </A>
                  }
              }}
              <A
                href=move || {
                    let mut query_params = query.get();
                    query_params.insert("list".into(), "\"Local\"".into());
                    query_params.to_query_string()
                }

                class=move || {
                    format!(
                        "btn join-item {}",
                        if Some(ListingType::Local) == list_func() { "btn-active" } else { "" },
                    )
                }
              >

                "Local"
              </A>
              <A
                href=move || {
                    let mut query_params = query.get();
                    query_params.insert("list".into(), "\"All\"".into());
                    query_params.to_query_string()
                }

                class=move || {
                    format!(
                        "btn join-item {}",
                        if Some(ListingType::All) == list_func() { "btn-active" } else { "" },
                    )
                }
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
                      (if Some(SortType::Active) == sort_func() { "btn-active" } else { "" })
                          .to_string()
                  }

                  on:click=on_sort_click(SortType::Active)
                >
                  <span>{t!(i18n, active)}</span>
                </li>
                <li
                  class=move || {
                      (if Some(SortType::Hot) == sort_func() { "btn-active" } else { "" })
                          .to_string()
                  }

                  on:click=on_sort_click(SortType::Hot)
                >
                  <span>{t!(i18n, hot)}</span>
                </li>
                <li
                  class=move || {
                      (if Some(SortType::New) == sort_func() { "btn-active" } else { "" })
                          .to_string()
                  }

                  on:click=on_sort_click(SortType::New)
                >
                  <span>{t!(i18n, new)}</span>
                </li>
              </ul>
            </div>
            <Transition fallback=|| {
                view! { <div>"Loading..."</div> }
            }>
              {move || {
                  site_signal
                      .get()
                      .map(|s| {
                          posts
                              .get()
                              .map(|res| match res {
                                  None => {
                                      view! {
                                        <div>"No posts for this type of query at the moment"</div>
                                      }
                                  }
                                  Some(res) => {
                                      view! {
                                        <div>
                                          <PostListings posts=res.posts.into()/>
                                          {move || {
                                              if let Some(s) = ssr_prev() {
                                                  if !s.is_empty() {
                                                      let mut st = s.split(",").collect::<Vec<_>>();
                                                      let p = st.pop().unwrap_or("");
                                                      let mut query_params = query.get();
                                                      query_params
                                                          .insert("prev".into(), st.join(",").to_string());
                                                      query_params.insert("from".into(), p.into());
                                                      view! {
                                                        <span>
                                                          <A
                                                            href=format!("{}", query_params.to_query_string())
                                                            class="btn"
                                                          >
                                                            "Prev"
                                                          </A>
                                                        </span>
                                                      }
                                                  } else {
                                                      view! { <span></span> }
                                                  }
                                              } else {
                                                  view! { <span></span> }
                                              }
                                          }}

                                          {move || {
                                              if let Some(n) = res.next_page.clone() {
                                                  let s = ssr_prev().unwrap_or_default();
                                                  let mut st = s.split(",").collect::<Vec<_>>();
                                                  let f = ssr_from().unwrap_or_default();
                                                  st.push(&f);
                                                  let mut query_params = query.get();
                                                  query_params
                                                      .insert("prev".into(), st.join(",").to_string());
                                                  query_params.insert("from".into(), n.0);
                                                  view! {
                                                    <span>
                                                      <A
                                                        href=format!("{}", query_params.to_query_string())
                                                        class="btn"
                                                      >
                                                        "Next"
                                                      </A>
                                                    </span>
                                                  }
                                              } else {
                                                  view! { <span></span> }
                                              }
                                          }}

                                        </div>
                                      }
                                  }
                              })
                      })
              }}

            </Transition>
          </main>
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
                                    <p>
                                      <For
                                        each=move || c_signal.get()
                                        key=|community| community.community.id
                                        children=move |cv: CommunityView| {
                                            view! {
                                              <A
                                                class="text-l font-bold link link-accent whitespace-nowrap"
                                                href=format!("/c/{}", cv.community.name)
                                              >
                                                {cv.community.title}
                                              </A>
                                              " "
                                            }
                                        }
                                      />

                                    </p>
                                    <A class="btn" href="/create_community">
                                      "Create a community"
                                    </A>
                                    <A class="btn" href="/communities">
                                      "Explore communities"
                                    </A>
                                  </div>
                                </div>
                              }
                          }
                      })
              }}

            </Transition>
            <Transition fallback=|| {
                view! { "Loading..." }
            }>
              {move || {
                  site_signal
                      .get()
                      .map(|o| {
                          view! {
                            <div class="card w-full bg-base-300 text-base-content mb-3">
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
                                    {o.site_view.counts.users_active_half_year} " users / 6 months"
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
                      })
              }}

            </Transition>
          </div>
        </div>
      </div>
    </div>
  }
}
