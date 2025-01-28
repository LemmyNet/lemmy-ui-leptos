use crate::ui::components::{
  common::icon::{Icon, IconSize, IconType},
  layouts::filter_bar_layout::{
    derive_link_type,
    listing_type_link::ListingTypeLink,
    sort_type_link::SortTypeLink,
  },
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::{ListingType, SortType};
use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn FilterBar(
  listing_type: RwSignal<ListingType>,
  sort_type: RwSignal<SortType>,
) -> impl IntoView {
  let local_listing_type = derive_link_type(
    "listingType",
    |user| user.default_listing_type,
    |site| site.default_post_listing_type,
  );
  // Effect::new(move |_| listing_type.set(local_listing_type.get()));

  let local_sort_type = derive_link_type(
    "sort",
    |user| user.default_sort_type,
    |site| site.default_sort_type,
  );
  // Effect::new(move |_| sort_type.set(local_sort_type.get()));

  view! {
    <div class="mb-4 flex flex-wrap gap-3">
      <div class="join">
        <button class="btn join-item btn-active">{move_tr!("posts")}</button>
        <button class="btn join-item btn-disabled">{move_tr!("comments")}</button>
      </div>
      <div class="join">
        <ListingTypeLink listing_type=listing_type link_listing_type=ListingType::Subscribed>
          {move_tr!("subscribed")}
        </ListingTypeLink>
        <ListingTypeLink listing_type=listing_type link_listing_type=ListingType::Local>
          {move_tr!("local")}
        </ListingTypeLink>
        <ListingTypeLink listing_type=listing_type link_listing_type=ListingType::All>
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
          <SortTypeLink sort_type=sort_type link_sort_type=SortType::Active>
            {move_tr!("active")}
          </SortTypeLink>
          <SortTypeLink sort_type=sort_type link_sort_type=SortType::Hot>
            {move_tr!("hot")}
          </SortTypeLink>
          <SortTypeLink sort_type=sort_type link_sort_type=SortType::New>
            {move_tr!("new")}
          </SortTypeLink>
        </menu>
      </details>
    </div>
  }
}
