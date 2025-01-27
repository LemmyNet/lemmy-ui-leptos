use crate::{
  serverfns::{comments::list_comments, posts::get_post},
  ui::components::{
    common::sidebar::{
      sidebar_data::{CommunitySidebarData, SidebarData},
      Sidebar,
    },
    post::post_listing::PostListing,
  },
};
use lemmy_client::lemmy_api_common::{
  comment::GetComments, lemmy_db_schema::newtypes::PostId, post::GetPost,
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn PostPage() -> impl IntoView {
  let params = use_params_map();

  let post_id = Signal::derive(move || {
    params
      .read()
      .get("id")
      .and_then(|post_id| Some(PostId(post_id.as_str().parse().ok()?)))
      .unwrap_or_default()
  });

  let post_resource = Resource::new_blocking(
    move || GetPost {
      id: Some(*post_id.read()),
      comment_id: None,
    },
    get_post,
  );

  let _list_comments_resource = Resource::new(
    move || GetComments {
      post_id: Some(*post_id.read()),
      max_depth: Some(8),
      ..Default::default()
    },
    list_comments,
  );

  view! {
    <div class="max-w-screen-2xl mx-auto flex gap-6 flex mt-4 mb-1 sm:gap-12 h-fit">
      <main class="basis-full lg:basis-13/20 xl:basis-7/10 flex flex-col mx-2.5 lg:mx-0 h-fit">
        <Transition>
            {move || Suspend::new(async move {
              post_resource.await.map(|post_response| view! {
                <PostListing post_view=post_response.post_view />
              })
            })}
        </Transition>

      // <Unpack item=list_comments_resource let:res>
      // <div>
      // <CommentNodes comments=res.comments.clone()/>
      // </div>
      // </Unpack>
      </main>
      <aside class="hidden basis-7/20 xl:basis-3/10 lg:block me-8 sticky top-6 h-fit">
        <Transition>
            {move || Suspend::new(async move {
              post_resource.await.map(|post_response| {
                let data = SidebarData::Community(CommunitySidebarData {
                  community: post_response.community_view.community,
                  counts: post_response.community_view.counts,
                  moderators: post_response.moderators.into_iter().map(|m| m.moderator).collect()
                });

                view! {
                  <Sidebar data=data />
                }
              })
            })}
        </Transition>
      </aside>
    </div>
  }
}
