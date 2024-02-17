use crate::ui::components::post::post_listing::PostListing;
use lemmy_api_common::lemmy_db_views::structs::PostView;
use leptos::*;

#[component]
pub fn PostListings(posts: MaybeSignal<Vec<PostView>>) -> impl IntoView {
  view! {
    <table class="table">
      // <Show when=move || posts.get().is_some() fallback=|| view! { <span></span> }>
      <For each=move || posts.get()/* .unwrap_or(vec![]) */ key=|pv| pv.post.id let:pv>
        <PostListing post_view=pv.into()/>
        // <span> </span>
      </For>
      // </Show>
    </table>
  }
}
