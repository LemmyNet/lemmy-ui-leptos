use lemmy_client::lemmy_api_common::lemmy_db_schema::SortType;
use leptos::*;
use leptos_router::{use_query_map, A};

use crate::utils::traits::BoolOptionStr;

#[component]
pub fn SortTypeLink<S>(sort_type: S, link_sort_type: SortType, children: Children) -> impl IntoView
where
  S: SignalGet<Value = SortType> + 'static,
{
  let query = use_query_map();

  view! {
    <li>

      <A
        href=move || {
            let mut query = query.get();
            query.insert(String::from("sort"), link_sort_type.to_string());
            query.to_query_string()
        }

        class="aria-selected:btn-active"
        attr:aria-selected=move || {
            (sort_type.get() == link_sort_type).then_str()
        }
      >

        {children()}
      </A>
    </li>
  }
}
