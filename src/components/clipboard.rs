
use web_sys;


pub fn copy_to_clipboard(text: &str) {
  web_sys::window()
    .unwrap()
    .navigator()
    .clipboard()
    .write_text(text);
  // TODO: handle errors here
}
