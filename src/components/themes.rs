
use leptos::prelude::*;
use leptos_router::components::*;
use phosphor_leptos::*;


#[component]
pub fn ChooseThemeDropdown() -> impl IntoView {
  view! {
    <div class="dropdown p-0">
      <div tabindex="0" role="button" class="btn btn-sm btn-secondary lg:btn-md btn-wide">
        "Theme"
        <Icon icon=SUN_HORIZON weight=IconWeight::Duotone/>
      </div>
      <ul tabindex="0" class="dropdown-content bg-[var(--color-secondary)] rounded-box shadow-2xl overflow-y-scroll p-0.5 max-h-54 min-w-54 -translate-x-3/4">

          <For
            each=move || THEMES_COLLECTION
            key=|each_theme| each_theme.class
            children=move |each_theme: Theme| {
              view! {
                <li class="list-row my-0.5">
                  <input
                  type="radio"
                  name="theme-dropdown"
                  class="theme-controller btn btn-sm btn-block btn-secondary justify-start"
                  aria-label={ each_theme.label }
                  value={ each_theme.class } />
                </li>
              }
            }
          />
                
      </ul>
    </div>
  }
}

const THEMES_COLLECTION: [Theme; 36] = [
  Theme { class: "midnight", label: "🌘 Dark Midnight", dark: true },
  Theme { class: "light", label: "🌼 Light Daisy", dark: false },
  Theme { class: "dark", label: "🌼 Dark Daisy", dark: true },
  Theme { class: "cupcake", label: "🧁 Cupcake", dark: false },
  Theme { class: "bumblebee", label: "🐝 Bumblebee", dark: false },
  Theme { class: "emerald", label: "💚 Emerald", dark: false },
  Theme { class: "corporate", label: "💼 Corporate", dark: false },
  Theme { class: "synthwave", label: "🥽 Synthwave", dark: true },
  Theme { class: "retro", label: "💾 Retro", dark: false },
  Theme { class: "cyberpunk", label: "🤖 Cyberpunk", dark: false },
  Theme { class: "valentine", label: "❤️ Valentine", dark: false },
  Theme { class: "halloween", label: "🎃 Halloween", dark: true },
  Theme { class: "garden", label: "💐 Garden", dark: false },
  Theme { class: "forest", label: "🌳 Forest", dark: true },
  Theme { class: "aqua", label: "🌊 Aqua", dark: false },
  Theme { class: "lofi", label: "🔅 Lofi", dark: false },
  Theme { class: "pastel", label: "🎨 Pastel", dark: false },
  Theme { class: "fantasy", label: "🦄 Fantasy", dark: false },
  Theme { class: "wireframe", label: "🩶 Wireframe", dark: false },
  Theme { class: "black", label: "🖤 Black", dark: true },
  Theme { class: "luxury", label: "🛋️ Luxury", dark: true },
  Theme { class: "dracula", label: "🧛🏽 Dracula", dark: true },
  Theme { class: "cmyk", label: "🔵🩷🟨 CMYK", dark: false },
  Theme { class: "autumn", label: "🍂 Autumn", dark: false },
  Theme { class: "business", label: "🧑‍💼 Business", dark: true },
  Theme { class: "acid", label: "🧪 Acid", dark: false },
  Theme { class: "lemonade", label: "🍋 Lemonade", dark: false },
  Theme { class: "night", label: "🌟 Night", dark: true },
  Theme { class: "coffee", label: "☕️ Coffee", dark: true },
  Theme { class: "winter", label: "❄️ Winter", dark: false },
  Theme { class: "dim", label: "🎚️ Dim", dark: true },
  Theme { class: "nord", label: "🐟 Nord", dark: false },
  Theme { class: "sunset", label: "🌅 Sunset", dark: true },
  Theme { class: "caramellatte", label: "🥛 Caramel Latte", dark: false },
  Theme { class: "abyss", label: "✻ Abyss", dark: true },
  Theme { class: "silk", label: "👗 Silk", dark: false },
];

pub struct ThemesCollection {
  themes: Vec<Theme>,
}

pub struct Theme {
  class:    &'static str,
  label:    &'static str,
  dark:     bool
}
