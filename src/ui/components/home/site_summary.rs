use crate::{
  i18n::*,
  queries::site_state_query::use_site_state,
  ui::components::common::{counts_badge::CountsBadge, unpack::Unpack},
  utils::derive_query_signal::derive_query_signal,
};
use leptos::*;
use leptos_query::QueryResult;

#[component]
pub fn SiteSummary() -> impl IntoView {
  let _i18n = use_i18n();
  let QueryResult {
    data: site_response,
    ..
  } = use_site_state().use_query(|| ());

  let site_name = derive_query_signal(site_response, |site_response| {
    site_response.site_view.site.name.clone()
  });
  let site_description = derive_query_signal(site_response, |site_response| {
    site_response
      .site_view
      .site
      .description
      .clone()
      .unwrap_or_default()
  });

  let counts = derive_query_signal(site_response, |site_response| {
    site_response.site_view.counts
  });

  let admins = derive_query_signal(site_response, |site_response| {
    site_response
      .admins
      .iter()
      .map(|admin| (admin.person.id, admin.person.name.clone()))
      .collect::<Vec<_>>()
  });

  view! {
    <div class="card w-full bg-base-300 text-base-content mb-3">
      <figure>
        <div class="card-body bg-neutral">
          <Transition fallback=|| "Loading">
            <Unpack item=site_name let:site_name>
              <h2 class="card-title text-neutral-content">{site_name}</h2>
            </Unpack>
          </Transition>
        </div>
      </figure>
      <div class="card-body">
        <Transition fallback=|| "Loading">
          <Unpack item=site_description let:site_description>
            <p>{site_description}</p>
          </Unpack>
          <Unpack item=counts let:counts>
            <p>
              <CountsBadge>{counts.users_active_day} " users / day"</CountsBadge>
              <CountsBadge>{counts.users_active_week} " users / week"</CountsBadge>
              <CountsBadge>{counts.users_active_month} " users / month"</CountsBadge>
              <CountsBadge>{counts.users_active_half_year} " users / 6 months"</CountsBadge>
              <CountsBadge>{counts.users} " users"</CountsBadge>
              <CountsBadge>{counts.communities} " communities"</CountsBadge>
              <CountsBadge>{counts.posts} " posts"</CountsBadge>
              <CountsBadge>{counts.comments} " comments"</CountsBadge>
              <CountsBadge>Modlog</CountsBadge>
            </p>

          </Unpack>

        </Transition>
        <h3 class="card-title">Admins</h3>

        <p>
          <Transition>
            <Unpack item=admins let:admins>
              <For each=move || admins.clone() key=|c| c.0 let:admin>
                <CountsBadge>{admin.1}</CountsBadge>
              </For>
            </Unpack>
          </Transition>
        </p>
      </div>
    </div>
  }
}
