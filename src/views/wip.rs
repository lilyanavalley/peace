
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::placeholders;


#[component]
pub fn Wip() -> impl IntoView {
  view! {
    <div class="flex flex-col grow h-full overflow-auto justify-center items-center animate-(--animation-rainbowswirl)">
      <Icon icon=FLOWER size="3rem" weight=IconWeight::Duotone/>
      <p class="victor-mono-400 italic">{ placeholders::WIP }</p>
    </div>
  }
}
