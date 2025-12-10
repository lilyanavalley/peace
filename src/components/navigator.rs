
use leptos::prelude::*;
use leptos_router::components::*;
use phosphor_leptos::*;
use crate::components::*;


/// Router Bar component.
#[component]
pub fn Navigator() -> impl IntoView {
  view! {
    <div class="fixed bottom-0 w-full self-center z-1">
      <nav class="navbar bg-base-200/50 shadow-none backdrop-blur-sm border-t-2 border-base-300/75">
        <div class="navbar-start hidden md:block">
          <span class="font-bold text-shadow-lg">"lilyvalley.dev"</span>
        </div>
        <div class="navbar-end w-full justify-center md:justify-end">
          <ul class="menu menu-horizontal">
            <For
              each=move || ROUTERBAR_SAMPLE
              key=|route| route.route
              children=move |route: RouterBarItem| {
                view! {

                  // * menu button container
                  <li class="m-1">
                    // * button itself
                    <a class="btn btn-primary btn-ghost justify-start text-primary hover:text-primary-content
                    focus:text-primary-content" href={route.route}>
                      <Icon icon={route.icon} weight=IconWeight::Fill size="1.2rem"/>
                      {route.label}
                    </a>
                  </li>

                }
              }
            />
          </ul>
        </div>
      </nav>
    </div>
  }
}

const ROUTERBAR_SAMPLE: [RouterBarItem; 4] = [
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
    icon: PEN_NIB,
    label: "Ramblings",
    route: "/ramblings",
    colorsa: "--color-selectables-green"
  },
  RouterBarItem {
    icon: PAPER_PLANE_TILT,
    label: "Contact",
    route: "/contact",
    colorsa: "--color-selectables-yellow"
  }
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
