//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

mod data_structs;

use eframe::egui;
use std::process::Command;
use rfd::FileDialog;

fn main() {
	//println!("{}", sh3_prob_map.len());
	let options = eframe::NativeOptions::default();
	
	let mut my_app: data_structs::MyApp = data_structs::MyApp::default();
	my_app.init();

    
    
    eframe::run_native(
        "Silent Hill Rustomizer",
        options,
        Box::new(|_cc| Box::new(my_app)),
    );


}

impl eframe::App for data_structs::MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Silent Hill Rustomizer");
           
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
           
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
           
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }

			if ui.button("Find SH3").clicked(){
				if let Some(path) = FileDialog::new()
                    .add_filter("Executable", &["exe"])
                    .pick_file()
                {
                    self.sh3_exe_name = path.file_name().unwrap().to_str().unwrap().to_owned();
                    self.sh3_path = path.display().to_string();
                }
			}

			if ui.button("Click to start SH3").clicked() {
                let sh3_process = Command::new(&self.sh3_path)
				.spawn()
				.expect("failed to execute process");
            }

			if ui.button("Update Probs").clicked() {
                self.set_probability();
            }
           
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.label(format!("The SH3 exe path: {}", self.sh3_path));
        });
    }
}
