use leptos::{component, view, IntoView};
use leptos_icons::{ChIcon::*, Icon};
use leptos_router::*;

#[component]
pub fn TopNav() -> impl IntoView {
  view! {
    <div class="navbar bg-base-300">
      <div class="navbar-start">
        <div class="dropdown">
          <label tabindex="0" class="btn btn-ghost btn-circle">
            <Icon icon=Icon::from(ChMenuHamburger) width="1.25rem" height="1.25rem"/>
          </label>
          <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52" >
            <li>
              <a>"Homepage"</a>
            </li>
          </ul>
        </div>
      </div>
      <div class="navbar-center">
        <a class="btn btn-ghost normal-case text-xl">"Lemmy"</a>
      </div>
      <div class="navbar-end gap-3">
        <button class="btn btn-ghost btn-circle">
          <Icon icon=Icon::from(ChSearch) width="1.25rem" height="1.25rem"/>
        </button>
        <button class="btn btn-ghost btn-circle">
          <div class="indicator">
            <Icon icon=Icon::from(ChBell) width="1.25rem" height="1.25rem"/>
            <span class="badge badge-xs badge-primary indicator-item"></span>
          </div>
        </button>
        <A href="/login">"Login"</A>
      </div>
    </div>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  view! {
    <footer class="sticky bottom-0">
      <div class="btm-nav btm-nav-lg">
        <A href="/" class="active">
          // TODO put svg's here
          <span class="btm-nav-label">"Home"</span>
        </A>
        <button>
          <span class="btm-nav-label">"TODO 1"</span>
        </button>
        <button>
          <span class="btm-nav-label">"TODO 2"</span>
        </button>
      </div>
    </footer>
  }
}
