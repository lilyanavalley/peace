
use leptos::prelude::*;
use phosphor_leptos::*;


#[component]
pub fn Work() -> impl IntoView {
  view! {
    <h1 class="text-center">Work</h1>

    <div class="flex flex-col grow h-full overflow-auto self-center w-3/4">
      
      <a class="flex items-center border-x-2 rounded-lg border-[var(--color-selectables-green)] hover:border-[var(--color-brightwhite)] without-link-symbol" href="https://github.com/lilyanavalley/">
        <img src="favicon.ico" alt="sample project name icon"/>
        <div class="dosis-400" style="margin: .5rem">Sample Project Name</div>
      </a>

    </div>
  }
}
