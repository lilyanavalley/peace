
use leptos::prelude::*;
use phosphor_leptos::*;
use comrak;
use crate::{ components::*, placeholders };


/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
  
  // options.render.hardbreaks = true;
  let description_long = || comrak::markdown_to_html(placeholders::DESCRIPTION_LONG_MD, &comrak::Options::default());
  let description_short = || comrak::markdown_to_html(placeholders::DESCRIPTION_SHORT, &comrak::Options::default());

  view! {

    <Autoscale>

      <div id="profile-headshot" class="flex flex-col lg:flex-row justify-center text-center">
        <img src="assets/avatar.png" class="self-center w-1/4 rounded-full" alt="Profile picture of Lily"/>
        <span class="flex flex-col self-center justify-self-center">
          <h1>"Lily Ana Valley"</h1>
          <div id="profile-pronouns" class="flex justify-center m-1">
            <span class="badge badge-info m-1">
              "she/her"
            </span>
            <span class="badge badge-success m-1">
              "it/its"
            </span>
          </div>
        </span>
      </div>

      // <div id="profile-tagline">
      //   <p class="text-center text-xs" inner_html=description_short()></p>
      // </div>

      <article inner_html=description_long()/>

    </Autoscale>

  }
}

