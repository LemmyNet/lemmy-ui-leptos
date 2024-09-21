use crate::{
  serverfns::{comments::list_comments, posts::get_post},
  ui::components::{
    common::{
      sidebar::{
        sidebar_data::{CommunitySidebarData, SidebarData},
        Sidebar,
      },
      unpack::Unpack,
    },
    post::post_listing::PostListing,
  },
  utils::derive_query_signal,
};
use lemmy_client::lemmy_api_common::{
  comment::GetComments,
  lemmy_db_schema::newtypes::PostId,
  post::GetPost,
};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn PostPage() -> impl IntoView {
  let params = use_params_map();

  let post_id = Signal::derive(move || {
    with!(|params| params
      .get("id")
      .and_then(|post_id| Some(PostId(post_id.as_str().parse().ok()?)))
      .unwrap_or_default())
  });

  let post_resource = create_blocking_resource(
    move || {
      with!(|post_id| GetPost {
        id: Some(*post_id),
        comment_id: None
      })
    },
    get_post,
  );

  let _list_comments_resource = create_resource(
    move || {
      with!(|post_id| GetComments {
        post_id: Some(*post_id),
        max_depth: Some(8),
        ..Default::default()
      })
    },
    list_comments,
  );

  let sidebar_data = derive_query_signal(post_resource, |post_response| {
    SidebarData::Community(CommunitySidebarData {
      community: post_response.community_view.community.clone(),
      counts: post_response.community_view.counts.clone(),
      moderators: post_response
        .moderators
        .iter()
        .map(|moderator| moderator.moderator.clone())
        .collect(),
    })
  });

  view! {
    <div class="max-w-screen-2xl mx-auto flex gap-6 flex mt-4 mb-1 sm:gap-12 h-fit">
      <main class="basis-full lg:basis-13/20 xl:basis-7/10 flex flex-col mx-2.5 lg:mx-0 h-fit">
        <Transition>
          <Unpack item=post_resource let:res>
            <PostListing post_view=&res.post_view />
          </Unpack>
        </Transition>

      // <Unpack item=list_comments_resource let:res>
      // <div>
      // <CommentNodes comments=res.comments.clone()/>
      // </div>
      // </Unpack>
      </main>
      <aside class="hidden basis-7/20 xl:basis-3/10 lg:block me-8 sticky top-6 h-fit">
        <Transition>
          <Unpack item=sidebar_data let:data>
            <Sidebar data=&data />
          </Unpack>
        </Transition>
      </aside>
    </div>
  }
}
