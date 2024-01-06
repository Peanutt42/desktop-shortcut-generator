use eframe::egui;
use egui::epaint;

fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 200.0]),
		..Default::default()
	};

	let mut executable_path = String::new();
	let mut name = String::new();
	let mut icon_path = String::new();
	let mut version = String::new();
	let mut comment = String::new();

	eframe::run_simple_native("Desktop Shortcut Generator", options, move |ctx, _frame| {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.visuals_mut().override_text_color = Some(epaint::Color32::WHITE);
			
			ui.horizontal(|ui| {
				ui.label("Executable: ");
				ui.text_edit_singleline(&mut executable_path);
				if ui.button("Browse").clicked() {
					if let Some(path) = rfd::FileDialog::new().pick_file() {
						executable_path = path.display().to_string();
					}
				}
			});

			ui.horizontal(|ui| {
				ui.label("Name: ");
				ui.add(egui::TextEdit::singleline(&mut name).desired_width(f32::INFINITY)).changed();
			});

			ui.horizontal(|ui| {
				ui.label("Icon: ");
				ui.text_edit_singleline(&mut icon_path);
				if ui.button("Browse").clicked() {
					if let Some(path) = rfd::FileDialog::new().pick_file() { // todo: maybe add filter for png, jpg, ico and executables
						icon_path = path.display().to_string();
					}
				}
			});

			ui.horizontal(|ui| {
				ui.label("Version: ");
				ui.add(egui::TextEdit::singleline(&mut version).desired_width(f32::INFINITY));
			});

			ui.horizontal(|ui| {
				ui.label("Comment: ");
				ui.add(egui::TextEdit::singleline(&mut comment).desired_width(f32::INFINITY));
			});
		});
	})
}