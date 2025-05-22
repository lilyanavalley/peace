
use leptos::prelude::*;
use leptos_router::components::*;
use phosphor_leptos::*;
use crate::components::*;


/// Router Bar component.
#[component]
pub fn Navigator() -> impl IntoView {
  view! {
    <nav class="navbar">
      <div class="navbar-start">
        <div class="dropdown p-0">
          <div tabindex="0" role="button" class="btn btn-sm btn-primary btn-wide md:hidden">
            <Icon icon=LIST weight=IconWeight::Duotone/>
          </div>
          <ul
            tabindex="0"
            class="dropdown-content bg-[var(--color-primary)] rounded-box shadow-2xl overflow-y-scroll p-0.5">
            <For
              each=move || ROUTERBAR_SAMPLE
              key=|route| route.route
              children=move |route: RouterBarItem| {
                view! {

                  // * menu button container
                  <li class="list-row my-0.5">
                    // * button itself
                    <a class="btn btn-lg btn-block btn-primary justify-start" href={route.route}>
                      <Icon icon={route.icon} weight=IconWeight::Duotone/>
                      {route.label}
                    </a>
                  </li>

                }
              }
            />
          </ul>
        </div>
        <a class="btn btn-ghost text-xl">"lilyvalley.dev"</a>
      </div>
      <div class="navbar-center hidden md:flex">
        <ul class="menu menu-horizontal">
          <For
            each=move || ROUTERBAR_SAMPLE
            key=|route| route.route
            children=move |route: RouterBarItem| {
              view! {

                // * menu button container
                <li class="m-0.5 grow list-row">
                  // * button itself
                  <a class="btn btn-sm lg:btn-md btn-primary md:justify-start self-center m-[0.25rem]" href={route.route}>
                    <Icon icon={route.icon} weight=IconWeight::Duotone/>
                    {route.label}
                  </a>
                </li>

              }
            }
          />
        </ul>
      </div>
      <div class="navbar-end">
        <ChooseThemeDropdown/>
      </div>
    </nav>
  }
}

const ROUTERBAR_SAMPLE: [RouterBarItem; 3] = [
  RouterBarItem {
    icon: FLOWER,
    label: "About",
    route: "/",
    colorsa: "--color-selectables-pink"
  },
  RouterBarItem {
    icon: BRIEFCASE,
    label: "Work",
    route: "/work",
    colorsa: "--color-selectables-red"
  },
  RouterBarItem {
    icon: PAPER_PLANE_TILT,
    label: "Contact",
    route: "/contact",
    colorsa: "--color-selectables-yellow"
  },
  // RouterBarItem {
  //   icon: CHATS_CIRCLE,
  //   label: "Ask",
  //   route: "/ask",
  //   colorsa: "--color-selectables-blue"
  // },
];

struct RouterBarItem {
  icon: phosphor_leptos::IconData,
  label: &'static str,
  route: &'static str,
  colorsa: &'static str
}
