use crate::{serverfns::theme::get_theme, utils::types::Theme};
use leptos::prelude::*;

pub type ThemeResource = Resource<Result<Theme, ServerFnError>>;

pub fn provide_theme_resource_context() {
  let theme_resource = Resource::new_blocking(|| (), |_| get_theme());

  provide_context(theme_resource);
}
