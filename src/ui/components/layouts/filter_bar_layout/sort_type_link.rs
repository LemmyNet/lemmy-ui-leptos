use crate::utils::traits::BoolOptionStr;
use lemmy_client::lemmy_api_common::lemmy_db_schema::SortType;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_query_map};

#[component]
pub fn SortTypeLink(
  #[prop(into)] sort_type: Signal<SortType>,
  link_sort_type: SortType,
  children: Children,
) -> impl IntoView {
  let query = use_query_map();

  view! {
    <li>

      <A
        href=move || {
          let mut query = query.get();
          query.insert(String::from("sort"), link_sort_type.to_string());
          query.to_query_string()
        }

        attr:class="aria-selected:btn-active"
        attr:aria-selected=move || { (sort_type.get() == link_sort_type).then_str() }
      >

        {children()}
      </A>
    </li>
  }
}
