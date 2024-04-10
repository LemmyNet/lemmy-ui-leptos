use crate::{
  contexts::site_resource_context::SiteResource,
  use_i18n,
  utils::derive_user_is_logged_in,
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::{
  source::{local_site::LocalSite, local_user::LocalUser},
  ListingType,
  SortType,
};
use leptos::*;
use leptos_i18n::t;
use leptos_router::{use_query_map, Outlet, A};
use serde::Deserialize;

#[component]
fn ListingTypeLink<S>(
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
    !user_is_logged_in()
      && matches!(
        link_listing_type,
        ListingType::Subscribed | ListingType::ModeratorView
      )
  });

  view! {
    <A
      href=move || {
          if disabled() {
              String::from("javascript:void(0)")
          } else {
              let mut query = query.get();
              query.insert(String::from("listingType"), link_listing_type.to_string());
              query.to_query_string()
          }
      }

      class="btn join-item aria-disabled:pointer-events-none aria-disabled:btn-disabled aria-selected:btn-active"
      attr:aria-disabled=move || if disabled() { Some("true") } else { None }
      attr:aria-selected=move || {
          if listing_type.get() == link_listing_type { Some("true") } else { None }
      }
    >

      {children()}
    </A>
  }
}

#[component]
fn SortTypeLink<S>(sort_type: S, link_sort_type: SortType, children: Children) -> impl IntoView
where
  S: SignalGet<Value = SortType> + 'static,
{
  let query = use_query_map();

  view! {
    <li
      class="aria-selected:btn-active"
      attr:aria-selected=move || if sort_type.get() == link_sort_type { Some("true") } else { None }
    >

      <A href=move || {
          let mut query = query.get();
          query.insert(String::from("sort"), link_sort_type.to_string());
          query.to_query_string()
      }>{children()}</A>
    </li>
  }
}

fn derive_link_type<T: for<'a> Deserialize<'a> + Default>(
  key: &'static str,
  get_user_default: impl Fn(&LocalUser) -> T + 'static,
  get_site_default: impl Fn(&LocalSite) -> T + 'static,
) -> Signal<T> {
  let site_resource = expect_context::<SiteResource>();
  let query = use_query_map();

  Signal::derive(move || with!(|site_resource, query| {
    let site_response = site_resource
      .as_ref()
      .and_then(|site_response| site_response.as_ref().ok());

    query
      .get(key)
      .and_then(|value| serde_json::from_str(format!(r#""{value}""#).as_str()).ok())
      .or_else(|| {
        site_response.and_then(|site_response| {
          site_response
            .my_user
            .as_ref()
            .map(|my_user| get_user_default(&my_user.local_user_view.local_user))
        })
      })
      .or_else(|| {
        site_response.map(|site_response| get_site_default(&site_response.site_view.local_site))
      })
      .unwrap_or_default()
  }))
}

#[component]
fn FilterBar(listing_type: RwSignal<ListingType>, sort_type: RwSignal<SortType>) -> impl IntoView {
  let i18n = use_i18n();
  let local_listing_type = derive_link_type(
    "listingType",
    |user| user.default_listing_type,
    |site| site.default_post_listing_type,
  );
  Effect::new(move |_| {
    listing_type.set(local_listing_type())
  });

  let local_sort_type = derive_link_type(
    "sort",
    |user| user.default_sort_type,
    |site| site.default_sort_type,
  );
  Effect::new(move |_| {
    sort_type.set(local_sort_type())
  });

  view! {
    <div class="block">
      <div class="join mr-3 hidden sm:inline-block">
        <button class="btn join-item btn-active">Posts</button>
        <button class="btn join-item btn-disabled">Comments</button>
      </div>
      <div class="join mr-3 hidden sm:inline-block">
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
      <div class="dropdown hidden sm:inline-block">
        <label tabindex="0" class="btn">
          Sort type
        </label>
        <menu tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
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
      </div>
    </div>
  }
}

#[component]
pub fn FilterBarLayout() -> impl IntoView {
  let listing_type = RwSignal::new(ListingType::default());
  let sort_type = RwSignal::new(SortType::default());

  provide_context(listing_type.read_only());
  provide_context(sort_type.read_only());

  view! {
    <Transition>
      <FilterBar listing_type=listing_type sort_type=sort_type/>
    </Transition>
    <Outlet/>
  }
}
