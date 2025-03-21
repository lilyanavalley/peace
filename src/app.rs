use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment, WildcardSegment,
};
use serde:: { Serialize, Deserialize };


#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href=format!("/pkg/{}.css", env!("CARGO_PKG_NAME"))/>
    <Link rel="preconnect" href="https://unpkg.com"/>
    <Link rel="preconnect" href="https://fonts.googleapis.com"/>
    <Link rel="preconnect" href="https://fonts.gstatic.com"/>
    <Link href="https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@100;300;600;700&family=Dosis:wght@200..800&family=Fira+Code:wght@300..700&family=Flow+Circular&family=Redacted+Script&family=Ubuntu+Condensed&family=Ubuntu+Mono:ital,wght@0,400;0,700;1,400;1,700&family=Ubuntu+Sans:ital,wght@0,100..800;1,100..800&family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&family=Victor+Mono:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

    <script src="https://unpkg.com/@phosphor-icons/web@2.1.1"></script>

    // sets the document title
    <Title text="Lily's Website ⚛️"/>
    
    // content for this welcome page
    <Router>
      <main class="flex flex-col-reverse h-full">
        <Routes fallback=move || "Not found.">
          <Route path=StaticSegment("") view=HomePage/>
          <Route path=WildcardSegment("any") view=NotFound/>
        </Routes>
      </main>
    </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
  view! {
    <RouterBar/>
    
    <div class="flex flex-col grow h-full overflow-auto">
  
      <div id="profile" class="flex flex-col self-center text-center border-0 rounded-full" style="margin-top: 2rem">
        <img src="assets/avatar.png" class="self-center w-1/4 rounded-full" />
        <h2 class="self-center justify-self-center">Lily Ana Valley</h2>
      </div>

      <div id="profile-badges" class="w-3/4 self-center">
        <p class="text-center">... 1</p>
      </div>

      <article class="w-3/4 self-center" style="margin: 2rem">
      "
        ... 2
      "
      </article>

      <FavoriteQuotes/>
      <Footer/>

    </div>
  }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
  // set an HTTP status code 404
  // this is feature gated because it can only be done during
  // initial server-side rendering
  // if you navigate to the 404 page subsequently, the status
  // code will not be set because there is not a new HTTP request
  // to the server
  #[cfg(feature = "ssr")]
  {
    // this can be done inline because it's synchronous
    // if it were async, we'd use a server function
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! {
    <h1>"Not Found"</h1>
  }
}

// TODO: Consider moving the below items into their own modules.

#[server]
pub async fn quote_today() -> Result<ReturnedQuote, ServerFnError> {
  
  Ok(ReturnedQuote {
    quotation:  "This is a quote!".to_string(),
    citation:   "This is a citation".to_string()
  })

}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnedQuote {
  pub quotation:  String,
  pub citation:   String,
}

/// Router Bar component.
#[component]
fn RouterBar() -> impl IntoView {
  view! {
    <div class="flex">
      <For
        each=move || ROUTERBAR_SAMPLE
        key=|route| route.route
        children=move |route: RouterBarItem| {
          view! {
            <a
              class=move || format!("flex flex-col flex-auto font-normal text-center justify-center without-link-symbol {}", route.colorsa)
              href={route.route}
            >
              <i style="margin: .5rem; margin-bottom: .1rem" class=move || format!("text-[30px] {}", route.icon)/>
              <p style="margin: 0; margin-bottom: .5rem">{route.label}</p>
            </a>
          }
        }
      />
    </div>
  }
}

const ROUTERBAR_SAMPLE: [RouterBarItem; 4] = [
  RouterBarItem {
    icon: "ph-fill ph-flower",
    label: "About",
    route: "/",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-purple"
  },
  RouterBarItem {
    icon: "ph-fill ph-briefcase",
    label: "Work",
    route: "/work",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-violet"
  },
  RouterBarItem {
    icon: "ph-fill ph-paper-plane-tilt",
    label: "Contact",
    route: "/contact",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-purple"
  },
  RouterBarItem {
    icon: "ph-fill ph-chats-circle",
    label: "Ask",
    route: "/ama",
    colortw: "text-purple-300",
    colorsa: "--color-selectables-blue"
  },
];

struct RouterBarItem {
  icon: &'static str,
  label: &'static str,
  route: &'static str,
  colortw: &'static str,
  colorsa: &'static str
}

#[component]
fn FavoriteQuotes() -> impl IntoView {

  view! {
    // ! there's a hydration problem around this line.
    <div class="flex flex-col text-[var(--color-offwhite)] border-t-2 border-[#988]" style="margin-top: 2rem">
      <p class="flex flex-col text-[#988] text-center">
        <i class="ph ph-quotes"/>
        quote of the day
      </p>
      
      // ! This suspense might be expecting a div of the same kind between fallback and normal.
      // TODO: Consider modifying the div classes and consult the open Leptos Book page on Suspense.
      <Suspense
        fallback=move || view! {
          <div class="suspense">
            pending pending pending
          </div>
          <p class="suspense">
            -- me, uwu uwu owo uwu
          </p>
        }>

        {move || leptos::prelude::Suspend::new(async move {
          let quote_today = quote_today().await.unwrap();
          view! {

            <div class="self-center before:content-['❝'] after:content-['❞'] text-[var(--color-selectables-red)] victor-mono-400 w-3/4">
              {quote_today.quotation}
            </div>
            <p class="self-center before:content-['–'] italic text-[.8rem] text-[var(--color-selectables-red)] victor-mono-400 w-3/4">
              {quote_today.citation}
            </p>

          }
        })}

      </Suspense>

    </div>
  }

}

#[component]
fn Footer() -> impl IntoView {
  view! {
    <div class="text-center text-[#666] border-t-2 border-t-neutral-800">
      <p class="text-xs m-1">the website of</p>
      <p class="text-xs m-3 mb-1">Lily Ana Valley</p>
      <p class="text-xs m-1 ">
        <a href="https://github.com/lilyanavalley/peace" class="text-[var(--color-selectables-pink)]">
          <i class="ph-fill ph-github-logo"/>
          Source
        </a>
        <a class="text-[#FFF]" disabled>
          <i class="ph-fill ph-asterisk"/>
          Issues
        </a>
        <a href="https://www.gnu.org/licenses/agpl-3.0.en.html" class="text-[var(--color-selectables-blue)]">
          <i class="ph-fill ph-scroll"/>
          AGPL-3
        </a>
      </p>
    </div>
  }
}

#[cfg(feature = "ssr")]
pub mod api {

  use mongodb;
  use leptos::*;
  use leptos::prelude::*;
  use actix_web::web;

  pub const MONGODB_D_PROFILE:  &'static str = "profile";
  pub const MONGODB_C_PROFILE_QUOTES: &'static str = "quotes";

  /// Retrieve a quote for *today*.
  async fn mongodb_database(mongodb: actix_web::web::Data<mongodb::Client>, database: &str) -> Result<mongodb::Database, ServerFnError> {

    Ok(mongodb.database(database))

  }

}
