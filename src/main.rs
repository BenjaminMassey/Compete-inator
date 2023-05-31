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
    id: i32,
    name: String,
}

impl Player {
    fn new(player_name: String, player_id: i32) -> Self {
        Player {
            id: player_id,
            name: player_name.clone(),
        }
    }
}

#[derive(Default, Clone)]
struct MatchComponent {
    player: Player,
    score: i32,
}

#[derive(Clone)]
struct Match {
    id: i32,
    components: Vec<MatchComponent>,
}

impl Match {
    fn new(match_id: i32) -> Self {
        Match {
            id: match_id,
            components: vec![],
        }
    }
    fn new_with_players(players: &Vec<Player>, match_id: i32) -> Self {
        let mut c = vec![];
        for player in players {
            c.push(
                MatchComponent {
                    player: player.clone(),
                    score: 0,
                }
            );
        }
        Match {
            id: match_id,
            components: c,
        }
    }
}

#[derive(Default)]
struct CompeteApp {
    player_count: i32,
    players: Vec<Player>,
    match_count: i32,
    matches: Vec<Match>,
    player_edit_result: String,
}

impl CompeteApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

}

fn delete_player_by_name(vector: &Vec<Player>, player_name: String) -> Vec<Player>{
    let mut result: Vec<Player> = vec![];
    for item in vector {
        if item.name != player_name.clone() {
            result.push(item.clone());
        }
    }
    result
}

fn delete_player_by_id(vector: &Vec<Player>, player_id: i32) -> Vec<Player>{
    let mut result: Vec<Player> = vec![];
    for item in vector {
        if item.id != player_id {
            result.push(item.clone());
        }
    }
    result
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut players = self.players.clone();
        let mut matches = self.matches.clone();

        egui::CentralPanel::default()
            .show(ctx, |cui| {
                cui.heading("Compete-inator");
                cui.separator();
                if cui.button("Create Match").clicked() {
                     matches.push(Match::new(self.match_count));
                     self.match_count += 1;
                }
                egui::Window::new("Players").show(ctx, |pui| {
                    for player in &(players.clone()) {
                        pui.horizontal(|hui| {
                            hui.label(player.name.clone());
                            if hui.button("🗑").clicked() {
                                players = delete_player_by_id(&(players.clone()), player.id);
                            }
                        });

                    }
                    let response = pui.add(
                        egui::TextEdit::singleline(&mut self.player_edit_result)
                            .hint_text("Enter player name...")
                    );
                    if response.lost_focus() && pui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        players.push(
                            Player::new(
                                self.player_edit_result.clone(),
                                self.player_count,
                            )
                        );
                        self.player_count += 1;
                        self.player_edit_result = "".to_string();
                    }
                }
            );

            for mat in &(matches.clone()) {
                egui::Window::new(format!("Match {}", mat.id))
                    .show(ctx, |mui| {
                        mui.label("Test");
                    }
                );
            }
        });

        self.players = players;
        self.matches = matches;
    }
}