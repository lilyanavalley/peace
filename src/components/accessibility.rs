
use leptos::{ prelude::*, logging::* };
use log:: { trace, debug, info, warn, error };
use phosphor_leptos::*;
use super::article::{ SectionIcon, Markdown };
use crate::placeholders;


/// 'Handlebar' -- Easy-access button to customize the site for site accessibility.
#[component]
pub fn Handlebar() -> impl IntoView {

  let (open_handlebar, set_open_handlebar) = signal(false);

  let show = move|_| {
    if open_handlebar.get() { set_open_handlebar.update(|h| *h = false); }
    else { set_open_handlebar.update(|h| *h = true); }
  };

  view! {
    // * Floating 'handlebar' button.
    <li class="accessibility m-2 grow z-11">
      <Show
        when=move|| { open_handlebar.get() == true }
        fallback=move|| view! {
            <button class="btn btn-outline btn-md lg:btn-lg btn-wide md:justify-start self-center" on:click=show>
              <Icon icon=PERSON_ARMS_SPREAD size="2rem" weight=IconWeight::Fill/>
              "Accessibility"
            </button>
          }>
          <button class="btn btn-error btn-md lg:btn-lg btn-wide md:justify-start self-center" on:click=show>
            <Icon icon=X size="2rem" weight=IconWeight::Bold/>
            "Close Menu"
          </button>
      </Show>
    </li>
    // * 'Handlebar' takeover menu.
    <Show
      when=move|| { open_handlebar.get() == true }
      fallback=|| view! { <div class="hidden opacity-0 transition-opacity"></div> }
    >
      <div class="accessibility flex justify-center fixed top-0 left-0 h-full w-full opacity-100 bg-[var(--color-base-200)] z-10">
        <div class="flex flex-col h-full items-center w-3/4">
          <h1>{ placeholders::ACCESSIBILITY }</h1>
          <article>
            <Markdown markdown={ placeholders::ACCESSIBILITY_DESCRIPTION_MD.to_string() }/>
          </article>
          <div class="w-full">

            <SectionIcon icon=PALETTE>
              
              <h2>"Themes"</h2>

              <div class="join join-vertical">
                <input
                  type="radio"
                  name="theme-buttons"
                  class="btn theme-controller join-item"
                  data-act-class="ACTIVECLASS"
                  data-set-theme=""
                  aria-label="Midnight Default"
                  value="midnight" />
                <input
                  type="radio"
                  name="theme-buttons"
                  class="btn theme-controller join-item"
                  data-act-class="ACTIVECLASS"
                  data-set-theme="morning"
                  aria-label="Morning"
                  value="morning" />
                <input
                  type="radio"
                  name="theme-buttons"
                  class="btn theme-controller join-item"
                  data-act-class="ACTIVECLASS"
                  data-set-theme="dracula"
                  aria-label="Dracula"
                  value="dracula" />
                <input
                  type="radio"
                  name="theme-buttons"
                  class="btn theme-controller join-item"
                  data-act-class="ACTIVECLASS"
                  data-set-theme="caramellatte"
                  aria-label="Caramel Latte"
                  value="caramellatte" />
                <input
                  type="radio"
                  name="theme-buttons"
                  class="btn theme-controller join-item"
                  data-act-class="ACTIVECLASS"
                  data-set-theme="abyss"
                  aria-label="Abyss"
                  value="abyss" />
                <input
                  type="radio"
                  name="theme-buttons"
                  class="btn theme-controller join-item"
                  data-act-class="ACTIVECLASS"
                  data-set-theme="coffee"
                  aria-label="Coffee"
                  value="coffee" />
                <input
                  type="radio"
                  name="theme-buttons"
                  class="btn theme-controller join-item"
                  data-act-class="ACTIVECLASS"
                  data-set-theme="garden"
                  aria-label="Garden"
                  value="garden" />
              </div>

            </SectionIcon>

          </div>
        </div>
      </div>
    </Show>
  }

}
