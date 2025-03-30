
use leptos::prelude::*;
use phosphor_leptos::*;
use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{ markdown_to_html_with_plugins, Options, Plugins };



#[component]
pub fn Code(source: &'static str) -> impl IntoView {

  // let markdown = markdown_to_html(markdown, &Options::default());

  let builder = SyntectAdapterBuilder::new().css();
  let adapter = builder.build();
  let options = Options::default();
  let mut plugins = Plugins::default();
  plugins.render.codefence_syntax_highlighter = Some(&adapter);

  let input = format!("```rust\n{}\n```", source);
  let output = markdown_to_html_with_plugins(&input, &options, &plugins);
  
  view! {
    <div class="border-4 border-[var(--color-deepblack)] rounded-[1rem]" style="margin-top: 1rem; margin-bottom: 1rem;">
    
      <pre style="padding: .5rem; margin: 0; overflow: scroll" inner_html=output></pre>
      
      <div class="flex items-center justify-between bg-[var(--color-deepblack)] border-t-4 border-[var(--color-deepblack)]" style="padding: .25rem; padding-left: .5rem; padding-right: .5rem; border-radius: .75rem;">
        
        <div class="tooltip">
          <Icon icon=FILE_RS size="1.5rem" color="#AAA"/>
          <span>"Rust"</span>
        </div>
        
        <button class="flex flex-row">
          <Icon icon=COPY size="1.2rem"/>
          <b style="margin-right: .25rem">Copy</b>
        </button>

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
