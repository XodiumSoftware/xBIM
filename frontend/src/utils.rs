#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]
pub struct Utils;

#[allow(dead_code)]
impl Utils {
    /// Returns an `egui::Color32` from a HEX color code.
    ///
    /// # Arguments
    ///
    /// * `hex` - A `&str` representing the HEX color code.
    ///
    /// # Returns
    ///
    /// * `egui::Color32` - The color represented by the HEX code.
    pub fn hex_color(hex: &str) -> egui::Color32 {
        egui::Color32::from_hex(hex).expect("Invalid HEX Color Code!")
    }

    /// Returns the current time in seconds since midnight as an `Option<f64>`.
    ///
    /// This function uses the `chrono` crate to get the local time and calculates
    /// the number of seconds from midnight, including fractional seconds.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The current time in seconds since midnight.
    /// * `None` - If the time could not be retrieved.
    pub fn current_time_in_seconds() -> Option<f64> {
        use chrono::Timelike;
        let time = chrono::Local::now().time();
        Some(time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64))
    }

    /// Returns the screen reader switch state.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to an `egui::Ui`.
    /// * `state` - A mutable reference to a `bool` representing the screen reader state.
    pub fn screen_reader_switch(ui: &mut egui::Ui, state: &mut bool) {
        let (icon, tooltip) = if *state {
            ("🔊", "Screen Reader ON")
        } else {
            ("🔇", "Screen Reader OFF")
        };
        if ui.button(icon).on_hover_text(tooltip).clicked() {
            *state = !*state;
        }
    }

    /// Displays a project card with a title, description, and a button to view the project.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to an `egui::Ui`.
    /// * `title` - The title of the project.
    /// * `desc` - The description of the project.
    /// * `url` - The URL to view the project.
    pub fn project_card(&self, ui: &mut egui::Ui, title: &str, desc: &str, url: &str) {
        //TODO: make the card clickable.
        let response = ui
            .group(|ui| {
                ui.set_max_width(200.0);
                ui.vertical(|ui| {
                    ui.heading(
                        egui::RichText::new(title)
                            .color(egui::Color32::from_hex("#CB2D3E").unwrap()),
                    );
                    ui.label(egui::RichText::new(desc));
                });
            })
            .response
            .interact(egui::Sense::click());
        response.clicked().then(|| ui.hyperlink(url));
    }

    /// Draws a grid on the given `Painter` within the given `Rect`.
    ///
    /// # Arguments
    ///
    /// * `painter` - A reference to the `egui::Painter`.
    /// * `rect` - The `Rect` to draw the grid within.
    pub fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        // TODO: refactor, or check if we can use a library?
        let grid_size = 20.0;
        let color = egui::Color32::from_gray(200);

        let min = rect.min;
        let max = rect.max;

        let mut x = min.x;
        while x <= max.x {
            painter.line_segment(
                [egui::pos2(x, min.y), egui::pos2(x, max.y)],
                egui::Stroke::new(1.0, color),
            );
            x += grid_size;
        }

        let mut y = min.y;
        while y <= max.y {
            painter.line_segment(
                [egui::pos2(min.x, y), egui::pos2(max.x, y)],
                egui::Stroke::new(1.0, color),
            );
            y += grid_size;
        }
    }
}
