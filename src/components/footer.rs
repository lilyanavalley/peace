
use leptos::prelude::*;
use phosphor_leptos::*;


#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <div class="text-center text-[#AAA]">
      <div style="margin: .2rem">
          <Icon icon=HAND_PEACE weight=IconWeight::Duotone size="1.8rem"/>
      </div>
      <p class="text-xs" style="margin-bottom: .2rem">"the home of"</p>
      <p class="text-xs" style="margin: .2rem">"Lily Ana Valley"</p>
      <p class="text-xs" style="margin: 1rem">
        { format!("peacelily v{}", env!("CARGO_PKG_VERSION")) }
      </p>
      <p class="text-xs" style="margin: .2rem">
        <a href="https://github.com/lilyanavalley/peace" class="text-[var(--color-selectables-pink)]">
          <Icon icon=GITHUB_LOGO/>
          Source
        </a>
        <a href="https://github.com/lilyanavalley/peace/issues" class="text-[#FFF]">
          <Icon icon=ASTERISK/>
          Issues
        </a>
        <a href="https://www.gnu.org/licenses/agpl-3.0.en.html" class="text-[var(--color-selectables-blue)]">
          <Icon icon=SCROLL/>
          License
        </a>
      </p>
      <img src="assets/agplv3-155x51.png" alt="GNU AGPL version 3 License Logo" style="margin: 1rem"/>
    </div>
  }
}
