use eframe::egui;
use egui::epaint;
use std::io::Write;
use std::path::Path;
use std::fs::File;

struct Generator {
	executable_path: String,
	name: String,
	icon_path: String,
	comment: String,
}

impl Generator {
	fn new() -> Self {
		Self {
			executable_path: String::new(),
			name: String::new(),
			icon_path: String::new(),
			comment: String::new(),
		}
	}

	fn generate(&self, filepath: &Path) -> Result<(), String> {
		if !Path::new(&self.executable_path.clone()).exists() {
			return Err(format!("{} doesn't exists", self.executable_path));
		}
		if !Path::new(&self.icon_path.clone()).exists() {
			return Err(format!("{} doesn't exists", self.icon_path));
		}
		if self.name.is_empty() {
			return Err("Name is empty".to_owned());
		}

		if let Ok(mut file) = File::create(filepath) {
			if let Err(e) = file.write_all(format!(
				"[Desktop Entry]
				Version=1.0
				Type=Application
				Name={}
				Exec={}
				Comment={}",
				self.name,
				self.executable_path,
				self.comment
			).as_bytes()) {
				return Err(format!("Failed to write to {}: {e}", filepath.display()));
			}
			else {
				return Ok(());
			}
		}

		Err(format!("{} couldn't be created, missing priviledge? try running with 'sudo'", filepath.display()))
	}
}

fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 200.0]),
		..Default::default()
	};

	let mut generator = Generator::new();
	let mut error_text = String::new();

	eframe::run_simple_native("Desktop Shortcut Generator", options, move |ctx, _frame| {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.visuals_mut().override_text_color = Some(epaint::Color32::WHITE);
			
			ui.horizontal(|ui| {
				ui.label("Executable: ");
				ui.text_edit_singleline(&mut generator.executable_path);
				if ui.button("Browse").clicked() {
					if let Some(path) = rfd::FileDialog::new().pick_file() {
						generator.executable_path = path.display().to_string();
					}
				}
			});

			ui.horizontal(|ui| {
				ui.label("Name: ");
				ui.add(egui::TextEdit::singleline(&mut generator.name).desired_width(f32::INFINITY)).changed();
			});

			ui.horizontal(|ui| {
				ui.label("Icon: ");
				ui.text_edit_singleline(&mut generator.icon_path);
				if ui.button("Browse").clicked() {
					if let Some(path) = rfd::FileDialog::new().pick_file() { // todo: maybe add filter for png, jpg, ico and executables
						generator.icon_path = path.display().to_string();
					}
				}
			});

			ui.horizontal(|ui| {
				ui.label("Comment: ");
				ui.add(egui::TextEdit::singleline(&mut generator.comment).desired_width(f32::INFINITY));
			});

			ui.separator();

			ui.horizontal(|ui| {
				if ui.add(egui::Button::new("Generate (current user only)").fill(egui::Color32::DARK_GREEN)).clicked() {
					let home_path_str = home::home_dir();
					if home_path_str.is_none() {
						error_text = "Couldn't get linux home dir of the current user -> Are you running linux?".to_string();
					}
					else {
						if let Err(e) = generator.generate(&Path::new(&home_path_str.unwrap()).join(format!(".local/share/applications/{}.desktop", generator.name))) {
							error_text = e;
						}
					}					
				}
				if ui.add(egui::Button::new("Generate (global)").fill(egui::Color32::DARK_RED)).clicked() {
					if let Err(e) = generator.generate(Path::new(&format!("/usr/share/applications/{}.desktop", generator.name))) {
						error_text = e;
					}
				}
			});
		});


		if !error_text.is_empty() {
			egui::Window::new("Error")
			.show(ctx, |ui| {
				ui.visuals_mut().override_text_color = Some(epaint::Color32::RED);
				ui.label(&error_text);
				ui.visuals_mut().override_text_color = Some(epaint::Color32::WHITE);
				if ui.button("Close").clicked() {
					error_text.clear();
				}
			});
		}
	})
}