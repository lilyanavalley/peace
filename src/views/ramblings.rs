use std::collections::BTreeMap;
use leptos::prelude::*;
use leptos_router::nested_router::Outlet;
use phosphor_leptos::*;
use serde::{Deserialize, Serialize};
use crate::{ components::*, placeholders };


pub const DB_NAME: &str = "blog";
pub const DB_COLLECTION_PRIVATE: &str = "private";
pub const DB_COLLECTION_PUBLIC: &str = "public";


#[component]
pub fn Ramblings() -> impl IntoView {
  view! {
    <Outlet/>
  }
}

#[component]
pub fn RamblingsIndex() -> impl IntoView {
  view! {
    <Autoscale>
      
      // * Introduction to the Ramlbings section.
      <Article>
        <Markdown
          markdown = { placeholders::RAMBLINGS_INDEX_MD.to_string() }
        />
        <div id="blink-caret"></div>
      </Article>

      // * Search tools
      <div class="my-4">
        <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4">
          <legend class="fieldset-legend">"Find & Filter"</legend>

          // * Search tools
          <div class="flex flex-col lg:flex-row justify-center items-center gap-4">

            // search bar
            <div class="join w-full max-w-xl">
              <div class="w-full">
                <label class="input join-item w-full">
                  <input type="search" class="grow" placeholder="Search" required/>
                  <kbd class="kbd kbd-sm font-(family-name:--font-title) font-bold">"S"</kbd>
                </label>
              </div>
              <button class="join-item btn btn-primary">
                <Icon icon=MAGNIFYING_GLASS size="1.5rem"/>
              </button>
            </div>

            // searchbar help
            <div class="collapse collapse-arrow bg-base-100 border-base-300 border rounded-lg border-2 border-info shadow-md shadow-info/15">
              <input type="checkbox" />
              <div class="collapse-title bg-info text-info-content font-(family-name:--font-title) text-lg">"Searchbar Help"</div>
              <div class="collapse-content text-sm">

                <p class="font-bold border-b border-base-300 py-3">
                  "Prefix your search with one of these options to filter results, based on your needs. Replace" <span class="text-accent italic font-bold mx-2">"[brackets]"</span> "with your search terms."
                </p>

                <p class="my-6">
                  <span class="bg-base-300 rounded p-2">
                    <span>
                      "tag:"
                    </span>
                    <span class="text-accent italic font-bold">
                      "[tag]"
                    </span>
                  </span>
                  <span>" - search by a tag"</span>
                </p>

                <p class="my-6">
                  <span class="bg-base-300 rounded p-2">
                    <span>
                      "date:"
                    </span>
                    <span class="text-accent italic font-bold">
                      "[date format]"
                    </span>
                  </span>
                  <span>" - search by " <a href="" class="link link-primary">"supported date formats"</a></span>
                </p>

                <p class="my-6">
                  <span class="bg-base-300 rounded p-2">
                    <span>
                      "!"
                    </span>
                    <span class="text-accent italic font-bold">
                      "[term]"
                    </span>
                  </span>
                  <span>" - omit results containing term or filter (ex: " <span class="italic font-bold">"!date:03.13.25"</span>" )"</span>
                </p>
                
              </div>
            </div> // * end of searchbar help

          </div> // * end of search tools
        </fieldset>
      </div>

      // * Latest posts list
      <ul class="list bg-base-100 rounded-box shadow-md">
  
        <li class="flex justify-center items-center p-4 pb-2 text-xs tracking-wide opacity-50">
          <span class="m-2">
            <Icon icon=NEWSPAPER_CLIPPING weight=IconWeight::Fill size="1.2rem"/>
          </span>
          "Latest Posts"
        </li>
        
        // * Collection of latest posts
        <li class="list-row">
          <div><img class="size-10 rounded-box" src="https://img.daisyui.com/images/profile/demo/1@94.webp"/></div>
          <div>
            <div>Dio Lupa</div>
            <div class="text-xs uppercase font-semibold opacity-60">Remaining Reason</div>
          </div>
          <p class="list-col-wrap text-xs">
            "\"Remaining Reason\" became an instant hit, praised for its haunting sound and emotional depth. A viral performance brought it widespread recognition, making it one of Dio Lupa’s most iconic tracks."
          </p>
        </li>
        
        <li class="list-row">
          <div><img class="size-10 rounded-box" src="https://img.daisyui.com/images/profile/demo/4@94.webp"/></div>
          <div>
            <div>Ellie Beilish</div>
            <div class="text-xs uppercase font-semibold opacity-60">Bears of a fever</div>
          </div>
          <p class="list-col-wrap">
            "\"Bears of a Fever\" captivated audiences with its intense energy and mysterious lyrics. Its popularity skyrocketed after fans shared it widely online, earning Ellie critical acclaim."
          </p>
          <button class="btn btn-primary">
            <Icon icon=BOOK_OPEN_TEXT weight=IconWeight::Duotone size="1.5rem"/>
            <span class="hidden md:block">"Read"</span>
          </button>
        </li>
        
      </ul>

      // * Paginator + bottom tools
      <div class="flex flex-col lg:flex-row justify-between items-center w-full my-6 gap-6">

        <div class="flex flex-col justify-center items-center gap-2">
          // RSS + scroll to top
          <div class="join">
            <div class="join-item tooltip tooltip-bottom tooltip-secondary" data-tip="copy RSS link">
              <button class="join-item btn btn-secondary">
                <Icon icon=RSS weight=IconWeight::Fill size="1.2rem"/>
                "RSS"
              </button>
            </div>
            <div class="join-item tooltip tooltip-bottom tooltip-secondary" data-tip="scroll to top">
              <button class="join-item btn btn-secondary">
                <Icon icon=ARROW_LINE_UP weight=IconWeight::Fill size="1.2rem"/>
                "Top"
              </button>
            </div>
          </div>

          // Paginator
          <div class="join">
            <button class="join-item btn btn-primary btn-disabled">
              <Icon icon=CARET_LINE_LEFT weight=IconWeight::Fill size="1.2rem"/>
            </button>
            <button class="join-item btn btn-primary btn-disabled">
              <Icon icon=CARET_LEFT weight=IconWeight::Fill size="1.2rem"/>
            </button>
            <input
              type="number"
              class="input validator w-26 md:w-36 join-item"
              placeholder="page 1"
              min="1"
              max="10"
              title="Must be between be 1 to 10"
            />
            <button class="join-item btn btn-primary">
              <Icon icon=CARET_RIGHT weight=IconWeight::Fill size="1.2rem"/>
            </button>
            <button class="join-item btn btn-primary">
              <Icon icon=CARET_LINE_RIGHT weight=IconWeight::Fill size="1.2rem"/>
            </button>
          </div>
        </div>

        <div class="flex flex-col justify-center items-center gap-2">
          // Showing x through y of z posts
          <div class="join justify-center items-center text-xs font-bold gap-4">
            <span class="join-item">
              "Showing"
            </span>
            <span class="join-item badge badge-sm badge-soft rounded-full">
              "1 through 10"
            </span>
            <span class="join-item">
              "of 100 posts"
            </span>
          </div>

          // Posts per page
          <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border border-secondary p-4">
            <legend class="fieldset-legend">"Posts Per Page"</legend>
            
            <div class="w-full max-w-xs">
              <input type="range" min="10" max="50" value="30" class="range range-secondary range-sm" step="10" />
              <div class="flex justify-between px-2.5 mt-2 text-xs">
                <span>10</span>
                <span>20</span>
                <span>30</span>
                <span>40</span>
                <span>50</span>
              </div>
            </div>

          </fieldset>
        </div>

      </div>

    </Autoscale>
  }
}

#[component]
pub fn RamblingsPost() -> impl IntoView {
  view! {

    <div id="top"
      class="hero min-h-64 bg-[url('https://images.unsplash.com/photo-1515879218367-8466d910aaa4?q=80&w=1169&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D')]"
    >
    </div>

    <Autoscale>

      <div class="flex flex-col border-b-1 border-base-300 py-4 mb-4">
        
        // * Header with author and title
        <div class="flex justify-between items-center py-4">

          <a class="btn btn-primary btn-sm" href="/ramblings">
            <Icon icon=LIST_MAGNIFYING_GLASS weight=IconWeight::Fill size="1.5rem"/>
            "Index"
          </a>

          <div id="author" class="flex justify-center items-center gap-2">
            <span class="text-[--font-subtitle] font-bold text-sm">
              "Lily Ana Valley"
            </span>
            <div class="avatar">
              <div class="w-10 rounded-full">
                <img
                  src="/assets/avatar.png"
                  alt="Tailwind-CSS-Avatar-component"
                />
              </div>
            </div>
          </div>

        </div>

        // * Post date and tags
        <div class="flex flex-col text-xs gap-2">
          <p>
            <span>"Published on"</span>
            <span class="font-bold mx-2">"March 13, 2025, 12:22 EST"</span>
          </p>
          <div class="flex gap-2">
            <button class="btn btn-xs">
              <Icon icon=TAG weight=IconWeight::Fill size="1rem"/>
              "tag"
            </button>
          </div>
        </div>

      </div>
  
      <Article>

        // * Cute little code thingy
        <p class="select-none text-md lg:text-lg font-bold">
          <span class="text-info/25 mx-1 italic">
            "pub fn"
          </span>
          <span class="text-success/25 mx-1">
            "post()"
          </span>
          <span class="text-base-300 mx-1">
            "->"
          </span>
          <span class="text-warning/25 mx-1 italic">
            "impl"
          </span>
          <span class="text-purple-300/25 mx-1">
            "IntoView"
          </span>
          <span class="text-base-300 mx-1">
            "{"
          </span>
        </p>
        // * Title
        <h1 class="text-8xl font-bold text-shadow-sm">"Post Title"</h1>
        // * Content
        <Markdown markdown={ placeholders::DESCRIPTION_LONG_MD.to_string() }/>
        // * End cute little code thingy block
        <p class="select-none text-md text-base-300 lg:text-lg font-bold">"}"</p>
        <div id="blink-caret"></div>

      </Article>

      <Article>
        <Code source={placeholders::RAMBLINGS_SAMPLE_POST_FEATURETEST}
        />
      </Article>

      <div class="flex flex-col border-t-1 border-base-300 py-4 my-8">
        <h2 class="text-neutral text-lg font-bold">"Stats"</h2>
        <div class="stats stats-vertical lg:stats-horizontal shadow">
          <div class="stat bg-base-100">
            <div class="stat-title">"Impressions"</div>
            <div class="stat-value text-info">"31K"</div>
            <div class="stat-desc">"Jan 1st - Feb 1st"</div>
          </div>
          <div class="stat bg-base-100">
            <div class="stat-title">"Shares"</div>
            <div class="stat-value text-warning">"12K"</div>
            <div class="stat-desc">"Jan 1st - Feb 1st"</div>
          </div>
          <div class="stat bg-base-100">
            <div class="stat-title">"Reactions"</div>
            <div class="stat-value text-error">"23K"</div>
            <div class="stat-desc">"Most: 😈"</div>
          </div>
        </div>
        <div class="flex justify-end items-center font-bold mt-2 gap-2">
          <div class="badge badge-error badge-lg badge-outline">
            "😈"
            <span class="mx-1">"14,893"</span>
          </div>
          <div class="badge badge-error badge-lg badge-outline">
            "😜"
            <span class="mx-1">"8,234"</span>
          </div>
        </div>
      </div>

      <div class="flex justify-center items-center gap-4 my-8">
        <div class="join">
          <a class="join-item btn btn-info btn-sm" href="#top">
            <Icon icon=ARROW_LINE_UP weight=IconWeight::Fill size="1.2rem"/>
            "Top"
          </a>
          <button class="join-item btn btn-warning btn-sm">
            <Icon icon=SHARE weight=IconWeight::Fill size="1.2rem"/>
            "Share"
          </button>
          <button class="join-item btn btn-error btn-sm">
            <Icon icon=SMILEY weight=IconWeight::Fill size="1.2rem"/>
            "React"
          </button>
        </div>
      </div>

    </Autoscale>

  }
}

#[server]
pub async fn get_index(term: Option<String>, pagination: u8) -> Result<(), ServerFnError> {
  Ok(())
}

#[server]
pub async fn get_post(post_id: PostId) -> Result<(), ServerFnError> {

  // TODO: Fetch a post and return its contents | metadata
  Ok(())

}

pub type PostId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
  pub id: PostId,
  pub metadata: PostMetadata,
  pub content_complete: String, // full markdown content of the post
  pub date_edited: Option<String>,
  pub stats: PostStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostAuthor {
  pub name: String,
  pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostStats {
  pub impressions: u32,
  pub shares: u32,
  pub reactions: BTreeMap<String, u32>, // emoji and count
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostMetadata {
  pub title: String,
  pub author: PostAuthor,
  pub date_published: String,
  pub content_truncated: String, // truncated content for previews
  pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSearchResult {
  pub posts: BTreeMap<PostId, PostMetadata>,
  pub total_count: usize, // total of all *published* posts, not current results or unpublished ones
  pub total_pages: usize, // total pages available based on pagination and total_count
  pub current_page: usize, // which page are we currently on, based on pagination + total_count
}

mod search {
  use super::*;

  pub fn search_posts(query: &str, page: usize, per_page: usize) -> PostSearchResult {
    // This function would implement the actual search logic.
    // For now, it returns an empty result.
    PostSearchResult {
      posts: BTreeMap::new(),
      total_count: 0,
      current_page: page,
      total_pages: 0,
    }
  }

}
