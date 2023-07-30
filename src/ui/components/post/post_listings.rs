use crate::ui::components::post::post_listing::PostListing;
use lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostListings(cx: Scope, posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
  view! { cx,
    <ul>
      <For
        each=posts
        key=|pv| pv.post.id
        view=move |cx, pv| {
            view! { cx,
              <li>
                <PostListing post_view=pv.into()/>
              </li>
            }
        }
      />

    </ul>
  }
}
