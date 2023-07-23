use leptos::{component, view, IntoView, Scope};
use leptos_heroicons::size_24::{
  outline::Bell,
  solid::{Bars3, MagnifyingGlass},
};
use leptos_router::*;

#[component]
pub fn TopNav(cx: Scope) -> impl IntoView {
  view! { cx,
    <div class="navbar bg-base-300">
      <div class="navbar-start">
        <div class="dropdown">
          <label tabindex="0" class="btn btn-ghost btn-circle">
            <Bars3 />
          </label>
          <ul
            tabindex="0"
            class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
          >
            <li>
              <a>"Homepage"</a>
            </li>
          </ul>
        </div>
      </div>
      <div class="navbar-center">
        <a class="btn btn-ghost normal-case text-xl">"Lemmy"</a>
      </div>
      <div class="navbar-end">
        <button class="btn btn-ghost btn-circle">
          <MagnifyingGlass />
        </button>
        <button class="btn btn-ghost btn-circle">
          <div class="indicator">
            <Bell class="w-3" />
            <span class="badge badge-xs badge-primary indicator-item"/>
          </div>
        </button>
        <A href="/login">"Login"</A>
      </div>
    </div>
  }
}

#[component]
pub fn BottomNav(cx: Scope) -> impl IntoView {
  view! { cx,
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
