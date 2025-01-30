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
        <ListingTypeLink link_listing_type=ListingType::Subscribed>
          {move_tr!("subscribed")}
        </ListingTypeLink>
        <ListingTypeLink link_listing_type=ListingType::Local>
          {move_tr!("local")}
        </ListingTypeLink>
        <ListingTypeLink link_listing_type=ListingType::All>
          {move_tr!("all")}
        </ListingTypeLink>
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
          <SortTypeLink link_sort_type=SortType::Active>
            {move_tr!("active")}
          </SortTypeLink>
          <SortTypeLink link_sort_type=SortType::Hot>
            {move_tr!("hot")}
          </SortTypeLink>
          <SortTypeLink link_sort_type=SortType::New>
            {move_tr!("new")}
          </SortTypeLink>
        </menu>
      </details>
    </div>
  }
}
