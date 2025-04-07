
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::{ components, placeholders };


/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    
    <div id="profile" class="flex flex-col self-center text-center border-0 rounded-full" style="margin-top: 2rem">
      <img src="assets/avatar.png" class="self-center w-1/4 rounded-full" alt="Profile picture of Lily"/>
      <h2 class="self-center justify-self-center">Lily Ana Valley</h2>
    </div>

    <div id="profile-badges" class="w-3/4 self-center">
      <p class="text-center">{ placeholders::DESCRIPTION_SHORT.to_string() }</p>
    </div>

    <article class="w-3/4 self-center text-justify" style="margin: 2rem">
      <components::article::Markdown
        markdown = { placeholders::DESCRIPTION_LONG.to_string() }
      />
    </article>

  }
}

