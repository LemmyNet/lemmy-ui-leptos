use lemmy_client::lemmy_api_common::lemmy_db_schema::aggregates::structs::{
  CommunityAggregates,
  SiteAggregates,
};

#[derive(Clone)]
pub enum SidebarData {
  Site {
    name: String,
    description: Option<String>,
    counts: SiteAggregates,
  },
  Community {
    name: String,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    icon: Option<String>,
    description: Option<String>,
    counts: CommunityAggregates,
  },
}

impl SidebarData {
  pub fn name(&self) -> String {
    match self {
      Self::Site { name, .. } => name,
      Self::Community { name, .. } => name,
    }
    .into()
  }

  pub fn description(&self) -> Option<String> {
    match self {
      Self::Site { description, .. } => description,
      Self::Community { description, .. } => description,
    }
    .as_ref()
    .map(Into::into)
  }

  pub fn posts(&self) -> i64 {
    match self {
      Self::Site { counts, .. } => counts.posts,
      Self::Community { counts, .. } => counts.posts,
    }
  }

  pub fn comments(&self) -> i64 {
    match self {
      Self::Site { counts, .. } => counts.comments,
      Self::Community { counts, .. } => counts.comments,
    }
  }

  pub fn users_today(&self) -> i64 {
    match self {
      Self::Site { counts, .. } => counts.users_active_day,
      Self::Community { counts, .. } => counts.users_active_day,
    }
  }

  pub fn users_week(&self) -> i64 {
    match self {
      Self::Site { counts, .. } => counts.users_active_week,
      Self::Community { counts, .. } => counts.users_active_week,
    }
  }

  pub fn users_month(&self) -> i64 {
    match self {
      Self::Site { counts, .. } => counts.users_active_month,
      Self::Community { counts, .. } => counts.users_active_month,
    }
  }

  pub fn users_6_months(&self) -> i64 {
    match self {
      Self::Site { counts, .. } => counts.users_active_half_year,
      Self::Community { counts, .. } => counts.users_active_half_year,
    }
  }
}
