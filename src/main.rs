use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(CompeteApp::new(cc))));
}

#[derive(Default)]
struct CompeteApp {
    players: Vec<String>,
    player_edit_result: String,
}

impl CompeteApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("Compete-inator");
            egui::Window::new("Players").show(ctx, |pui| {
                for player in &self.players {
                    pui.label(player.clone());
                }
                let response = pui.add(
                    egui::TextEdit::singleline(&mut self.player_edit_result)
                        .hint_text("Enter player name...")
                );
                if response.lost_focus() && pui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.players.push(self.player_edit_result.clone());
                    self.player_edit_result = "".to_string();
                }
            });
        });
    }
}