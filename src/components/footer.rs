
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::placeholders;
use crate::components::FavoriteQuotes;


#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <div class="flex flex-col md:flex-row items-center justify-between text-center text-sm bg-[var(--color-base-200)]">
      <FavoriteQuotes/>
      <div class="m-6">
        <div class="m-2">
            <Icon icon=HAND_PEACE weight=IconWeight::Duotone size="1.8rem"/>
        </div>
        <span class="m-1">{ placeholders::FOOTER }</span>
        <span class="m-1">{ placeholders::FOOTER_NAME }</span>
        <p class="m-3">
          { format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) }
        </p>
        <span class="m-2">
          <a href="https://github.com/lilyanavalley/peace" class="btn btn-soft btn-accent btn-sm m-1">
            // <Icon icon=GITHUB_LOGO/>
            { placeholders::FOOTER_SOURCE }
          </a>
          <a href="https://github.com/lilyanavalley/peace/issues" class="btn btn-soft btn-accent btn-sm m-1">
            // <Icon icon=ASTERISK/>
            { placeholders::FOOTER_ISSUES }
          </a>
          <a href="https://www.gnu.org/licenses/agpl-3.0.en.html" class="btn btn-soft btn-accent btn-sm m-1">
            // <Icon icon=SCROLL/>
            { placeholders::FOOTER_LICENSE }
          </a>
        </span>
        <img class="m-2" src="assets/agplv3-155x51.png" alt="GNU AGPL version 3 License Logo"/>
      </div>
    </div>
  }
}
