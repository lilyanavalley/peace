
use leptos::prelude::*;
use leptos::logging::log;
use log::{ trace, info, warn, error, debug };
use serde:: { Serialize, Deserialize };
use phosphor_leptos::*;
use crate:: { placeholders, components::Markdown };


#[server]
pub async fn quote_today() -> Result<ReturnedQuote, ServerFnError> {

  use std::sync::Mutex;
  use actix_web::{ web::Data, HttpRequest };
  use leptos_actix::extract;
  use crate::config::PeaceConfig;
  use mongodb;

  let request: HttpRequest = extract().await?;
  let config = request.app_data::<Data<Mutex<PeaceConfig>>>().unwrap();
  let mongodb = request.app_data::<Data<mongodb::Client>>().unwrap();;

  let mut config = config.lock().unwrap();
  config.quotes.queued_or_fetching(mongodb).await; // todo: do something with result
  
  if let Some(quote) = config.quotes.returned_quote() {
    Ok(ReturnedQuote {
      quotation: quote.quotation,
      citation:  quote.citation
    })
  }

  else {
    Ok(ReturnedQuote::default())
  }

}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnedQuote {
  pub quotation:  String,
  pub citation:   Option<String>,
}

impl Default for ReturnedQuote {
  fn default() -> Self {
    ReturnedQuote {
      quotation:  "There is no quote! The server broke while fishing for one. :(".to_string(),
      citation:   Some("The server".to_string()),
    }
  }
}

#[component]
pub fn FavoriteQuotes() -> impl IntoView {

  let quote = OnceResource::new(quote_today());

  view! {
    <div class="flex flex-col text-[var(--color-offwhite)] border-t-3 border-b-3 border-t-[#988] border-b-[#988]" style="margin: 2rem">

      <div class="flex flex-col text-[#988] items-center" style="padding: 1rem">
        <Icon icon=QUOTES size="1.5rem"/>
        <p style="margin-top: 0; margin-bottom: 0;">{ placeholders::FAVQUOTES }</p>
      </div>
      
      <Suspense fallback=move || view! {
        <div id="quote-quotation" class="suspense self-center text-[var(--color-selectables-red)] w-3/4">
          "pending pending pending"
        </div>
        <p id="quote-citation" class="suspense self-center text-[.8rem] text-[var(--color-selectables-red)] w-3/4">
          "-- me, uwu uwu owo uwu"
        </p>
      }>

        { move || Suspend::new( async move {
          let mut quote = quote.await.unwrap_or_default();
          let citation = quote.citation.clone().unwrap_or_default();
          view! {
            <div id="quote-quotation" class="self-center victor-mono-400 max-w-3/4">
              <Markdown markdown={ quote.quotation } />
            </div>
            <Show
              when=move || { quote.citation.is_some() }
              fallback=move || view! {}
            >
              <div id="quote-citation" class="self-center before:content-['–'] italic text-[.8rem] text-center victor-mono-400 w-3/4">
                // ? Why the double clone, you ask? Because the compiler is a bit dumb and doesn't know that the
                // ? citation is not going to be used again. 🤡
                // TODO: Fix this.
                <Markdown markdown={ citation.clone() } />
              </div>
            </Show>
          }
        })}

      </Suspense>

    </div>
  }.into_any()

}
