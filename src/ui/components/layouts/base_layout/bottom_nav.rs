use crate::{
  contexts::site_resource_context::SiteResource,
  i18n::*,
  ui::components::common::{icon::{Icon, IconType}, unpack::Unpack},
  utils::derive_query_signal,
};
use leptos::*;
use leptos_router::*;

// #[component]
// pub fn BottomNav() -> impl IntoView {
//   let i18n = use_i18n();
//   const FE_VERSION: &str = env!("CARGO_PKG_VERSION");

//   view! {
//     <footer class="container navbar mx-auto hidden sm:flex mt-auto justify-self-end">
//       <div class="navbar-start w-auto"></div>
//       <div class="navbar-end grow w-auto">
//         <ul class="menu menu-horizontal flex-nowrap items-center">
//           <li>
//             <a href="//github.com/LemmyNet/lemmy-ui-leptos/releases" class="text-md">
//               "FE: "
//               {FE_VERSION}
//             </a>
//           </li>
//           <li>
//             <Transition>
//               <BackendVersion/>
//             </Transition>
//           </li>
//           <li>
//             <A href="/modlog" class="text-md">
//               {t!(i18n, modlog)}
//             </A>
//           </li>
//           <li>
//             <A href="/instances" class="text-md">
//               {t!(i18n, instances)}
//             </A>
//           </li>
//           <li>
//             <a href="//join-lemmy.org/docs/en/index.html" class="text-md">
//               {t!(i18n, docs)}
//             </a>
//           </li>
//           <li>
//             <a href="//github.com/LemmyNet" class="text-md">
//               {t!(i18n, code)}
//             </a>
//           </li>
//           <li>
//             <a href="//join-lemmy.org" class="text-md">
//               "join-lemmy.org"
//             </a>
//           </li>
//         </ul>
//       </div>
//     </footer>
//   }
// }

// #[component]
// fn BackendVersion() -> impl IntoView {
//   let site_resource = expect_context::<SiteResource>();
//   let version = derive_query_signal(site_resource, |res| format!("BE: {}", res.version));

//   view! {
//     <Unpack item=version let:version>
//       <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
//         {version}
//       </a>
//     </Unpack>
//   }
// }

#[component]
pub fn MobileNav() -> impl IntoView {
  view! {
    <nav aria-label="Mobile nav" class="btm-nav w-full md:hidden border-t border-neutral">
      <A href="/search"><Icon icon=IconType::Search/></A>
      <A href="/communities"><Icon icon=IconType::Communities/></A>
      <A href="/"><Icon icon=IconType::Home/></A>
      <A href="/saved"><Icon icon=IconType::Saved/></A>
      <A href="/"><Icon icon=IconType::Profile/></A>
    </nav>
  }
}