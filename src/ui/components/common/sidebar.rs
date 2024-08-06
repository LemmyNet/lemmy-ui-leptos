use crate::{
  ui::components::common::{
    icon::{Icon, IconSize, IconType},
    unpack::Unpack,
  },
  utils::types::QuerySignal,
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::{
  aggregates::structs::{CommunityAggregates, SiteAggregates},
  source::person::Person,
};
use leptos::*;
use leptos_fluent::move_tr;
use leptos_router::A;
use si_format::Formattable;

mod site_sidebar;
pub use site_sidebar::SiteSidebar;

mod community_sidebar;

#[derive(Clone)]
enum Aggregates {
  Site(SiteAggregates),
  Community(CommunityAggregates),
}

impl Aggregates {
  fn posts(&self) -> i64 {
    match self {
      Aggregates::Site(aggregates) => aggregates.posts,
      Aggregates::Community(aggregates) => aggregates.posts,
    }
  }

  fn comments(&self) -> i64 {
    match self {
      Aggregates::Site(aggregates) => aggregates.comments,
      Aggregates::Community(aggregates) => aggregates.comments,
    }
  }

  fn users_today(&self) -> i64 {
    match self {
      Aggregates::Site(aggregates) => aggregates.users_active_day,
      Aggregates::Community(aggregates) => aggregates.users_active_day,
    }
  }

  fn users_week(&self) -> i64 {
    match self {
      Aggregates::Site(aggregates) => aggregates.users_active_week,
      Aggregates::Community(aggregates) => aggregates.users_active_week,
    }
  }

  fn users_month(&self) -> i64 {
    match self {
      Aggregates::Site(aggregates) => aggregates.users_active_month,
      Aggregates::Community(aggregates) => aggregates.users_active_month,
    }
  }

  fn users_6_months(&self) -> i64 {
    match self {
      Aggregates::Site(aggregates) => aggregates.users_active_half_year,
      Aggregates::Community(aggregates) => aggregates.users_active_half_year,
    }
  }
}

#[component]
fn UserStatRow(count: i64, text: Signal<String>) -> impl IntoView {
  view! {
    <tr class="*:p-2.5 [&:not(:first-child)]:border-t-2 [&:not(:first-child)]:border-accent">
      <td class="text-xs font-semibold">{text}</td>
      <td class="text-center font-bold">{count.si_format().to_string()}</td>
    </tr>
  }
}

#[component]
fn AdminCard(person: Person) -> impl IntoView {
  view! {
    <li class="flex-1 text-center max-w-fit rounded-lg p-3 even:bg-base-100 odd:bg-base-300 shadow-lg shadow-neutral">
      <img
        src=person
            .avatar
            .map(|p| p.to_string())
            .unwrap_or(String::from("/assets/default-avatar.png"))

        loading="lazy"
        class="mx-auto size-12"
      />
      <div class="font-medium">{person.display_name.unwrap_or_else(|| person.name.clone())}</div>
      <A href=format!("/u/{}", person.name.clone()) class="text-xs block text-primary font-light">
        {format!("@{}", person.name.clone())}
      </A>
    </li>
  }
}

#[component]
fn Sidebar(
  name: QuerySignal<String>,
  description: QuerySignal<String>,
  counts: QuerySignal<Aggregates>,
  team: QuerySignal<Vec<Person>>,
) -> impl IntoView {
  let today = move_tr!("today");
  let past_week = move_tr!("past-week");
  let past_month = move_tr!("past-month");
  let past_6_months = move_tr!("past-6-months");

  view! {
    <div class="card w-full mb-3 bg-base-200">
      <div class="card-body">
        <Transition>
          <Unpack item=name let:name>
            <h2 class="card-title">{name}</h2>
          </Unpack>
          <Unpack item=description let:description>
            <p>{description}</p>
          </Unpack>
          <section aria-labelledby="instance-stats-heading" class="my-4">
            <h3 id="instance-stats-heading" class="text-2xl font-bold mb-2">
              {move_tr!("instance-stats")}
            </h3>
            <Unpack item=counts let:counts>
              <div class="font-semibold flex flex-wrap *:m-1.5">
                <div>
                  <Icon icon=IconType::Posts size=IconSize::Large class="inline" />
                  {counts.posts().si_format().to_string()}
                  " "
                  <span class="text-sm">{move_tr!("posts")}</span>
                </div>
                <div>
                  <Icon icon=IconType::Comments size=IconSize::Large class="inline" />
                  {counts.comments().si_format().to_string()}
                  " "
                  <span class="text-sm">{move_tr!("comments")}</span>
                </div>
                {if let Aggregates::Site(counts) = counts {
                    Some(
                        view! {
                          <div>
                            <Icon icon=IconType::Communities size=IconSize::Large class="inline" />
                            {counts.communities.si_format().to_string()}
                            " "
                            <span class="text-sm">{move_tr!("communities")}</span>
                          </div>
                        },
                    )
                } else {
                    None
                }}
              </div>
              <table class="w-full mt-3 table shadow-lg">
                <caption class="text-lg font-semibold whitespace-nowrap align-middle text-start mb-2">
                  <Icon icon=IconType::Users size=IconSize::Large class="inline me-2" />
                  {move_tr!("active-users")}
                </caption>
                <thead>
                  <tr class="font-extrabold text-sm bg-base-300 *:p-3">
                    <th class="text-start" scope="col">
                      {move_tr!("time-frame")}
                    </th>
                    <th class="text-center" scope="col">
                      {move_tr!("count")}
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-base-100">
                  <UserStatRow text=today count=counts.users_today() />
                  <UserStatRow text=past_week count=counts.users_week() />
                  <UserStatRow text=past_month count=counts.users_month() />
                  <UserStatRow text=past_6_months count=counts.users_6_months() />
                  {match counts {
                      Aggregates::Site(counts) => {
                          view! {
                            <UserStatRow
                              text=move_tr!("all-time")
                              count=counts.users_active_month
                            />
                          }
                              .into_view()
                      }
                      Aggregates::Community(counts) => {
                          view! {
                            <UserStatRow
                              text=move_tr!("local-subscribers")
                              count=counts.subscribers_local
                            />
                            <UserStatRow text=move_tr!("subscribers") count=counts.subscribers />
                          }
                              .into_view()
                      }
                  }}
                </tbody>
              </table>
            </Unpack>
          </section>
          <section aria-labelledby="instances-admins-heading">
            <h3 id="instance-admins-heading" class="text-2xl font-bold mb-2">
              {move_tr!("admins")}
            </h3>
            <ul class="flex flex-wrap gap-2 my-4">
              <Unpack item=team let:team>
                <For each=move || team.clone() key=|member| member.id let:member>
                  <AdminCard person=member />
                </For>
              </Unpack>
            </ul>
          </section>
        </Transition>
      </div>
    </div>
  }
}
