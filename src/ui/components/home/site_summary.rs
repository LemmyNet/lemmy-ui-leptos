use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::{
    counts_badge::CountsBadge,
    icon::{Icon, IconSize, IconType},
    unpack::Unpack,
  },
  utils::{derive_query_signal, format_number_si},
};
use leptos::*;

#[component]
pub fn SiteSummary() -> impl IntoView {
  let site_resource = expect_context::<SiteResource>();

  let site_name = derive_query_signal(site_resource, |site_response| {
    site_response.site_view.site.name.clone()
  });

  let site_description = derive_query_signal(site_resource, |site_response| {
    site_response
      .site_view
      .site
      .description
      .clone()
      .unwrap_or_default()
  });

  let counts = derive_query_signal(site_resource, |site_response| {
    site_response.site_view.counts
  });

  let admins = derive_query_signal(site_resource, |site_response| {
    site_response
      .admins
      .iter()
      .map(|admin| (admin.person.id, admin.person.name.clone()))
      .collect::<Vec<_>>()
  });

  view! {
    <div class="card w-full mb-3 bg-base-200">
      <div class="card-body">
        <Transition>
          <Unpack item=site_name let:site_name>
            <h2 class="card-title">{site_name}</h2>
          </Unpack>
          <Unpack item=site_description let:site_description>
            <p>{site_description}</p>
          </Unpack>
          <section aria-labelledby="instance-stats-heading">
            <h3 id="instance-stats-heading" class="text-2xl font-bold">
              Instance Stats
            </h3>
            <Unpack item=counts let:counts>
              <section aria-labelledby="users-stats-heading">
                <div class="flex items-center gap-x-2.5">
                  <Icon icon=IconType::Users size=IconSize::Large/>
                  <h4 id="users-stats-heading" class="text-xl font-semibold">
                    Users
                  </h4>
                </div>
                <div class="flex flex-wrap gap-2">
                  <section aria-labelledby="users-active-today" class="text-center">
                    {format_number_si(counts.users_active_day)}
                    <div id="users-active-today" class="text-xs font-semibold">Active Today</div>
                  </section>
                  <section aria-labelledby="users-active-week" class="text-center">
                    {format_number_si(counts.users_active_week)}
                    <div id="users-active-week" class="text-xs font-semibold">Active This Week</div>
                  </section>
                  <section aria-labelledby="users-active-month" class="text-center">
                    {format_number_si(counts.users_active_month)}
                    <div id="users-active-month" class="text-xs font-semibold">Active This Month</div>
                  </section>
                  <section aria-labelledby="users-active-6-months" class="text-center">
                    {format_number_si(counts.users_active_half_year)}
                    <div id="users-active-6-months" class="text-xs font-semibold">Active Past 6 Months</div>
                  </section>
                  <section aria-labelledby="total-users" class="text-center">
                    {format_number_si(counts.users_active_day)}
                    <div id="total-users" class="text-xs font-semibold">Total Users</div>
                  </section>
                </div>
              </section>
              <p>
                <CountsBadge>{counts.communities} " communities"</CountsBadge>
                <CountsBadge>{counts.posts} " posts"</CountsBadge>
                <CountsBadge>{counts.comments} " comments"</CountsBadge>
                <CountsBadge>Modlog</CountsBadge>
              </p>

            </Unpack>
          </section>

          <h3 class="card-title">Admins</h3>
          <p>
            <Unpack item=admins let:admins>
              <For each=move || admins.clone() key=|c| c.0 let:admin>
                <CountsBadge>{admin.1}</CountsBadge>
              </For>
            </Unpack>
          </p>
        </Transition>
      </div>
    </div>
  }
}
