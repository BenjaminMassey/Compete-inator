use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Compete-inator",
        native_options,
        Box::new(|cc| Box::new(CompeteApp::new(cc)))
    );
}


#[derive(Default, Clone)]
struct Player {
    name: String,
    score: i32,
}

impl Player {
    fn new(player_name: String) -> Self {
        Player {
            name: player_name.clone(),
            score: 0,
        }
    }
}

#[derive(Default)]
struct CompeteApp {
    players: Vec<Player>,
    player_edit_result: String,
}

impl CompeteApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

}

fn deletePlayerByName(vector: &Vec<Player>, player_name: String) -> Vec<Player>{
    let mut result: Vec<Player> = vec![];
    for item in vector {
        if item.name != player_name.clone() {
            result.push(item.clone());
        }
    }
    result
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut players = (&self.players).to_vec();

        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("Compete-inator");
            egui::Window::new("Players").show(ctx, |pui| {
                for player in &(players.clone()) {
                    pui.horizontal(|hui| {
                        hui.label(player.name.clone());
                        if hui.button("🗑").clicked() {
                            players = deletePlayerByName(
                                &(players.clone()),
                                player.name.clone()
                            );
                        }
                    });

                }
                let response = pui.add(
                    egui::TextEdit::singleline(&mut self.player_edit_result)
                        .hint_text("Enter player name...")
                );
                if response.lost_focus() && pui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    players.push(
                        Player::new(self.player_edit_result.clone())
                    );
                    self.player_edit_result = "".to_string();
                }
            });
        });

        self.players = players;
    }
}