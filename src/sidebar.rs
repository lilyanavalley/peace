
use crate::sharedstate::SharedState;
use crate::AppMessage;
use egui::{Align, Button, Layout, RichText, Ui, Vec2};
use egui_flex::{Flex, FlexItem};


pub struct SideBar {
    
}

impl SideBar {

    pub fn ui(ui: &mut Ui, shared: &mut SharedState) -> bool {
        
        let mut clicked = false;

        ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
            
            ui.add_space(8.0);
            ui.spacing_mut().button_padding = egui::vec2(6.0, 4.0);

        });

        ui.add_space(8.0);
        ui.separator();

        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            ui.add_space(8.0);
            ui.hyperlink_to(
                "Source Code",
                "https://github.com/lilyanavalley/peace",
            );
        });

        clicked
    }

}
