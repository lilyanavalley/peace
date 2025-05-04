
use leptos::prelude::*;
use leptos_router::components::*;
use phosphor_leptos::*;


/// Router Bar component.
#[component]
pub fn Navigator() -> impl IntoView {
  view! {
    <nav id="navigator" class="flex bg-[var(--color-deeperblack)]">
      <For
        each=move || ROUTERBAR_SAMPLE
        key=|route| route.route
        children=move |route: RouterBarItem| {
          view! {
            <A
              // class="flex flex-col flex-auto font-normal items-center justify-center without-link-symbol"
              href={route.route}
            >
              <p style="margin: .5rem; margin-bottom: .1rem;">
                <Icon icon=route.icon size="2rem" weight=IconWeight::Duotone/>
              </p>
              <p style="margin: .5rem; margin-top: 0;">{route.label}</p>
            </A>
          }
        }
      />
    </nav>
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
