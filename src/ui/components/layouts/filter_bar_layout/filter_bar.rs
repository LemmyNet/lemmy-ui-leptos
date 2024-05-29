use crate::{
  ui::components::{
    common::icon::{Icon, IconSize, IconType},
    layouts::filter_bar_layout::{
      derive_link_type,
      listing_type_link::ListingTypeLink,
      sort_type_link::SortTypeLink,
    },
  },
  use_i18n,
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::{ListingType, SortType};
use leptos::*;
use leptos_i18n::t;

#[component]
pub fn FilterBar(
  listing_type: RwSignal<ListingType>,
  sort_type: RwSignal<SortType>,
) -> impl IntoView {
  let i18n = use_i18n();
  let local_listing_type = derive_link_type(
    "listingType",
    |user| user.default_listing_type,
    |site| site.default_post_listing_type,
  );
  Effect::new(move |_| listing_type.set(local_listing_type.get()));

  let local_sort_type = derive_link_type(
    "sort",
    |user| user.default_sort_type,
    |site| site.default_sort_type,
  );
  Effect::new(move |_| sort_type.set(local_sort_type.get()));

  view! {
    <div class="mb-4 flex flex-wrap gap-3">
      <div class="join">
        <button class="btn join-item btn-active">Posts</button>
        <button class="btn join-item btn-disabled">Comments</button>
      </div>
      <div class="join">
        <ListingTypeLink listing_type=listing_type link_listing_type=ListingType::Subscribed>
          Subscribed
        </ListingTypeLink>
        <ListingTypeLink listing_type=listing_type link_listing_type=ListingType::Local>
          Local
        </ListingTypeLink>
        <ListingTypeLink listing_type=listing_type link_listing_type=ListingType::All>
          All
        </ListingTypeLink>
      </div>
      <details class="dropdown dropdown-end group">
        <summary class="btn">
          <span class="text-nowrap leading-loose">
            "Sort type "
            <Icon
              class="align-bottom inline group-open:rotate-180 transition-transform"
              icon=IconType::DropdownCaret
              size=IconSize::Small
            />
          </span>

        </summary>
        <menu class="*:p-0 p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box">
          <SortTypeLink sort_type=sort_type link_sort_type=SortType::Active>
            {t!(i18n, active)}
          </SortTypeLink>
          <SortTypeLink sort_type=sort_type link_sort_type=SortType::Hot>
            {t!(i18n, hot)}
          </SortTypeLink>
          <SortTypeLink sort_type=sort_type link_sort_type=SortType::New>
            {t!(i18n, new)}
          </SortTypeLink>
        </menu>
      </details>
    </div>
  }
}
