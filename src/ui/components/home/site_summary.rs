use crate::{
  contexts::site_resource_context::SiteResource,
  ui::components::common::{
    icon::{Icon, IconSize, IconType},
    unpack::Unpack,
  },
  utils::derive_query_signal,
};
use leptos::*;
use leptos_router::A;
use si_format::Formattable;

#[component]
fn UserStatRow(count: i64, text: &'static str) -> impl IntoView {
  view! {
    <tr class="*:p-2.5 [&:not(:first-child)]:border-t-2 [&:not(:first-child)]:border-accent">
      <td class="text-xs font-semibold">{text}</td>
      <td class="text-center font-bold">{count.si_format().to_string()}</td>
    </tr>
  }
}

#[component]
fn AdminCard(
  #[prop(into)] avatar: MaybeProp<String>,
  name: String,
  #[prop(into)] display_name: MaybeProp<String>,
) -> impl IntoView {
  view! {
    <li class="flex-1 text-center max-w-fit border-neutral rounded-lg p-3 even:bg-base-100 odd:bg-base-300 shadow-lg">
      <A href=format!("/u/{}", name.clone())>
        <img
          src=move || { avatar.get().unwrap_or(String::from("/assets/default-avatar.png")) }

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
            <Unpack item=counts let:counts>
              <div class="font-semibold flex flex-wrap *:m-1 *:bg-base-100 *:rounded-lg *:shadow-md *:p-2.5">
                <div>
                  <Icon icon=IconType::Posts size=IconSize::Large class="inline"/>
                  {counts.posts.si_format().to_string()}
                  " "
                  <span class="text-sm">Posts</span>
                </div>
                <div>
                  <Icon icon=IconType::Comments size=IconSize::Large class="inline"/>
                  {counts.comments.si_format().to_string()}
                  " "
                  <span class="text-sm">Comments</span>
                </div>
              </div>
              <table class="w-full mt-3 table shadow-lg">
                <caption class="text-lg font-semibold whitespace-nowrap align-middle text-start mb-2">
                  <Icon icon=IconType::Users size=IconSize::Large class="inline me-2"/>
                  Active Users
                </caption>
                <thead>
                  <tr class="font-extrabold text-sm bg-base-300 *:p-3">
                    <th class="text-start" scope="col">
                      Time Frame
                    </th>
                    <th class="text-center" scope="col">
                      Count
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-base-100">
                  <UserStatRow text="Today" count=counts.users_active_day/>
                  <UserStatRow text="Past Week" count=counts.users_active_week/>
                  <UserStatRow text="Past Month" count=counts.users_active_month/>
                  <UserStatRow text="Past 6 Months" count=counts.users_active_month/>
                  <UserStatRow text="All Time" count=counts.users_active_month/>
                </tbody>
              </table>
            </Unpack>
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
