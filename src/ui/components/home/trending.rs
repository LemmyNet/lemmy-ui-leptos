use crate::{errors::LemmyAppError, i18n::*, lemmy_client::*};
use lemmy_api_common::{
  community::*,
  lemmy_db_schema::{ListingType, SortType},
  lemmy_db_views_actor::structs::CommunityView,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Trending() -> impl IntoView {
  let _i18n = use_i18n();

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();

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

      let result = LemmyClient.list_communities(form).await;

      match result {
        Ok(o) => Some(o),
        Err(e) => {
          error.set(Some(e));
          None
        }
      }
    },
  );

  view! {
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
                              <h2 class="card-title text-info-content">"Trending Communities"</h2>
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
  }
}
