
use leptos::prelude::*;
use phosphor_leptos::*;
use comrak;


#[component]
pub fn Markdown(markdown: String) -> impl IntoView {
  let markdown = comrak::markdown_to_html(&markdown, &comrak::Options::default());
  view! {
    <div inner_html=markdown>
    </div>
  }
}

#[component]
pub fn Article(children: ChildrenFn) -> impl IntoView {
  view! {
    <article>
      { children() }
    </article>
  }
}

#[component]
pub fn ArticleIcon(children: ChildrenFn, icon: &'static IconWeightData) -> impl IntoView {
  view! {
    <article>
      <div class="flex">
        <Icon icon=icon weight=IconWeight::Fill/>
        { children() }
      </div>
    </article>
  }
}

#[component]
pub fn Section(children: ChildrenFn) -> impl IntoView {
  view! {
    <section>
      { children() }
    </section>
  }
}

#[component]
pub fn SectionIcon(children: ChildrenFn, icon: &'static IconWeightData) -> impl IntoView {
  view! {
    <section class="flex flex-col items-center justify-center" style="margin-top: 2rem; margin-bottom: 2rem;">
      <Icon icon=icon weight=IconWeight::Fill size="1.5rem"/>
      { children() }
    </section>
  }
}

#[component]
pub fn SectionReveal(children: ChildrenFn, revealer_text: String, icon: &'static IconWeightData) -> impl IntoView {
  
  let (hidden, set_hidden) = signal(true);
  let show = move |_| { 
    if hidden.get() { 
      *set_hidden.write() = false;
    } else {
      *set_hidden.write() = true;
    }
  };

  view! {
    <section class="flex flex-col items-center justify-center b-1 bg-[var(--color-deepblack)]" style="border-radius: 1rem; margin-top: 2rem; margin-bottom: 2rem; padding: 1rem;">
      
      <button on:click=show class="self-center" style="margin: .5rem">
        <Icon icon=icon size="1.5rem"/>
        { revealer_text }
      </button>

      <Show
        when=move|| { hidden.get() == false }
        fallback=move || view! {}
      >
        { children() }
      </Show>

    </section>
  }

}

#[component]
pub fn Spacer() -> impl IntoView {
  view! {
    <div style="margin-top: 2rem; margin-bottom: 4rem;"></div>
  }
}
