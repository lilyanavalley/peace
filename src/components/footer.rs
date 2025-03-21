
use leptos::prelude::*;


#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <div class="text-center text-[#AAA] bg-[var(--color-deeperblack)]">
      <p class="text-xs m-1">the website of</p>
      <p class="text-xs m-3 mb-1">Lily Ana Valley</p>
      <p class="text-xs m-1 ">
        <a href="https://github.com/lilyanavalley/peace" class="text-[var(--color-selectables-pink)]">
          <i class="ph-fill ph-github-logo"/>
          Source
        </a>
        <a class="text-[#FFF]" disabled>
          <i class="ph-fill ph-asterisk"/>
          Issues
        </a>
        <a href="https://www.gnu.org/licenses/agpl-3.0.en.html" class="text-[var(--color-selectables-blue)]">
          <i class="ph-fill ph-scroll"/>
          License
        </a>
      </p>
      <img src="assets/agplv3-155x51.png" alt="GNU AGPL version 3 License Logo"/>
    </div>
  }
}
