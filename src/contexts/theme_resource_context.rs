use crate::{serverfns::theme::get_theme, utils::types::Theme};
use leptos::*;

pub type ThemeResource = Resource<(), Result<Theme, ServerFnError>>;

pub fn provide_theme_resource_context() {
  let theme_resource = create_blocking_resource(|| (), |_| get_theme());

  provide_context(theme_resource);
}
