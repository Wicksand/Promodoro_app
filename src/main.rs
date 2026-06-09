#![windows_subsystem = "windows"]

//ui prossessing crate
use std::time::Instant;
use eframe::egui;

// audio processing crate
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::fs::File;


//variables that are used each session
struct App{
    study_time: String,
    break_time: String,
    timer: String,
    running: bool,
    is_break: bool,
    start_time:Option<Instant>,
    study_sec: u64,
    break_sec: u64,
    wait: bool,
    completed_sessions: Vec<String>,

    //audio specific variables
    _stream: OutputStream, 
    stream_handle: rodio::OutputStreamHandle,
    buzzer_sink: Option<Sink>,
}

// defalt initialises all of the variables
impl Default for App {
    fn default() -> Self {
       let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            study_time: String::new(),
            break_time: String::new(),
            timer: String::new(),
            running: false,
            is_break: false,
            start_time: None,
            study_sec: 0,
            break_sec:  0,
            wait: false,
            completed_sessions: Vec::new(),
            
            _stream, 
            stream_handle,
            buzzer_sink: None,
        
        }
    }
}

// This is the main UI interface
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pomodoro Timer");

            ui.label("Study time:");
            ui.text_edit_singleline(&mut self.study_time);
            
            ui.label("Break time:");
            ui.text_edit_singleline(&mut self.break_time);

            ui.separator();

            ui.label(egui::RichText::new(&self.timer).size(60.0));

            if ui.button("Start Timer").clicked() {
               let study = self.study_time.trim().parse::<u64>();
               let brk = self.break_time.trim().parse::<u64>();
                //stops timer if running when cliked
                if let Some(sink) = &self.buzzer_sink {
                    sink.stop();
                }
                    self.buzzer_sink = None;

               match (study, brk)  {
                   (Ok(s),Ok(b)) if s > 0 && b > 0 =>{
                    self.study_sec   = s * 60;
                    self.break_sec   = b * 60;
                    self.running     = true;
                    self.is_break    = false;
                    self.start_time  = Some(Instant::now());
                   }
                   _=>{
                    self.timer = String::from("Enter positive numbers")
                   }
               }
            }
            if self.wait {
                if ui.button("Start Break").clicked() {
                    self.is_break = true;
                    self.wait = false;
                    self.running = true;
                    self.start_time = Some(Instant::now());
                    self.timer = String::from("Break started!");

                    //stops timer if running when cliked
                    if let Some(sink) = &self.buzzer_sink {
                        sink.stop();
                    }
                    self.buzzer_sink = None;
                    
                }
            }
            if self.running{
                if let Some(start) = self.start_time{
                    let duration = if self.is_break{
                        std::time::Duration::from_secs(self.break_sec)
                    } else{
                        std::time::Duration::from_secs(self.study_sec)
                    };

                    let passed = start.elapsed();

                    if passed >= duration{
                        self.running = false;
                        self.start_time = None;
                        if !self.is_break{
                            self.wait = true;
                            
                            self.play_buzzer();

                            //calc time again (replace with function)
                            let mins = passed.as_secs() / 60;

                            //records session:
                            let session_num = self.completed_sessions.len()+1;
                            self.completed_sessions.push(format!("session {}: {} minute(s) worked", session_num, mins));
                        
                            self.timer = String::from("Ready to start your break?");
                        
                        } else{
                            self.is_break = false;
                            self.wait = false;
                            self.play_buzzer();
                            self.timer = String::from("Break time's up. Ready to study Again?");
                        } 

                    } else{
                        let remaining = duration - passed;
                        let mins = remaining.as_secs() / 60;
                        let secs = remaining.as_secs() % 60;
                        self.timer = format!("{}:{:02}",mins,secs);
                        ctx.request_repaint();
                    }
                }
            }
            ui.separator();
            ui.heading("completed Sessions");

                        for session in &self.completed_sessions{
                            ui.label(session);
                        }
        });
    }
}

//play buzzer function
impl App{
    fn play_buzzer(&mut self) {
    if let Some(sink) = &self.buzzer_sink {
        sink.stop();
    }

    let sink = match Sink::try_new(&self.stream_handle) {
        Ok(s) => s,
        Err(e) => { eprintln!("Sink error: {}", e); return; }
    };

    let file = match File::open("assets/buzzer.wav") {
        Ok(f) => f,
        Err(e) => { eprintln!("File error: {}", e); return; }
    };

    let source = match Decoder::new(BufReader::new(file)) {
        Ok(s) => s,
        Err(e) => { eprintln!("Decoder error: {}", e); return; }
    };

    sink.append(source);
    self.buzzer_sink = Some(sink);
}
}

// starts the eframe
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Work Timer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}