
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::{ placeholders, components::* };


#[component]
pub fn Work() -> impl IntoView {
  view! {
    <Autoscale>
      <Article>
        <Markdown
          markdown = { placeholders::WORK_MD.to_string() }
        />
      </Article>
    </Autoscale>
  }
}
