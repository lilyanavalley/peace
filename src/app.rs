
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
  *,
  components::*,
};
use phosphor_leptos::*;
use crate::{ components, placeholders };


#[cfg(feature="wip")]
#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  view! {
    <Stylesheet id="leptos" href=format!("/pkg/{}.css", env!("CARGO_PKG_NAME"))/>
    <Link rel="preconnect" href="https://unpkg.com"/>
    <Link rel="preconnect" href="https://fonts.googleapis.com"/>
    <Link rel="preconnect" href="https://fonts.gstatic.com"/>
    <Link href="https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@100;300;600;700&family=Dosis:wght@200..800&family=Fira+Code:wght@300..700&family=Flow+Circular&family=Redacted+Script&family=Ubuntu+Condensed&family=Ubuntu+Mono:ital,wght@0,400;0,700;1,400;1,700&family=Ubuntu+Sans:ital,wght@0,100..800;1,100..800&family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&family=Victor+Mono:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

    // sets the document title
    <Title text="Lily's Website ⚛️"/>
    <Router>

      <Routes fallback=|| "not found">
        <Route path=StaticSegment("") view=Wip />
      </Routes>

    </Router>
  }
}

#[cfg(not(feature="wip"))]
#[component]
pub fn App() -> impl IntoView {
  // ? Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    // ? injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href=format!("/pkg/{}.css", env!("CARGO_PKG_NAME"))/>
    <Link rel="preconnect" href="https://unpkg.com"/>
    <Link rel="preconnect" href="https://fonts.googleapis.com"/>
    <Link rel="preconnect" href="https://fonts.gstatic.com"/>
    <Link href="https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@100;300;600;700&family=Dosis:wght@200..800&family=Fira+Code:wght@300..700&family=Flow+Circular&family=Redacted+Script&family=Ubuntu+Condensed&family=Ubuntu+Mono:ital,wght@0,400;0,700;1,400;1,700&family=Ubuntu+Sans:ital,wght@0,100..800;1,100..800&family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&family=Victor+Mono:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

    // sets the document title
    <Title text="Lily's Website ⚛️"/>

    // content for this welcome page
    <Router>
      <main class="flex flex-col-reverse h-full">
        <Routes fallback=move || "Not found.">
          <Route path=WildcardSegment("any") view=Wip />
          // <Route path=StaticSegment("") view=HomePage/>
          // <Route path=StaticSegment("work") view=Work/>
          // <Route path=WildcardSegment("any") view=NotFound/>
        </Routes>
      </main>
    </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
  view! {

    <components::navigator::Navigator/>
    
    <div class="flex flex-col grow h-full overflow-auto">
  
      <div id="profile" class="flex flex-col self-center text-center border-0 rounded-full" style="margin-top: 2rem">
        <img src="assets/avatar.png" class="self-center w-1/4 rounded-full" alt="Profile picture of Lily"/>
        <h2 class="self-center justify-self-center">Lily Ana Valley</h2>
      </div>

      <div id="profile-badges" class="w-3/4 self-center">
        <p class="text-center">{ placeholders::DESCRIPTION_SHORT }</p>
      </div>

      <article class="w-3/4 self-center text-wrap hyphens-auto whitespace-pre-wrap" style="margin: 2rem">
        { placeholders::DESCRIPTION_LONG }
      </article>

      <components::favoritequotes::FavoriteQuotes/>
      <components::footer::Footer/>

    </div>
  }
}

#[component]
fn Work() -> impl IntoView {
  view! {

    <components::navigator::Navigator/>
    
    <div id="portfolio" class="flex flex-col grow h-full overflow-auto">

      <h1 class="text-center">Work</h1>

      <div class="flex flex-col grow h-full overflow-auto self-center w-3/4">
        
        <a class="flex items-center border-x-2 rounded-lg border-[var(--color-selectables-green)] hover:border-[var(--color-brightwhite)] without-link-symbol" href="https://github.com/lilyanavalley/">
          <img src="favicon.ico" alt="sample project name icon"/>
          <div class="dosis-400" style="margin: .5rem">Sample Project Name</div>
        </a>

      </div>

      <components::favoritequotes::FavoriteQuotes/>
      <components::footer::Footer/>

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
    <components::navigator::Navigator/>
    <div class="flex flex-col grow h-full overflow-auto">
      <div class="flex flex-col flex-grow text-center justify-center">

        <h1 class="dosis-400" style="font-size: 500%; margin: 0;">~ 404 ~</h1>
        <h2 class="dosis-400" style="font-size: 200%; margin: 0;">Not Found</h2>
        
        <article style="margin: 1rem;">

          "Requested page could not be located. If you expected something in particular, please "

          <a class="text-[var(--color-selectables-red)]" href="https://github.com/lilyanavalley/peace/issues">
            <Icon icon=ASTERISK/>
            " open an issue"
          </a>
          
          " on this site's GitHub repo."

        </article>

      </div>
      <components::footer::Footer/>
    </div>
  }
}

#[component]
fn Wip() -> impl IntoView {
  view! {
    <div class="flex flex-col grow h-full overflow-auto justify-center items-center animate-(--animation-rainbowswirl)">
      <Icon icon=FLOWER size="3rem" weight=IconWeight::Duotone/>
      <p class="victor-mono-400 italic">content coming soon</p>
    </div>
  }
}
