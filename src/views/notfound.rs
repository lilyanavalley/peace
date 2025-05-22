
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::{ placeholders, components::* };


/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
  // set an HTTP status code 404
  // this is feature gated because it can only be done during
  // initial server-side rendering
  // if you navigate to the 404 page subsequently, the status
  // code will not be set because there is not a new HTTP request
  // to the server
  #[cfg(feature = "ssr")]
  {
    // ? this can be done inline because it's synchronous
    // ? if it were async, we'd use a server function
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! {
    <Autoscale>
      <div class="flex flex-col flex-grow text-center justify-center items-center min-h-full">

        <Icon icon=BUG size="3rem" weight=IconWeight::Duotone/>
        <h1>"404: " { placeholders::NOTFOUND }</h1>
        
        <Article>
          <Markdown markdown={ placeholders::NOTFOUND_DESCRIPTION_MD.to_string() }/>
        </Article>

      </div>
    </Autoscale>
  }
}
