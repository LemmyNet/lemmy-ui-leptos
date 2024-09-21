use crate::ui::components::common::icon::{Icon, IconSize, IconType};
use leptos::*;
use leptos_fluent::move_tr;
use pretty_num::PrettyNumber;
use sidebar_data::SidebarData;
use team_member_card::TeamMemberCard;
use user_stat_row::UserStatRow;

pub mod sidebar_data;
mod team_member_card;
mod user_stat_row;

#[component]
pub fn Sidebar<'a>(data: &'a SidebarData) -> impl IntoView {
  let today = move_tr!("today");
  let past_week = move_tr!("past-week");
  let past_month = move_tr!("past-month");
  let past_6_months = move_tr!("past-6-months");
  let time_frame = move_tr!("time-frame");
  let all_time = move_tr!("all-time");
  let local_subscribers = move_tr!("local-subscribers");
  let subscribers = move_tr!("subscribers");

  // Extract the common elements
  let (
    heading,
    team_heading,
    team,
    name,
    description,
    posts,
    comments,
    users_today,
    users_week,
    users_month,
    users_6_months,
  ) = match data {
    SidebarData::Site(s) => (
      move_tr!("instance-stats"),
      move_tr!("admins"),
      s.admins.clone(),
      s.site.name.clone(),
      s.site.description.clone(),
      s.counts.posts,
      s.counts.comments,
      s.counts.users_active_day,
      s.counts.users_active_week,
      s.counts.users_active_month,
      s.counts.users_active_half_year,
    ),
    SidebarData::Community(c) => (
      move_tr!("community-stats"),
      move_tr!("moderators"),
      c.moderators.clone(),
      c.community.name.clone(),
      c.community.description.clone(),
      c.counts.posts,
      c.counts.comments,
      c.counts.users_active_day,
      c.counts.users_active_week,
      c.counts.users_active_month,
      c.counts.users_active_half_year,
    ),
  };

  view! {
    <div class="card w-full mb-3 bg-base-200">
      <div class="card-body">
        <h2 class="card-title">{name}</h2>
        {description.map(|description| view! { <p>{description}</p> })}
        <section aria-labelledby="instance-stats-heading" class="my-4">
          <h3 id="instance-stats-heading" class="text-2xl font-bold mb-2">
            {heading}
          </h3>
          <div class="font-semibold flex flex-wrap *:m-1.5">
            <div>
              <Icon icon=IconType::Posts size=IconSize::Large class="inline" />
              {posts.pretty_format()}
              " "
              <span class="text-sm">{move_tr!("posts")}</span>
            </div>
            <div>
              <Icon icon=IconType::Comments size=IconSize::Large class="inline" />
              {comments.pretty_format()}
              " "
              <span class="text-sm">{move_tr!("comments")}</span>
            </div>

            {if let SidebarData::Site(s) = data {
              Some(
                view! {
                  <div>
                    <Icon icon=IconType::Communities size=IconSize::Large class="inline" />
                    {s.counts.communities.pretty_format()}
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
                  {time_frame}
                </th>
                <th class="text-center" scope="col">
                  {move_tr!("count")}
                </th>
              </tr>
            </thead>
            <tbody class="bg-base-100">
              <UserStatRow text=today count=users_today />
              <UserStatRow text=past_week count=users_week />
              <UserStatRow text=past_month count=users_month />
              <UserStatRow text=past_6_months count=users_6_months />
              {match data {
                SidebarData::Site(s) => {
                  view! { <UserStatRow text=all_time count=s.counts.users /> }.into_view()
                }
                SidebarData::Community(c) => {
                  view! {
                    <UserStatRow text=local_subscribers count=c.counts.subscribers_local />
                    <UserStatRow text=subscribers count=c.counts.subscribers />
                  }
                    .into_view()
                }
              }}
            </tbody>
          </table>
        </section>
        <section aria-labelledby="instances-admins-heading">
          <h3 id="instance-admins-heading" class="text-2xl font-bold mb-2">
            {team_heading}
          </h3>
          <ul class="flex flex-wrap gap-2 my-4">
            <For each=move || team.clone() key=|member| member.id let:member>
              <TeamMemberCard person=member />
            </For>
          </ul>
        </section>
      </div>
    </div>
  }
}
