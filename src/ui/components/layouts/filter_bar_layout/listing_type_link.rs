use crate::{
  contexts::site_resource_context::SiteResource,
  utils::{derive_user_is_logged_in, traits::BoolOptionStr},
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::ListingType;
use leptos::*;
use leptos_router::{use_query_map, A};

#[component]
pub fn ListingTypeLink<S>(
  listing_type: S,
  link_listing_type: ListingType,
  children: Children,
) -> impl IntoView
where
  S: SignalGet<Value = ListingType> + 'static,
{
  let query = use_query_map();
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let disabled = Signal::derive(move || {
    !user_is_logged_in.get()
      && matches!(
        link_listing_type,
        ListingType::Subscribed | ListingType::ModeratorView
      )
  });

  view! {
    <A
      href=move || {
        if disabled.get() {
          String::from("javascript:void(0)")
        } else {
          let mut query = query.get();
          query.insert(String::from("listingType"), link_listing_type.to_string());
          query.to_query_string()
        }
      }

      class="btn join-item aria-disabled:pointer-events-none aria-disabled:btn-disabled aria-selected:btn-active"
      attr:aria-disabled=move || disabled.get().then_str()
      attr:aria-selected=move || { (listing_type.get() == link_listing_type).then_str() }
    >

      {children()}
    </A>
  }
}
