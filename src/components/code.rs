
use leptos::prelude::*;
use phosphor_leptos::*;
use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{ markdown_to_html_with_plugins, Options, Plugins };
use crate::components::clipboard::copy_to_clipboard;


#[component]
pub fn Plain(text: &'static str) -> impl IntoView {

  let (do_copy, set_do_copy) = create_signal(false);
  let copy_to_clipboard = move |_| {
    copy_to_clipboard(text);
    set_do_copy.update(|v| *v = true);
    set_do_copy.update(|v| *v = false);
  };

  view! {
    <div class="border-4 border-[var(--color-base-300)] bg-[var(--color-base-200)] max-w-full mx-1">
      
    <pre style="padding: .5rem; margin: 0; overflow: scroll" inner_html=text></pre>
    
      <div class="border-t-4 border-[var(--color-base-300)] flex items-center justify-between bg-[var(--color-base-300)]">
        
        <button class="btn btn-sm btn-accent" on:click=copy_to_clipboard>
          <Icon icon=COPY size="1.2rem"/>
          <b style="margin-right: .25rem">Copy</b>
        </button>

        <Show
          when= move || { do_copy.get() == true }
          fallback=|| view! {}
        >
          <div class="opacity-0 animate-(--animation-fader) text-[var(--color-info)]">
            <span>"Copied!"</span>
          </div>
        </Show>

      </div>

    </div>
  }
}

#[component]
pub fn Code(source: &'static str) -> impl IntoView {

  let (do_copy, set_do_copy) = create_signal(false);
  let copy_to_clipboard = move |_| {
    copy_to_clipboard(source);
    set_do_copy.update(|v| *v = true);
    set_do_copy.update(|v| *v = false);
  };

  let builder = SyntectAdapterBuilder::new().css();
  let adapter = builder.build();
  let options = Options::default();
  let mut plugins = Plugins::default();
  plugins.render.codefence_syntax_highlighter = Some(&adapter);

  // TODO: interpret source code language and reflect that in the below format.
  let input = format!("```rust\n{}\n```", source);
  let output = markdown_to_html_with_plugins(&input, &options, &plugins);
  
  view! {
    <div class="border-4 border-[var(--color-base-300)] bg-[var(--color-base-200)] max-w-full mx-1">
    
      <pre style="padding: .5rem; margin: 0; overflow: scroll" inner_html=output></pre>
      
      <div class="border-t-4 border-[var(--color-base-300)] flex items-center justify-between bg-[var(--color-base-300)]">
        
        <div class="flex items-center">
        
          <button class="btn btn-sm btn-accent" on:click=copy_to_clipboard>
            <Icon icon=COPY size="1.2rem"/>
            <b style="margin-right: .25rem">Copy</b>
          </button>

          <div class="tooltip" style="margin-left: .5rem">
            <Icon icon=FILE_RS size="1.5rem" color="#AAA"/>
            <span>"Rust"</span>
          </div>

        </div>

        <Show
          when= move || { do_copy.get() == true }
          fallback=|| view! {}
        >
          <div class="opacity-0 animate-(--animation-fader) text-[var(--color-info)]">
            <span>"Copied!"</span>
          </div>
        </Show>

      </div>

    </div>
  }
}

enum FileTypeIcon {
  Blank,
  C,
  Cpp,
  CSharp,
  Cloud,
  Css,
  Html,
  Js,
  Jsx,
  Md,
  Py,
  Rs,
  Ts,
  Tsx,
  Txt,
}

impl FileTypeIcon {

  fn from_info_string(info: &str) -> Self {
    match info {
      "rust"  => Self::Rs,
      _       => Self::Blank
    }
  }

  fn from_extension(extension: &str) -> Self {
    match extension {
      "c"     => Self::C,
      "cpp"   => Self::Cpp,
      "cs"    => Self::CSharp,
      "css"   => Self::Css,
      "html"  => Self::Html,
      "js"    => Self::Js,
      "jsx"   => Self::Jsx,
      "md"    => Self::Md,
      "py"    => Self::Py,
      "rs"    => Self::Rs,
      "ts"    => Self::Ts,
      "tsx"   => Self::Tsx,
      "txt"   => Self::Txt,
      "yaml"  => Self::Cloud,
      "yml"   => Self::Cloud,
      "json"  => Self::Cloud,
      _       => Self::Blank
    }
  }

  fn to_icon(&self) -> &'static IconWeightData {
    match self {
      Self::C       => FILE_C,
      Self::Cpp     => FILE_CPP,
      Self::CSharp  => FILE_C_SHARP,
      Self::Css     => FILE_CSS,
      Self::Html    => FILE_HTML,
      Self::Js      => FILE_JS,
      Self::Jsx     => FILE_JSX,
      Self::Md      => FILE_MD,
      Self::Py      => FILE_PY,
      Self::Rs      => FILE_RS,
      Self::Ts      => FILE_TS,
      Self::Tsx     => FILE_TSX,
      Self::Txt     => FILE_TXT,
      Self::Cloud   => FILE_CLOUD,
      _             => FILE
    }
  }

  fn hint(&self) -> &'static str {
    match self {
      Self::C       => "C",
      Self::Cpp     => "C Plus Plus",
      Self::CSharp  => "C Sharp",
      Self::Css     => "Cascading Style Sheet",
      Self::Html    => "Hyper Text Markup Language",
      Self::Js      => "Javascript",
      Self::Jsx     => "Javascript",
      Self::Md      => "Markdown",
      Self::Py      => "Python",
      Self::Rs      => "Rust",
      Self::Ts      => "Typescript",
      Self::Tsx     => "Typescript",
      Self::Txt     => "Text",
      _             => "File"
    }
  }

}
