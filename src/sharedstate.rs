
use egui_inbox::UiInboxSender;
use crate::AppMessage;


pub struct SharedState {
    tx: UiInboxSender<AppMessage>
}

impl SharedState {
    pub fn new(tx: UiInboxSender<AppMessage>) -> Self {
        SharedState {
            tx
        }
    }
}
