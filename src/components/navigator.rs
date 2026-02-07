
use leptos::prelude::*;
use leptos_router::components::*;
use phosphor_leptos::*;
use crate::components::*;


/// Router Bar component.
#[component]
pub fn Navigator() -> impl IntoView {
  view! {
    <div class="fab fixed bottom-0 right-0 p-4 z-10">
      <div tabindex="0" role="button" class="btn btn-outline btn-accent btn-circle btn-lg bg-base-200/68 backdrop-blur-xs ring-3 ring-base-200 shadow-lg shadow-accent/25 cursor-pointer">
        <Icon icon=FLOWER weight=IconWeight::Fill size="1.25rem" />
      </div>
      <For
      each=move || ROUTERBAR_SAMPLE
      key=|route| route.route
      children=move |route: RouterBarItem| {
        view! {

          <a class="btn btn-outline btn-accent bg-base-200/68 backdrop-blur-xs ring-3 ring-base-200  shadow-md shadow-accent/25 cursor-pointer my-1" href={route.route}>
            <Icon icon={route.icon} weight=IconWeight::Fill size="1.2rem"/>
            <span>{route.label}</span>
          </a>
      
        }
      }
      />
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
