
use leptos::prelude::*;
use phosphor_leptos::*;


#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <div class="text-center text-[#AAA]">
      <p class="text-xs m-1">the website of</p>
      <p class="text-xs m-3 mb-1">Lily Ana Valley</p>
      <p class="text-xs m-1 ">
        <a href="https://github.com/lilyanavalley/peace" class="text-[var(--color-selectables-pink)]">
          <div style="translateY(.2rem)"><Icon icon=GITHUB_LOGO/></div>
          Source
        </a>
        <a href="https://github.com/lilyanavalley/peace/issues" class="text-[#FFF]">
          <div style="translateY(.2rem)"><Icon icon=ASTERISK/></div>
          Issues
        </a>
        <a href="https://www.gnu.org/licenses/agpl-3.0.en.html" class="text-[var(--color-selectables-blue)]">
          <div style="translateY(.2rem)"><Icon icon=SCROLL/></div>
          License
        </a>
      </p>
      <img src="assets/agplv3-155x51.png" alt="GNU AGPL version 3 License Logo" style="margin: 2rem"/>
    </div>
  }
}
