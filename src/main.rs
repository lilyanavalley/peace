
extern crate log;
extern crate egui;
extern crate eframe;
extern crate egui_router;
extern crate egui_inbox;

use core::net;

use egui::{Context, RichText};
use egui_router::*;
use egui_inbox::{ type_inbox::TypeInbox, UiInbox };
use egui_phosphor::variants;
use eframe::Frame;
use log::{ debug, trace, info, warn, error };

mod sharedstate;
mod router;
mod sidebar;


#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {

    use pretty_env_logger;

    pretty_env_logger::init();
    let native_options = eframe::NativeOptions::default();

    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))))

}

#[cfg(target_arch = "wasm32")]
fn main() {

    use web_sys;
    use wasm_bindgen_futures;
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Trace).ok();

    let web_options = eframe::WebOptions::default();

    let document = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document");

    let canvas = document
        .get_element_by_id("the_canvas_id")
        .expect("Failed to find the_canvas_id")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("the_canvas_id was not a HtmlCanvasElement");

    wasm_bindgen_futures::spawn_local(async move {

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(App::new(cc)) as Box<dyn eframe::App>)),
            )
            .await;
        trace!("end WebRunner::new()");
        info!("start result: {:#?}", start_result);

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                    trace!("end remove loading text");
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });

}

struct App {

    /// Receiver channel.
    rx:             UiInbox<AppMessage>,
    /// SPA Router.
    router:         EguiRouter<sharedstate::SharedState>,
    /// Application shared state.
    state:          sharedstate::SharedState,
    /// Show SideBar?
    navbar_show:    bool,

}

impl App {

    fn new(cc: &eframe::CreationContext) -> Self {

        let context = &cc.egui_ctx;

        context.options_mut(|opts| {
            opts.max_passes = std::num::NonZeroUsize::new(4).unwrap();
        });
        egui_extras::install_image_loaders(context);
        context.enable_accesskit();
        
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        context.set_fonts(fonts);

        let (tx, rx) = egui_inbox::UiInbox::channel();
        let mut state = sharedstate::SharedState::new(tx);
        let router = router::init(&mut state);

        App {
            rx,
            router,
            state,
            navbar_show:    true
        }

    }

    fn landing(request: Request<sharedstate::SharedState>) -> impl Route<sharedstate::SharedState> {
        | ui: &mut egui::Ui, state: &mut sharedstate::SharedState | {
            ui.add(egui::Label::new("Landing Page"));
        }
    }

    fn about(request: Request<sharedstate::SharedState>) -> impl Route<sharedstate::SharedState> {
        | ui: &mut egui::Ui, state: &mut sharedstate::SharedState | {
            ui.add(egui::Label::new("About Page"));
        }
    }

}

impl eframe::App for App {

    fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame) {
        
        self.rx.set_ctx(context);
        self.rx.read_without_ctx().for_each(|message| match message {
            AppMessage::Back             => self.router.back().unwrap(),
            AppMessage::Navigate(n)      => self.router.navigate(&mut self.state, n).unwrap()
        });

        let width = context.screen_rect().width();
        let collapsible_sidebar = width < 800.0;
        let is_expanded = !collapsible_sidebar || self.navbar_show;

        egui::SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(170.0)
        .show_animated(context, is_expanded, |ui| {
            if sidebar::SideBar::ui(ui, &mut self.state) {
                self.navbar_show = false;
            }
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(context.style().visuals.panel_fill.gamma_multiply(0.7)))
            .show(context, |ui| {

                if collapsible_sidebar {
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        if ui.add(egui::Button::new(
                            egui::RichText::new(variants::fill::DOTS_THREE_OUTLINE_VERTICAL).size(32.0)
                        )).clicked() {
                            self.navbar_show = !self.navbar_show;
                        }   
                    });
                }

                if !(collapsible_sidebar && is_expanded) {
                    self.router.ui(ui, &mut self.state);
                }

            })
        ; // CentralPanel

    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        if _raw_input.modifiers.ctrl && _raw_input.modifiers.shift {
            trace!("Debug on hover ON");
            _ctx.set_debug_on_hover(true);
        }
        else {
            trace!("Debug on hover OFF");
            _ctx.set_debug_on_hover(false);
        }
    }

}

enum AppMessage {
    Navigate(String),
    Back
}

// enum AppImageSources {

//     Web(),
//     File(),

// }
