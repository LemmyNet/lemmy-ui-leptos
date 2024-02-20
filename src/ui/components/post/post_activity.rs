use crate::{
  lemmy_client::*,
  ui::components::{comment::comment_nodes::CommentNodes, post::post_listing::PostListing},
};
use lemmy_api_common::{comment::GetComments, lemmy_db_schema::newtypes::PostId, post::GetPost};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn PostActivity() -> impl IntoView {
  let params = use_params_map();

  let post_id = move || params.get().get("id").cloned().unwrap_or_default();

  let post = create_resource(
    post_id,
    move |id_string| async move {
      let id = id_string.parse::<i32>()?;
      let form = GetPost {
        id: Some(PostId(id)),
        comment_id: None,
      };
      LemmyClient.get_post(form).await
    },
  );

  let comments = create_resource(
    post_id,
    move |id_string| async move {
      let id = id_string.parse::<i32>()?;
      let form = GetComments {
        post_id: Some(PostId(id)),
        community_id: None,
        type_: None,
        sort: None,
        max_depth: Some(8),
        page: None,
        limit: None,
        community_name: None,
        parent_id: None,
        saved_only: None,
        disliked_only: None,
        liked_only: None,
      };
      LemmyClient.get_comments(form).await
    },
  );

  view! {
    <main class="mx-auto">
      <h2 class="p-6 text-4xl">"Post page"</h2>
      <Transition fallback=|| {
          view! { "Loading..." }
      }>
        {move || {
            post.get()
                .map(|res| match res {
                    Err(e) => {
                        view! { <div>{e.to_string()}</div> }
                    }
                    Ok(res) => {
                        view! {
                          <div>
                            <PostListing post_view=res.post_view.into()/>
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
            comments
                .get()
                .map(|res| match res {
                    Err(e) => {
                        view! { <div>{e.to_string()}</div> }
                    }
                    Ok(res) => {
                        view! {
                          <div>
                            <CommentNodes comments=res.comments.into()/>
                          </div>
                        }
                    }
                })
        }}

      </Transition>
    </main>
  }
}
