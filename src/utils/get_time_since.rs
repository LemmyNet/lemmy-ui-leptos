use chrono::{DateTime, Utc};
use leptos_fluent::tr;

pub fn get_time_since(date_time: &DateTime<Utc>) -> String {
  let now = Utc::now();

  let years = now.years_since(*date_time).unwrap_or_default();
  if years > 0 {
    return tr!("years-ago", { "years" => years });
  }

  let delta = now - date_time;

  let weeks = delta.num_weeks();
  let months = weeks / 4;

  if months > 0 {
    return tr!("months-ago", { "months" => months });
  } else if weeks > 0 {
    return tr!("weeks-ago", { "weeks" => weeks });
  }

  let days = delta.num_days();

  if days > 0 {
    return tr!("days-ago", { "days" => days });
  }

  let hours = delta.num_hours();
  if hours > 0 {
    return tr!("hours-ago", { "hours" => hours });
  }

  let minutes = delta.num_minutes();
  if minutes > 0 {
    return tr!("minutes-ago", { "minutes" => minutes });
  }

  let seconds = delta.num_seconds();
  if seconds > 0 {
    return tr!("seconds-ago", { "seconds" => seconds });
  }

  let now = tr!("now");
  now
}
