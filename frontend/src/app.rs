#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::{
    modules::pages::{Page, Pages},
    utils::Utils,
};

pub struct App {
    page: Page,
    screen_reader_state: bool,
    pages: Pages,
}

impl Default for App {
    fn default() -> Self {
        Self {
            page: Page::Home,
            screen_reader_state: false,
            pages: Pages {},
        }
    }
}

impl eframe::App for App {
    /// The entry point of the application.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the `egui::Context`.
    /// * `frame` - A reference to the `eframe::Frame`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.options_mut(|o| o.screen_reader = self.screen_reader_state);

        self.header(ctx);
        self.body(ctx);
        self.footer(ctx);
    }
}

impl App {
    const XODIUM_REPOSITORY_URL: &'static str = "https://github.com/XodiumSoftware";
    const XBIM_REPOSITORY_URL: &'static str = "https://github.com/XodiumSoftware/xBIM";
    const VANILLAPLUS_REPOSITORY_URL: &'static str =
        "https://github.com/XodiumSoftware/VanillaPlus";
    const EGUI_REPOSITORY_URL: &'static str = "https://github.com/emilk/egui";
    const EFRAME_REPOSITORY_URL: &'static str =
        "https://github.com/emilk/egui/tree/master/crates/eframe";
    const CONTACT_EMAIL_URL: &'static str = "mailto:info@xodium.org";
    const LICENSE_URL: &'static str = "https://www.gnu.org/licenses/agpl-3.0.html";

    /// The header of the application.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the `egui::Context`.
    fn header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.columns(2, |cols| {
                    cols[0].with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.heading(
                            egui::RichText::new("Xodium")
                                .color(egui::Color32::from_hex("#CB2D3E").unwrap())
                                .strong(),
                        );
                        ui.hyperlink_to(
                            egui::special_emojis::GITHUB.to_string(),
                            Self::XODIUM_REPOSITORY_URL,
                        )
                        .on_hover_text("Github Repo");
                        egui::warn_if_debug_build(ui);
                        ui.add_space(15.0);
                        if self.page == Page::Home {
                            ui.menu_button("Projects", |ui| {
                                ui.vertical(|ui| {
                                    Utils.project_card(
                                        ui,
                                        "xBIM",
                                        "All-in solution to BIM models, written in Rust",
                                        Self::XBIM_REPOSITORY_URL,
                                    );
                                    ui.add_space(5.0);
                                    Utils.project_card(
                                        ui,
                                        "VanillaPlus",
                                        "Minecraft plugin that enhances the base gameplay",
                                        Self::VANILLAPLUS_REPOSITORY_URL,
                                    );
                                });
                            });
                        }
                    });
                    cols[1].with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // TODO: make settings better.
                        ui.menu_button("⚙", |ui| {
                            ui.label("Settings");
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label("Screen Reader:");
                                Utils::screen_reader_switch(ui, &mut self.screen_reader_state);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Theme Preference:");
                                egui::widgets::global_theme_preference_switch(ui);
                            });
                        });
                        ui.button("⎆")
                            .on_hover_text("Control Panel")
                            .clicked()
                            .then(|| self.page = Page::ControlPanel);
                    });
                });
            });
        });
    }

    /// The body of the application.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the `egui::Context`.
    fn body(&mut self, ctx: &egui::Context) {
        match self.page {
            Page::Home => self.pages.home(ctx),
            Page::ControlPanel => self.pages.control_panel(ctx),
        }
    }

    /// The footer of the application.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the `egui::Context`.
    fn footer(&self, ctx: &egui::Context) {
        // TODO: make bottom_panel vertically stacked when the window is too narrow.
        // TODO: fix layout.
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.columns(3, |cols| {
                cols[0].with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("© 2024 ");
                        ui.hyperlink_to("XODIUM™", Self::XODIUM_REPOSITORY_URL);
                        ui.label(". Open-Source (CAD) Software Company.");
                    });
                });
                cols[1].with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Powered by ");
                        ui.hyperlink_to("egui", Self::EGUI_REPOSITORY_URL);
                        ui.label(" and ");
                        ui.hyperlink_to("eframe", Self::EFRAME_REPOSITORY_URL);
                        ui.label(".");
                    });
                });
                cols[2].with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 16.0;
                        ui.hyperlink_to("About", Self::XODIUM_REPOSITORY_URL);
                        ui.hyperlink_to("Licensing", Self::LICENSE_URL);
                        ui.hyperlink_to("Contact", Self::CONTACT_EMAIL_URL);
                    });
                });
            });
        });
    }
}
