use eframe::egui;

struct App{
    click_count: u32,
    label_text: String,
}

impl eframe::App for App {
    //update is called every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My First egui App");
            ui.label(&self.label_text);
            ui.separator();
            
            if ui.button("Click me").clicked() {
                self.click_count += 1;
                self.label_text = format!("Clicked {} time(s)!", self.click_count);
            }
        });
    }
}

impl Default for App{
    fn default() -> Self{
        Self{
            click_count: 0,
            label_text: String::from("Click the button!")
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pomodoro Timer",
        options,
        Box::new(|_cc| Box::new(App::default())),

    )
    
}
