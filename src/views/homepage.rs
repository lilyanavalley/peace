
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::{ components::*, placeholders };


/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
  view! {

    <Autoscale>

      <div id="profile-headshot" class="flex flex-col text-center">
        <img src="assets/avatar.png" class="self-center w-1/4 rounded-full" alt="Profile picture of Lily"/>
        <h1 class="self-center justify-self-center">"Lily Ana Valley"</h1>
      </div>

      <div id="profile-badge">
        <p class="text-center text-sm text-[var(--color-accent-content)]]">
          <Markdown
            markdown = { placeholders::DESCRIPTION_SHORT.to_string() }
          />
        </p>
      </div>

      <article id="description">
        <Markdown
          markdown = { placeholders::DESCRIPTION_LONG_MD.to_string() }
        />
      </article>

    </Autoscale>

  }
}

