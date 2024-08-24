use crate::{
  serverfns::{comments::list_comments, posts::get_post},
  ui::components::{
    common::{
      sidebar::{Sidebar, SidebarData},
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

  let sidebar_data = derive_query_signal(post_resource, |post_response| SidebarData::Community {
    name: post_response.community_view.community.name.clone(),
    title: post_response.community_view.community.title.clone(),
    icon: post_response
      .community_view
      .community
      .icon
      .as_ref()
      .map(|url| url.to_string()),
    description: post_response.community_view.community.description.clone(),
    counts: post_response.community_view.counts.clone(),
  });
  let moderators = derive_query_signal(post_resource, |post_response| {
    post_response
      .moderators
      .iter()
      .map(|moderator| moderator.moderator.clone())
      .collect()
  });

  view! {
    <div class="flex mx-auto mt-4 mb-1 sm:gap-12 h-fit sm:h-full">
      <main class="basis-full lg:basis-[65%] xl:basis-3/4 flex flex-col mx-2.5 sm:mx-0 h-fit sm:h-full">
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
      <aside class="hidden basis-[35%] xl:basis-1/4 lg:block me-8 overflow-y-auto min-h-0">
        <Sidebar data=sidebar_data team=moderators />
      </aside>
    </div>
  }
}
