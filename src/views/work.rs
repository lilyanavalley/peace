
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::{ placeholders, components };


#[component]
pub fn Work() -> impl IntoView {
  view! {
    <article class="w-3/4 self-center" style="margin: 2rem">
      <components::article::Markdown
        markdown = { placeholders::WORK_MD.to_string() }
      />
    </article>
  }
}
