
use leptos::{ prelude::*, html, logging::log };
use leptos_meta::*;
use leptos_router::{
  *,
  components::*,
};
use leptos_hotkeys::*;
use log:: { trace, debug, info, warn, error };
use phosphor_leptos::*;
use crate::{ components::*, placeholders, views::* };


#[cfg(feature="wip")]
#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  view! {
    
    <Stylesheet id="leptos" href=format!("/pkg/{}.css", env!("CARGO_PKG_NAME"))/>
    <Link rel="preconnect" href="https://unpkg.com"/>
    <Link rel="preconnect" href="https://fonts.googleapis.com"/>
    <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
    <Link href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&family=Fira+Sans:ital,wght@0,300;0,400;0,500;0,700;0,900;1,300;1,400;1,500;1,700;1,900&family=Flow+Circular&family=Victor+Mono:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
    <Link rel="authorization_endpoint" href="https://indieauth.com/auth"/>
    <Link rel="me" href="https://github.com/lilyanavalley"/>

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

    <script async src="https://rum.cronitor.io/script.js"></script>
    <script>
      "window.cronitor = window.cronitor || function() { (window.cronitor.q = window.cronitor.q || []).push(arguments); };
      cronitor('config', { clientKey: '023d177b319ea5eed8339cd8cc5392fc' });"
    </script>

    <Script src="https://cdn.jsdelivr.net/npm/theme-change@2.0.2/index.js"></Script>

    // ? injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href=format!("/pkg/{}.css", env!("CARGO_PKG_NAME"))/>
    <Link rel="preconnect" href="https://unpkg.com"/>
    <Link rel="preconnect" href="https://fonts.googleapis.com"/>
    <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
    <Link href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&family=Fira+Sans:ital,wght@0,300;0,400;0,500;0,700;0,900;1,300;1,400;1,500;1,700;1,900&family=Flow+Circular&family=Victor+Mono:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet"/>
    <Link rel="shortcut icon" type_="image/svg+xml" href="/assets/favicon.svg"/>
    <Link rel="authorization_endpoint" href="https://indieauth.com/auth"/>
    <Link rel="me" href="https://github.com/lilyanavalley"/>

    // sets the document title
    <Title text="lily valley"/>

    // content for this welcome page
    <Router>
      <Navigator/>
      // * bottom margin is provided for spacing the navigator so as to not to cut off content.
      <main class="flex flex-col h-full w-full overflow-y-scroll overflow-x-hidden mb-20 md:mb-28">
        <noscript>
          <div class="alert alert-warning">
          <Icon icon=WARNING size="1.5rem"/>
          <span>
            <p class="m-0.5">"JavaScript is turned off. Some features may not work as expected."</p>
            <a class="btn btn-sm m-0.5" href="https://www.enable-javascript.com/)">
              "See a Fix"
            </a>
            // <a class="btn btn-sm m-0.5" href="js/">
            //   "JS Usage Statement"
            // </a>
          </span>
          </div>
        </noscript>
        <Routes fallback=|| "not found">

          <Route path=StaticSegment("/") view=HomePage/>
        
          // <ParentRoute path=path!("/authentication") view=Authenticate>
          //   <Route path=path!("/otp/:code") view=OtpCode/>
          //   <Route path=path!("/register") view=WebauthnKeyRegister/>
          //   <Route path=path!("/keys") view=WebauthnKeys/>
          //   <Route path=path!("/sessions") view=AuthenticateSessions/>
          //   <Route path=path!("/logout") view=AuthenticateLogout/>
          //   <Route path=path!("") view=AuthenticateStart/>
          // </ParentRoute>
        
          <Route path=StaticSegment("/work") view=Work/>

          <ParentRoute path=StaticSegment("/ramblings") view=Ramblings>
            <Route path=StaticSegment("") view=RamblingsIndex/>
            <Route path=path!(":slug") view=RamblingsPost/>
          </ParentRoute>
        
          // <ParentRoute path=path!("/contact") view=contact::Contact>
          //   // <Route path=path!(":cid") view=contact::ContactUpdate/>
          //   // <Route path=path!("") view=contact::ContactStart/>
          //   </ParentRoute>
          <Route path=StaticSegment("/contact") view=ContactAlternative/> // ? this route is temporary.
        
          // * Remember to re-enable the navigator item too.
          // <Route path=StaticSegment("/ask") view=Ask/>

          <Route path=StaticSegment("/stats") view=Stats/>
        
          <Route path=path!("/*any") view=NotFound/>
        
        </Routes>
        <Footer/>
      </main>
    </Router>

  }
}
