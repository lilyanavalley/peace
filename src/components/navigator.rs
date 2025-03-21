
use leptos::prelude::*;


/// Router Bar component.
#[component]
pub fn Navigator() -> impl IntoView {
  view! {
    <div class="flex">
      <For
        each=move || ROUTERBAR_SAMPLE
        key=|route| route.route
        children=move |route: RouterBarItem| {
          view! {
            <a
              class=move || format!("flex flex-col flex-auto font-normal text-center justify-center without-link-symbol {}", route.colorsa)
              href={route.route}
            >
              <i style="margin: .5rem; margin-bottom: .1rem" class=move || format!("text-[30px] {}", route.icon)/>
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
    icon: "ph-fill ph-flower",
    label: "About",
    route: "/",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-purple"
  },
  RouterBarItem {
    icon: "ph-fill ph-briefcase",
    label: "Work",
    route: "/work",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-violet"
  },
  RouterBarItem {
    icon: "ph-fill ph-paper-plane-tilt",
    label: "Contact",
    route: "/contact",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-purple"
  },
  RouterBarItem {
    icon: "ph-fill ph-chats-circle",
    label: "Ask",
    route: "/ama",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-blue"
  },
];

struct RouterBarItem {
  icon: &'static str,
  label: &'static str,
  route: &'static str,
  colortw: &'static str,
  colorsa: &'static str
}
