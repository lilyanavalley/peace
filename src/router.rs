
use egui_router::EguiRouter;
use egui_phosphor::variants::{ regular, fill };
use crate::{App, sharedstate::SharedState, AppMessage };


pub const ROUTES: &'static [Route] = &[
    Route {
        title:          "Home",
        description:    None,
        icon:           Some(regular::FLOWER),
        route:          "/"
    },
    Route {
        title:          "About",
        description:    Some("introduction to Lily and this site"),
        icon:           Some(regular::USER_FOCUS),
        route:          "/about"
    },
    Route {
        title:          "Work",
        description:    Some("collection of works and contributions"),
        icon:           Some(regular::BRACKETS_CURLY),
        route:          "/work"
    },
    Route {
        title:          "Ask Me Anything",
        description:    Some("submit a question to be responded to publicly"),
        icon:           Some(regular::CHAT_CIRCLE),
        route:          "/ask"
    },
    Route {
        title:          "Contact",
        description:    Some("submit direct inquiries and find me on other platforms"),
        icon:           Some(regular::MAILBOX),
        route:          "/contact"
    },
    Route {
        title:          "Site",
        description:    Some("fun statistics and information about this site"),
        icon:           Some(regular::BROADCAST),
        route:          "/site"
    }
];

struct Route {

    /// Human-presented string representing the route.
    title:          &'static str,

    /// Description of this route.
    description:    Option<&'static str>,

    /// Identifier of a Phosphor icon.
    icon:           Option<&'static str>,

    /// Site route to refer to on button activation.
    route:          &'static str

}

pub fn init(state: &mut SharedState) -> EguiRouter<SharedState> {
    EguiRouter::builder()
        .history({
            #[cfg(target_arch = "wasm32")]
            let history =
                egui_router::history::BrowserHistory::new(Some("/hello_egui/#".to_string()));
            #[cfg(not(target_arch = "wasm32"))]
            let history = egui_router::history::DefaultHistory::default();
            history
        })
        .default_duration(0.3)
        .default_path("/")
        .route("/", App::landing)
        .route("/about", App::about)
        .build(state)
}
