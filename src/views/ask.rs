
use leptos::prelude::*;
use phosphor_leptos::*;
use crate::components;


#[component]
pub fn Ask() -> impl IntoView {
  view! {
    <h1 class="text-center">Ask Me Anything</h1>
    
    <div class="flex flex-col self-center w-3/4">
      <components::article::Article>
        <components::article::Markdown markdown="Questions about [Rust]()? Maybe, you (and other visitors) would like to know something oddly specific to Technology, Art, Copyleft and/or the Human Condition? Ask me anything, *Reddit AMA* style.

An account or social media platform is not required to ask me questions. All questions are anonymous, yet public.".to_string()/>
      </components::article::Article>

    </div>

    // Sample questions list.
    <div id="questions" class="flex flex-col self-center fira-code-400 bg-[var(--color-vantablack)] w-3/4">
      <div style="margin: 1rem; padding: 1rem; border-radius: 2rem;" class="flex flex-col bg-[var(--color-deeperblack)] b-[.25rem] b-solid b-[#222]">
        <div class="" style="margin-bottom: 1rem;">
            <div class="" style="font-style: italic;font-family: Victor Mono;color: var(--color-selectables-blue);font-weight: bold;font-size: 1.2rem;background-color: #a0c4ff11;border-radius: 1rem;margin-bottom: .5rem;">
                "//? Anonymous asks"
            </div>
            <div class="">
                "What do you like about Rust exactly?"
            </div>
        </div>
        <div class="" style="margin-bottom: 1rem;">
            <div style="font-style: italic;font-family: Victor Mono;color: var(--color-selectables-green);font-weight: bold;font-size: 1.2rem;background-color: #caffbf11;border-radius: 1rem;margin-bottom: .5rem;">
                "//* Lily replies"
            </div>
            <div>
                "Its 'type safety' and 'data borrowing' rules help take the anxiety out of debugging, knowing full well the shape of the fundamental parts going in to an application's design. Writing Rust code forces oneself to think about how data is stored, accessed and evacuated from the moment it is initialized to the moment it is dropped. Concurrency and parallelism is not just a footnote for this language and Tokio proves that. And, the built-in web documentation makes it painless to communicate an API and/or your application to other developers. Rust is simply amazing. Not perfect, or event right sometimes, but it is an amazing work."
            </div>
        </div>
        <div class="" 
            style="font-style: italic;font-family: Victor Mono;color: #AAA;font-weight: bold;background-color: #AAA1;border-radius: 1rem;font-size: 1.05rem;margin-bottom: .5rem;">
            "// Answered on 2025.12.12 at 12:12 EST"
        </div>
        <div class="text-[.9rem]">
            <a href="#" class="text-[var(--color-selectables-red)]">
              <Icon icon=EYE/>
              "Request a Review"
            </a>
            <a href="#" class="text-[var(--color-selectables-blue)]">
              <Icon icon=QUOTES/>
              "Ask a Similar Question"
            </a>
        </div>
      </div>
    </div>

  }
}
