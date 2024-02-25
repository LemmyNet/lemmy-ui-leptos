use crate::{errors::LemmyAppError, i18n::*};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;

#[component]
pub fn SiteSummary(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  let _i18n = use_i18n();

  view! {
    {move || {
        site_signal
            .get()
            .map(|o| match o {
                Ok(o) => {
                    view! {
                      <div class="card w-full bg-base-300 text-base-content mb-3">
                        <figure>
                          <div class="card-body bg-neutral">
                            <h2 class="card-title text-neutral-content">{o.site_view.site.name}</h2>
                          </div>
                        </figure>
                        <div class="card-body">
                          <p>{o.site_view.site.description}</p>
                          <p>
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.users_active_day} " user / day"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.users_active_week} " users / week"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.users_active_month} " users / month"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.users_active_half_year} " users / 6 months"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.users} " users"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.communities} " Communities"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.posts} " Posts"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              {o.site_view.counts.comments} " Comments"
                            </span>
                            " "
                            <span class="badge badge-neutral inline-block whitespace-nowrap">
                              "Modlog"
                            </span>
                          </p>
                          <h3 class="card-title">"Admins"</h3>
                          <p>
                            <For
                              each=move || o.admins.clone()
                              key=|admin| admin.person.id
                              children=move |a| {
                                  view! {
                                    <span class="badge badge-neutral inline-block whitespace-nowrap">
                                      {a.person.name}
                                    </span>
                                    " "
                                  }
                              }
                            />

                          </p>
                        </div>
                      </div>
                    }
                }
                _ => {
                    view! { <div class="hidden"></div> }
                }
            })
    }}
  }
}
