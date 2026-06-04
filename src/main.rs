use eframe::egui;

struct App {
    click_count: u32,
    label_text: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            click_count: 0,
            label_text: String::from("Click the button!"),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My First egui App");
            ui.label(&self.label_text);

            if ui.button("Click me").clicked() {
                self.click_count += 1;
                self.label_text = format!("Clicked {} time(s)!", self.click_count);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Work Timer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}