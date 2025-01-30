use crate::ui::components::common::icon::{Icon, IconSize, IconType};
use lemmy_client::lemmy_api_common::lemmy_db_schema::{ListingType, SortType};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use listing_type_link::ListingTypeLink;
use sort_type_link::SortTypeLink;

mod listing_type_link;
mod sort_type_link;

#[component]
pub fn FilterBar() -> impl IntoView {
  view! {
    <div class="mb-4 flex flex-wrap gap-3">
      <div class="join">
        <button class="btn join-item btn-active">{move_tr!("posts")}</button>
        <button class="btn join-item btn-disabled">{move_tr!("comments")}</button>
      </div>
      <div class="join">
        <ListingTypeLink link_listing_type=ListingType::Subscribed text=move_tr!("subscribed") />
        <ListingTypeLink link_listing_type=ListingType::Local text=move_tr!("local") />
        <ListingTypeLink link_listing_type=ListingType::All text=move_tr!("all") />
      </div>
      <details class="dropdown dropdown-end group">
        <summary class="btn">
          <span class="text-nowrap leading-loose">
            {move_tr!("sort-type")}" "
            <Icon
              class="align-bottom inline group-open:rotate-180 transition-transform"
              icon=IconType::DropdownCaret
              size=IconSize::Small
            />
          </span>
        </summary>
        <menu class="*:p-0 p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box">
          <SortTypeLink link_sort_type=SortType::Active text=move_tr!("active") />
          <SortTypeLink link_sort_type=SortType::Hot text=move_tr!("hot") />
          <SortTypeLink link_sort_type=SortType::New text=move_tr!("new") />
        </menu>
      </details>
    </div>
  }
}
