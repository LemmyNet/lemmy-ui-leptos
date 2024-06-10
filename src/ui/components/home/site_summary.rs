use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::{
    icon::{Icon, IconSize, IconType},
    unpack::Unpack,
  },
  utils::{derive_query_signal, format_number_si},
};
use leptos::*;
use leptos_router::A;

#[component]
fn StatCard(count: i64, text: &'static str, icon: IconType) -> impl IntoView {
  view! {
    <li class="flex-1 text-center max-w-fit border border-neutral rounded-lg p-2 even:bg-base-100 odd:bg-base-300">
      <Icon icon=icon size=IconSize::Large class="mx-auto"/>
      {format_number_si(count)}
      <div class="text-xs font-semibold text-balance">{text}</div>
    </li>
  }
}

#[component]
fn AdminCard(
  #[prop(into)] avatar: MaybeProp<String>,
  name: String,
  #[prop(into)] display_name: MaybeProp<String>,
) -> impl IntoView {
  view! {
    <li class="flex-1 text-center max-w-fit border border-neutral rounded-lg p-2 even:bg-base-100 odd:bg-base-300">
      <A href=format!(
          "/u/{}",
          name.clone(),
      )>
      <img
      src=move || {
          avatar.get().map(|avatar| avatar).unwrap_or(String::from("/assets/default-avatar.png"))
      }

      loading="lazy"
      class="mx-auto size-12"
    />
        <div class="font-medium">{display_name.get().unwrap_or_else(|| name.clone())}</div>
        <div class="text-xs">{format!("@{}", name.clone())}</div>
      </A>
    </li>
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
      .map(|admin| admin.person.clone())
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
          <section aria-labelledby="instance-stats-heading" class="my-4">
            <h3 id="instance-stats-heading" class="text-2xl font-bold mb-2">
              Instance Stats
            </h3>
            <ul class="flex flex-wrap gap-2 my-4">
              <Unpack item=counts let:counts>
                <StatCard
                  count=counts.users_active_day
                  text="Users Active Today"
                  icon=IconType::Users
                />
                <StatCard
                  count=counts.users_active_week
                  text="Users Active This Week"
                  icon=IconType::Users
                />
                <StatCard
                  count=counts.users_active_month
                  text="Users Active This Month"
                  icon=IconType::Users
                />
                <StatCard
                  count=counts.users_active_half_year
                  text="Users Active Past 6 Months"
                  icon=IconType::Users
                />
                <StatCard count=counts.users text="Total Users" icon=IconType::Users/>
                <StatCard count=counts.communities text="Communities" icon=IconType::Communities/>
                <StatCard count=counts.posts text="Posts" icon=IconType::Posts/>
                <StatCard count=counts.comments text="Comments" icon=IconType::Comments/>
              </Unpack>
            </ul>
          </section>
          <section aria-labelledby="instances-admins-heading">
            <h3 id="instance-admins-heading" class="text-2xl font-bold mb-2">
              Admins
            </h3>
            <ul class="flex flex-wrap gap-2 my-4">
              <Unpack item=admins let:admins>
                <For each=move || admins.clone() key=|admin| admin.id let:admin>
                  <AdminCard
                    avatar=admin.avatar.map(|url| url.to_string())
                    name=admin.name
                    display_name=admin.display_name
                  />
                </For>
              </Unpack>
            </ul>
          </section>
        </Transition>
      </div>
    </div>
  }
}
