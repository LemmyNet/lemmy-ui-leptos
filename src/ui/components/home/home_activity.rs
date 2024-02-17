use crate::{
  errors::{self, LemmyAppError, LemmyAppErrorType, NoneError},
  i18n::*,
  lemmy_client::*,
  ui::components::{home::{site_summary::SiteSummary, trending::Trending}, post::post_listings::PostListings},
};
use anyhow::{Context, Error};
use lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  lemmy_db_views::structs::{PaginationCursor, PostView},
  post::{GetPosts, GetPostsResponse},
  site::GetSiteResponse,
};
use leptos::*;
use leptos_router::*;
use web_sys::{wasm_bindgen::UnwrapThrowExt, MouseEvent};

#[component]
pub fn HomeActivity(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>
) -> impl IntoView {
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

  let from_func = move || {
    if let Some(t) = query.get().get("from").cloned() {
      if !t.is_empty() {
        Some(PaginationCursor(t))
      } else {
        None
      }
    } else {
      None
    }
  };

  let ssr_prev = move || query.get().get("prev").cloned();
  // let ssr_from = move || query.get().get("from").cloned();
  let ssr_limit = move || query.get().get("limit").cloned().unwrap_or("".to_string()).parse::<i64>().ok();

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

  let ssr_posts = create_resource(
    move || (user.get(), list_func(), sort_func(), from_func(), ssr_limit()),
    move |(_user, list_type, sort_type, from, limit)| async move {
      let form = GetPosts {
        type_: list_type,
        sort: sort_type,
        community_name: None,
        community_id: None,
        page: None,
        limit: limit,
        saved_only: None,
        disliked_only: None,
        liked_only: None,
        page_cursor: from,
      };

      let result = LemmyClient.list_posts(form).await;

      match result {
        Ok(o) => Some(o),
        Err(e) => {
          error.set(Some(e));
          None
        }
      }
    },
  );

  let csr_posts = RwSignal::new(None::<Vec<PostView>>);
  let csr_paginator = RwSignal::new(None::<PaginationCursor>);
  let csr_pagesize = RwSignal::new(10);
  

  // let pages_signal = create_rw_signal(vec![posts]);

  // let mut ps = pages_signal.get();
  // ps.push(posts);

  // pages_signal.set(ps);

  // csr_posts.get().map_or_else(|| ssr_posts.get().map_or((), |s| {}), |c| {});

  // let iw = window().inner_width().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
  // let ow = window().outer_width().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
  // logging::log!(" width {} {}", iw, ow);



  #[cfg(not(feature = "ssr"))]
  {
    let iw = window().inner_width().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
    logging::log!("1 {}", iw);

    if iw >= 1536f64 {

      let limit = if iw >= 1536f64 {
        20
      } else {
        10
      };

      let mut query_params = query.get();
      query_params.insert("limit".into(), limit.to_string());
      // query_params.to_query_string()

      let navigate = leptos_router::use_navigate();
      navigate(
        &format!("{}", query_params.to_query_string()),
        Default::default(),
      );
  
      // logging::log!("2 {} ", limit);
      // create_local_resource(
      //   move || (user.get(), list_func(), sort_func(), ssr_from()),
      //   move |(_user, list_type, sort_type, from)| async move {
      //     let form = GetPosts {
      //       type_: list_type,
      //       sort: sort_type,
      //       community_name: None,
      //       community_id: None,
      //       page: None,
      //       limit: Some(limit),
      //       saved_only: None,
      //       disliked_only: None,
      //       liked_only: None,
      //       page_cursor: csr_paginator.get(),
      //     };
    
      //     let result = LemmyClient.list_posts(form).await;
    
      //     match result {
      //       Ok(mut o) => {
      //         csr_paginator.set(o.next_page);
      //         let mut p = csr_posts.get().unwrap_or(vec![]);
      //         p.append(&mut o.posts);
      //         logging::log!("count {}", p.len());
      //         csr_posts.set(Some(p));
      //       },
      //       Err(e) => {
      //         error.set(Some(e));
      //       }
      //     }
      //   },
      // );
    }
    logging::log!("3");

    if iw < 640f64 {
      logging::log!("4");

      let on_scroll = move |_| {



        // fn calc() -> Result<(bool, f64), LemmyAppError> {
        //   let h = window().inner_height()?.as_f64().n()?;// .ok_or(LemmyAppErrorType::InternalClientError)?;
        //   let o = window().page_y_offset().ok().unwrap_or(0.0);
        //   let b = f64::from(document().body().map(|b| b.offset_height()).unwrap_or(1));
      
        //   let endOfPage = h + o >= b;
      
        //   let iw = window().inner_width().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
        //   let ow = window().outer_width().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
    
        //   Ok((true, 1.0))
        // }
    
    
        let h = window().inner_height().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
        let o = window().page_y_offset().ok().unwrap_or(0.0);
        let b = f64::from(document().body().map(|b| b.offset_height()).unwrap_or(1));
    
        let endOfPage = h + o >= b;
    
        // let iw = window().inner_width().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
        // let ow = window().outer_width().ok().map(|b| b.as_f64().unwrap_or(0.0)).unwrap_or(0.0);
    
          // let endOfPage: Result<bool, _> = { Ok(window().inner_height()?.into() + window().page_y_offset()?.into() >= document().body()?.offset_height().into()) };

        logging::log!("{} {} {} {}", endOfPage, h, o, b);

        if /* iw < 640f64 && */ endOfPage {
    
          // logging::log!("scroll {} width {} {}", endOfPage, iw, ow);
    
          // let ssr_posts = 
          create_local_resource(
            move || (user.get(), list_func(), sort_func()),
            move |(_user, list_type, sort_type)| async move {
              // let f = {
              //   if let Some(t) = from.clone() {
              //     if !t.is_empty() {
              //       Some(PaginationCursor(t))
              //     } else {
              //       None
              //     }
              //   } else {
              //     None
              //   }
              // };
        
              let form = GetPosts {
                type_: list_type,
                sort: sort_type,
                community_name: None,
                community_id: None,
                page: None,
                // limit: if csr_paginator.get().is_none() { Some(40) } else { None },
                limit: None,
                saved_only: None,
                disliked_only: None,
                liked_only: None,
                page_cursor: csr_paginator.get(),
              };
        
              let result = LemmyClient.list_posts(form).await;
        
              match result {
                Ok(mut o) => {
                  logging::log!("{:#?} page {:#?} ", csr_paginator.get(), o.next_page.clone());

                  // if csr_paginator.get().is_none() {
                    csr_paginator.set(o.next_page);
                    let mut p = csr_posts.get().unwrap_or(vec![]);
                    p.append(&mut o.posts);
                    logging::log!("count {}", p.len());
                    csr_posts.set(Some(p));
                    logging::log!("{:#?} ua ", csr_paginator.get());
                    // csr_posts.set(Some(csr_posts.get().unwrap_or(vec![]).append(o.posts)));
                  // } else {
                    // csr_paginator.set(o.next_page.clone());
                  //   csr_posts.set(csr_posts.get());
                  // }
                  // Some(o)
                },
                Err(e) => {
                  error.set(Some(e));
                  // None
                }
              }
            },
          );
      
        }
      };
    
      logging::log!("5");

      window_event_listener_untyped("scroll", on_scroll);
      // need resize hook as well
    }
  }




  view! {
    <div class="w-full flex flex-col sm:flex-row flex-grow">
      <div class="sm:container sm:mx-auto">
        <div class="w-full flex flex-col sm:flex-row flex-grow">
          <main role="main" class="w-full h-full flex-grow sm:p-3">
            <div class="join mr-3 hidden sm:inline-block">
              <button class="btn join-item btn-active">"Posts"</button>
              <button class="btn join-item btn-disabled">"Comments"</button>
            </div>
            <div class="join mr-3 hidden sm:inline-block">
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
            <div class="dropdown hidden sm:inline-block">
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
            <Transition fallback=|| {}>
            {move || ssr_posts.get().unwrap_or(None).map(|p| {
              // csr_posts.set(Some(p.posts));
              // csr_paginator.set(p.next_page);


              view! {
                <div class="columns-1 2xl:columns-2 3xl:columns-3">
                // {
                //   move || format!("{:#?}", csr_posts.get())
    
                // }
                              // <table class="table">
                              //   // <Show when=move || csr_posts.get().is_some() fallback=|| view! { <span></span> }>
                              //   // <For each=move || posts.get().unwrap_or(vec![]) key=|pv| pv.post.id let:pv>
                              //   //   <PostListing post_view=pv.into()/>
                              //     // <span> </span>
                              //   // </For>
                              //   // </Show>
                              // </table>
          
                                  <PostListings posts=p.posts.into() />
                                  // <PostListings posts=csr_posts /> //.get().unwrap_or(vec![]).into() />//csr_posts.get().unwrap_or(vec![]).into() />
                            </div>
    
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
                              if let Some(n) = p.next_page.clone() {
                                  let s = ssr_prev().unwrap_or_default();
                                  let mut st = s.split(",").collect::<Vec<_>>();
                                  let f = if let Some(PaginationCursor(g)) = from_func() {
                                    g
                                    // let s = f.clone(); //}.unwrap_or(PaginationCursor(""));
                                    // st.push(&s);
                                  } else {
                                    "".to_string()
                                  };
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
    
              }

            // }) }
            // </Transition>
            // <Transition fallback=|| {
            //     view! { <div>"Loading..."</div> }
            // }>
              // <For each={move || pages_signal.get()} key={|ps| ps.into()} let:ps>
              //   <span>1</span>

              // </For>
    
              // {move || {
                // csr_posts.get(). .unwrap_or(ssr_posts.get().unwrap_or)
                
                // logging::log!("will be none {:#?}", csr_posts.get());
                // let func = move || csr_posts.get().unwrap_or(vec![]).into();
                
                // csr_posts.get().unwrap_or(ssr_posts.get().unwrap_or(None))
                // csr_posts.get()
                //   .map(|res| //match res {
                          // None => {
                          //     view! { <div>"No posts for this type of query at the moment"</div> }
                          // }
                          // Some(res) => {
                              // view! {
                              //   <div>
                              
                              //   // <PostListings posts=p.posts.into() />//csr_posts.get().unwrap_or(vec![]).into() />

                              //   </div>
                              // }
                          // )// }
                      }
                    )
              }

            //             <div class="columns-1 2xl:columns-2 3xl:columns-3">
            // // {
            // //   move || format!("{:#?}", csr_posts.get())

            // // }
            //               // <table class="table">
            //               //   // <Show when=move || csr_posts.get().is_some() fallback=|| view! { <span></span> }>
            //               //   // <For each=move || posts.get().unwrap_or(vec![]) key=|pv| pv.post.id let:pv>
            //               //   //   <PostListing post_view=pv.into()/>
            //               //     // <span> </span>
            //               //   // </For>
            //               //   // </Show>
            //               // </table>
      
            //                   <PostListings posts=ssr_posts.get().map(|p| p).into() />
            //                   // <PostListings posts=csr_posts /> //.get().unwrap_or(vec![]).into() />//csr_posts.get().unwrap_or(vec![]).into() />
            //             </div>

            //             {move || {
            //               if let Some(s) = ssr_prev() {
            //                   if !s.is_empty() {
            //                       let mut st = s.split(",").collect::<Vec<_>>();
            //                       let p = st.pop().unwrap_or("");
            //                       let mut query_params = query.get();
            //                       query_params
            //                           .insert("prev".into(), st.join(",").to_string());
            //                       query_params.insert("from".into(), p.into());
            //                       view! {
            //                         <span>
            //                           <A
            //                             href=format!("{}", query_params.to_query_string())
            //                             class="btn"
            //                           >
            //                             "Prev"
            //                           </A>
            //                         </span>
            //                       }
            //                   } else {
            //                       view! { <span></span> }
            //                   }
            //               } else {
            //                   view! { <span></span> }
            //               }
            //           }}
          
            //           {move || {
            //               if let Some(n) = p.next_page.clone() {
            //                   let s = ssr_prev().unwrap_or_default();
            //                   let mut st = s.split(",").collect::<Vec<_>>();
            //                   let f = if let Some(PaginationCursor(g)) = from_func() {
            //                     g
            //                     // let s = f.clone(); //}.unwrap_or(PaginationCursor(""));
            //                     // st.push(&s);
            //                   } else {
            //                     "".to_string()
            //                   };
            //                   st.push(&f);
            //                   let mut query_params = query.get();
            //                   query_params
            //                       .insert("prev".into(), st.join(",").to_string());
            //                   query_params.insert("from".into(), n.0);
            //                   view! {
            //                     <span>
            //                       <A
            //                         href=format!("{}", query_params.to_query_string())
            //                         class="btn"
            //                       >
            //                         "Next"
            //                       </A>
            //                     </span>
            //                   }
            //               } else {
            //                   view! { <span></span> }
            //               }
            //           }}
          
                        </Transition>

          </main>
          <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 p-4 hidden lg:block">
            // causing deserialization at the moment
            // <Trending/>
            <SiteSummary site_signal/>
          </div>
        </div>
      </div>
    </div>
  }
}
