
use leptos::prelude::*;
use serde:: { Serialize, Deserialize };
use phosphor_leptos::*;


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

#[component]
pub fn FavoriteQuotes() -> impl IntoView {

  view! {
    // ! there's a hydration problem around this line.
    <div class="flex flex-col text-[var(--color-offwhite)]" style="margin: 2rem">

      <div class="flex flex-col text-[#988] items-center">
        <Icon icon=QUOTES size="1.5rem"/>
        <p>quote of the day</p>
      </div>
      
      // ! This suspense might be expecting a div of the same kind between fallback and normal.
      // TODO: Consider modifying the div classes and consult the open Leptos Book page on Suspense.
      <Suspense fallback=move || view! {
        <div id="quote-quotation" class="suspense self-center text-[var(--color-selectables-red)] w-3/4">
          pending pending pending
        </div>
        <p id="quote-citation" class="suspense self-center text-[.8rem] text-[var(--color-selectables-red)] w-3/4">
          -- me, uwu uwu owo uwu
        </p>
      }>

        { move || Suspend::new( async move {
          let quote = quote_today().await.unwrap();
          view! {
            <div id="quote-quotation" class="self-center before:content-['❝'] after:content-['❞'] text-[var(--color-selectables-red)] victor-mono-400 w-3/4">
              {quote.quotation}
            </div>
            <p id="quote-citation" class="self-center before:content-['–'] italic text-[.8rem] text-[var(--color-selectables-red)] victor-mono-400 w-3/4">
              {quote.citation}
            </p>
          }
        })}

      </Suspense>

    </div>
  }

}
