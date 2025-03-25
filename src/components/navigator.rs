
use leptos::prelude::*;
use phosphor_leptos::*;


/// Router Bar component.
#[component]
pub fn Navigator() -> impl IntoView {
  view! {
    <div id="navigator" class="flex">
      <For
        each=move || ROUTERBAR_SAMPLE
        key=|route| route.route
        children=move |route: RouterBarItem| {
          view! {
            <a
              class=move || format!("flex flex-col flex-auto font-normal text-center justify-center without-link-symbol {}", route.colorsa)
              href={route.route}
            >
              // <i style="margin: .5rem; margin-bottom: .1rem" class=move || format!("text-[30px] {}", route.icon)/>
              <Icon icon=route.icon weight=IconWeight::Duotone/>
              <p style="margin: 0; margin-bottom: .5rem">{route.label}</p>
            </a>
          }
        }
      />
    </div>
  }
}

const ROUTERBAR_SAMPLE: [RouterBarItem; 4] = [
  RouterBarItem {
    icon: FLOWER_TULIP,
    // icon: "ph ph-flower",
    label: "About",
    route: "/",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-purple"
  },
  RouterBarItem {
    icon: BRIEFCASE,
    // icon: "ph ph-flower",
    label: "Work",
    route: "/work",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-violet"
  },
  RouterBarItem {
    icon: PAPER_PLANE_TILT,
    // icon: "ph ph-flower",
    label: "Contact",
    route: "/contact",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-purple"
  },
  RouterBarItem {
    icon: CHATS_CIRCLE,
    // icon: "ph ph-flower",
    label: "Ask",
    route: "/ama",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-blue"
  },
];

struct RouterBarItem {
  icon: phosphor_leptos::IconData,
  // icon: &'static str,
  label: &'static str,
  route: &'static str,
  colortw: &'static str,
  colorsa: &'static str
}
