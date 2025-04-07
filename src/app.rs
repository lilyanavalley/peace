
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
  *,
  components::*,
};
use phosphor_leptos::*;
use crate::{ components, placeholders, views::* };


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
      <main class="flex flex-col h-full">
        <div class="flex flex-col grow h-full overflow-auto">
          <Routes fallback=|| "not found">
            <Route path=StaticSegment("/") view=homepage::HomePage/>
            <Route path=StaticSegment("/work") view=work::Work/>
            <ParentRoute path=path!("/contact") view=contact::Contact>
              <Route path=path!(":cid") view=contact::ContactUpdate/>
              <Route path=path!("") view=contact::ContactStart/>
            </ParentRoute>
            <Route path=StaticSegment("/ask") view=ask::Ask/>
            <Route path=path!("/*any") view=notfound::NotFound/>
          </Routes>
          <components::favoritequotes::FavoriteQuotes/>
          <components::footer::Footer/>
          <components::accessibility::Handlebar/>
        </div>
        <components::navigator::Navigator/>
      </main>
    </Router>
  }
}
