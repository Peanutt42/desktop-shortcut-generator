use eframe::egui;
use egui::epaint;
use std::io::Write;
use std::path::Path;
use std::fs::File;

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
				ui.label(&generator.executable_path);
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
				ui.label(&generator.icon_path.clone().unwrap_or("Not specified".to_string()));
				if ui.button("Browse").clicked() {
					if let Some(path) = rfd::FileDialog::new().pick_file() { // todo: maybe add filter for png, jpg, ico and executables
						generator.icon_path = Some(path.display().to_string());
					}
					else {
						generator.icon_path = None;
					}
				}
				if generator.icon_path.is_some() {
					if ui.button("Remove").clicked() {
						generator.icon_path = None;
					}
				}
			});

			ui.horizontal(|ui| {
				ui.label("Comment: ");
				ui.add(egui::TextEdit::singleline(&mut generator.comment).desired_width(f32::INFINITY));
			});

			ui.separator();

			ui.horizontal(|ui| {
				match dirs::home_dir() {
					Some(home_dir) => {
						if home_dir.display().to_string() != "/root" {
							if ui.add(egui::Button::new("Generate (current user only)").fill(egui::Color32::DARK_GREEN)).clicked() {
								if let Err(e) = generator.generate(&home_dir.join(&format!(".local/share/applications/{}.desktop", &generator.name))) {
									error_text = e;
								}
							}
						}
						else {
							ui.colored_label(egui::Color32::RED, "Can't get the users home dir in sudo mode");
						}
					},
					None => {},
				}
				if ui.add(egui::Button::new("Generate (global)").fill(egui::Color32::DARK_RED)).clicked() {
					if let Err(e) = generator.generate(Path::new(&format!("/usr/share/applications/{}.desktop", &generator.name))) {
						error_text = e;
					}
				}
			});
		});


		if !error_text.is_empty() {
			egui::Window::new("Error")
			.show(ctx, |ui| {
				ui.colored_label(egui::Color32::RED, &error_text);
				if ui.button("Close").clicked() {
					error_text.clear();
				}
			});
		}
	})
}

struct Generator {
	executable_path: String,
	name: String,
	icon_path: Option<String>,
	comment: String,
}

impl Generator {
	fn new() -> Self {
		Self {
			executable_path: String::new(),
			name: String::new(),
			icon_path: None,
			comment: String::new(),
		}
	}

	fn generate(&self, filepath: &Path) -> Result<(), String> {
		if !Path::new(&self.executable_path.clone()).exists() {
			return Err(format!("Executable {} doesn't exists", self.executable_path));
		}
		if let Some(icon_path) = &self.icon_path {
			if !Path::new(&icon_path.clone()).exists() {
				return Err(format!("Icon {} doesn't exists", icon_path));
			}
		}
		if self.name.is_empty() {
			return Err("Name is empty".to_owned());
		}

		match File::create(filepath) {
			Ok(mut file) => {
				let mut file_content = "[Desktop Entry]\nVersion=1.0\nType=Application\n".to_string();
				file_content.push_str(format!("Name={}\n", self.name).as_str());				
				file_content.push_str(format!("Exec={}\n", self.executable_path).as_str());
				if let Some(icon_path) = &self.icon_path {
					file_content.push_str(format!("Icon={}\n", icon_path).as_str());
				}
				if !self.comment.is_empty() {
					file_content.push_str(format!("Comment={}\n", self.comment).as_str());
				}

				if let Err(e) = file.write_all(file_content.as_bytes()) {
					Err(format!("Failed to write to {}: {e}", filepath.display()))
				}
				else {
					Ok(())
				}
			},
			Err(e) => {
				match e.kind() {
					std::io::ErrorKind::PermissionDenied => Err(format!("{} couldn't be created due to missing privilidge -> try running with 'sudo'", filepath.display())),
					_ => Err(format!("{} couldn't be created: {e}", filepath.display()))
				}
			},
		}
	}
}