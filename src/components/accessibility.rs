
use leptos::{ prelude::*, logging::* };
use phosphor_leptos::*;
use super::article::{ SectionIcon, Markdown };
use crate::placeholders;


/// 'Handlebar' -- Easy-access button to customize the site for site accessibility.
#[component]
pub fn Handlebar() -> impl IntoView {

  let (open_handlebar, set_open_handlebar) = signal(false);

  let show = move|_| {
    if open_handlebar.get() { *set_open_handlebar.write() = false; }
    else { *set_open_handlebar.write() = true; }
  };

  view! {
    // * Floating 'handlebar' button.
    <div class="fixed z-11" style="margin: .5rem; top: 0; right: 0;">
      <Show
        when=move|| { open_handlebar.get() == true }
        fallback=move|| view! {
            <button class="rounded-full" style="margin: 0;" on:click=show>
              <Icon icon=PERSON_SIMPLE_CIRCLE size="2rem" weight=IconWeight::Bold/>
            </button>
          }
      >
        <button class="rounded-full" style="margin: 0;" on:click=show>
          <Icon icon=X_CIRCLE size="2rem" weight=IconWeight::Bold />
        </button>
      </Show>
    </div>
    // * 'Handlebar' takeover menu.
    <Show
      when=move|| { open_handlebar.get() == true }
      fallback=|| view! { <div class="hidden opacity-0 transition-opacity"></div> }
    >
      <div class="flex justify-center fixed h-full w-full opacity-100 bg-[var(--color-vantablack)] z-10">
        <div class="flex flex-col h-full items-center w-3/4">
          <h1>{ placeholders::ACCESSIBILITY }</h1>
          <article>
            <Markdown markdown={ placeholders::ACCESSIBILITY_DESCRIPTION_MD.to_string() }/>
          </article>
          <div id="accessibility" class="w-full">

            <SectionIcon icon=PALETTE>
              
              <h2>"Theme & Colors"</h2>

              <div class="w-full">
                <h3>"Tone"</h3>
                <div class="flex">
                  <button>
                    <Icon icon=MOON_STARS weight=IconWeight::Fill />
                    "Dark Mode"
                  </button>
                  <button>
                    <Icon icon=SUN weight=IconWeight::Fill />
                    "Light Mode"
                  </button>
                </div>
              </div>

              <div class="w-full">
                <h3>"Color Range"</h3>
                <div class="flex">
                  <button>
                    <Icon icon=CIRCLE_DASHED weight=IconWeight::Fill />
                    "Input Contrast Only"
                  </button>
                  <button disabled>
                    <Icon icon=CIRCLE_HALF_TILT weight=IconWeight::Fill />
                    "Maximum Contrast"
                  </button>
                  <button>
                    <Icon icon=BOULES weight=IconWeight::Fill />
                    "Reduced Contrast"
                  </button>
                </div>
              </div>

            </SectionIcon>

          </div>
        </div>
      </div>
    </Show>
  }

}
