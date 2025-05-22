use leptos::prelude::*;


#[component]
pub fn Autoscale(children: ChildrenFn) -> impl IntoView {
  view! {
    <div id="autoscale" class="flex flex-col w-full max-w-7xl md:w-3/4 self-center my-8">
      { children() }
    </div>
  }
}
