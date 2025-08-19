
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::placeholders;
use crate::components::FavoriteQuotes;


#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="flex flex-col md:flex-row self-center items-center justify-around border-t-3 border-accent text-center text-sm w-full">
      <FavoriteQuotes/>
      <div class="flex flex-col justify-center items-center m-6 md:w-min-24">
        <div class="flex justify-center items-center m-2">
            <Icon icon=HAND_PEACE weight=IconWeight::Duotone size="1.8rem"/>
        </div>
        <span class="m-1">{ placeholders::FOOTER }</span>
        <span class="m-1">{ placeholders::FOOTER_NAME }</span>
        <p class="m-3 text-info text-sm">
          { format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) }
        </p>
        <span class="m-2">
          <a href="https://github.com/lilyanavalley/peace" class="btn btn-soft btn-accent btn-sm m-1">
            <Icon icon=GITHUB_LOGO weight=IconWeight::Fill size="1.2rem"/>
            { placeholders::FOOTER_SOURCE }
          </a>
          <a href="https://github.com/lilyanavalley/peace/issues" class="btn btn-soft btn-accent btn-sm m-1">
            <Icon icon=ASTERISK weight=IconWeight::Fill size="1.2rem"/>
            { placeholders::FOOTER_ISSUES }
          </a>
          <a href="https://www.gnu.org/licenses/agpl-3.0.en.html" class="btn btn-soft btn-accent btn-sm m-1">
            <Icon icon=SCROLL weight=IconWeight::Fill size="1.2rem"/>
            { placeholders::FOOTER_LICENSE }
          </a>
          <a href="https://lilyvalley.cronitorstatus.com" class="btn btn-soft btn-accent btn-sm m-1">
            <Icon icon=NETWORK_X weight=IconWeight::Fill size="1.2rem"/>
            { placeholders::FOOTER_STATUS }
          </a>
          <a href="https://canary.lilyvalley.dev" class="btn btn-soft btn-accent btn-sm m-1">
            <Icon icon=BOMB weight=IconWeight::Fill size="1.2rem"/>
            { placeholders::FOOTER_CANARY }
          </a>
        </span>
        <img class="m-2" src="assets/agplv3-155x51.png" alt="GNU AGPL version 3 License Logo"/>
      </div>
    </footer>
  }
}
