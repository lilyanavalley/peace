
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::{ *, article::Markdown };


#[derive(Serialize, Deserialize)]
pub struct StatsSnapshot {

  /// Total requests served since website start.
  site_hits:  u64,

}

#[component]
pub fn Stats() -> impl IntoView {
  let snapshot = StatsSnapshot { site_hits: 1 };
  let hits = snapshot.site_hits.to_string();
  let seedling = "40GB";
  view! {

    <h1>Fun Statistics</h1>
    
    <article class="w-3/4 self-center" style="margin: 2rem">
      <Markdown
        markdown = format!(
          "This website has served `{hits}` total requests since its inception.

          My HomeLab has seeded a total of {seedling} in [FOSS Torrents](https://fosstorrents.com/).
          
          // TODO: add more stats :)
          ")
      >
      </Markdown>
    </article>

  }
}
