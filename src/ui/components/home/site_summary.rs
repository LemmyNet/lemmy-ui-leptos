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
fn UserCount(count: i64, id: &'static str, text: &'static str) -> impl IntoView {
  view! {
    <section aria-labelledby=id class="text-center w-16 border border-neutral rounded-lg bg-base-100 p-1.5">
      {format_number_si(count)}
      <div id=id class="text-xs font-semibold">
        {text}
      </div>
    </section>
  }
}

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
                <div class="flex items-center gap-2.5">
                  <Icon icon=IconType::Users size=IconSize::Large/>
                  <h4 id="users-stats-heading" class="text-xl font-semibold">
                    Users
                  </h4>
                </div>
                <div class="flex flex-wrap gap-2 my-4">
                  <UserCount
                    count=counts.users_active_day
                    text="Active Today"
                    id="users-active-today"
                  />
                  <UserCount
                    count=counts.users_active_week
                    text="Active This Week"
                    id="users-active-week"
                  />
                  <UserCount
                    count=counts.users_active_month
                    text="Active This Month"
                    id="users-active-month"
                  />
                  <UserCount
                    count=counts.users_active_half_year
                    text="Active Past 6 Months"
                    id="users-active-half-year"
                  />
                  <UserCount count=counts.users text="Total Users" id="users-total"/>
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
