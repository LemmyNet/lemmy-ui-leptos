use crate::{
  ui::components::common::{
    icon::{Icon, IconSize, IconType},
    unpack::Unpack,
  },
  utils::types::QuerySignal,
};
use lemmy_client::lemmy_api_common::lemmy_db_schema::source::person::Person;
use leptos::*;
use leptos_fluent::move_tr;
use si_format::Formattable;
use team_member_card::TeamMemberCard;
use user_stat_row::UserStatRow;

mod sidebar_data;
pub use sidebar_data::SidebarData;
mod team_member_card;
mod user_stat_row;

#[component]
pub fn Sidebar(data: QuerySignal<SidebarData>, team: QuerySignal<Vec<Person>>) -> impl IntoView {
  let today = move_tr!("today");
  let past_week = move_tr!("past-week");
  let past_month = move_tr!("past-month");
  let past_6_months = move_tr!("past-6-months");

  view! {
    <div class="card w-full mb-3 bg-base-200">
      <div class="card-body">
        <Transition>
          <Unpack item=data let:data>
            <h2 class="card-title">{data.name()}</h2>
            {data.description().map(|description| view! { <p>{description}</p> })}
            <section aria-labelledby="instance-stats-heading" class="my-4">
              <h3 id="instance-stats-heading" class="text-2xl font-bold mb-2">
                {if matches!(data, SidebarData::Site { .. }) {
                    move_tr!("instance-stats")
                } else {
                    move_tr!("community-stats")
                }}
              </h3>
              <div class="font-semibold flex flex-wrap *:m-1.5">
                <div>
                  <Icon icon=IconType::Posts size=IconSize::Large class="inline" />
                  {data.posts().si_format().to_string()}
                  " "
                  <span class="text-sm">{move_tr!("posts")}</span>
                </div>
                <div>
                  <Icon icon=IconType::Comments size=IconSize::Large class="inline" />
                  {data.comments().si_format().to_string()}
                  " "
                  <span class="text-sm">{move_tr!("comments")}</span>
                </div>
                {if let SidebarData::Site { ref counts, .. } = data {
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
                  <UserStatRow text=today count=data.users_today() />
                  <UserStatRow text=past_week count=data.users_week() />
                  <UserStatRow text=past_month count=data.users_month() />
                  <UserStatRow text=past_6_months count=data.users_6_months() />
                  {match data {
                      SidebarData::Site { ref counts, .. } => {
                          view! {
                            <UserStatRow
                              text=move_tr!("all-time")
                              count=counts.users_active_month
                            />
                          }
                              .into_view()
                      }
                      SidebarData::Community { ref counts, .. } => {
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
            </section>
            <section aria-labelledby="instances-admins-heading">
              <h3 id="instance-admins-heading" class="text-2xl font-bold mb-2">
                {if matches!(data, SidebarData::Site { .. }) {
                    move_tr!("admins")
                } else {
                    move_tr!("moderators")
                }}
              </h3>
              <ul class="flex flex-wrap gap-2 my-4">
                <Unpack item=team let:team>
                  <For each=move || team.clone() key=|member| member.id let:member>
                    <TeamMemberCard person=member />
                  </For>
                </Unpack>
              </ul>
            </section>
          </Unpack>
        </Transition>
      </div>
    </div>
  }
}
