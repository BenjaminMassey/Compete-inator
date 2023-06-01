use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Compete-inator",
        native_options,
        Box::new(|cc| Box::new(CompeteApp::new(cc)))
    );
}


#[derive(Default, Clone, PartialEq)]
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
    selected: usize,
}

impl CompeteApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

}

fn get_player_by_name(vector: &Vec<Player>, player_name: String) -> Option<Player>{
    for item in vector {
        if item.name == player_name.clone() {
            Some(item);
        }
    }
    None
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

fn players_to_strings(players: &Vec<Player>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for player in players {
        result.push(player.name.clone());
    }
    result
}

fn repeat_component(components: &Vec<MatchComponent>, player: Player) -> bool {
    for component in components {
        if component.player.clone() == player.clone() {
            return true;
        }
    }
    false
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut players = self.players.clone();
        let mut matches = self.matches.clone();

        egui::CentralPanel::default()
            .show(ctx, |cui| {
                cui.heading("Compete-inator");
                cui.separator();
                if cui.button("Create Match").clicked() && players.len() > 0 {
                     matches.push(Match::new(self.match_count));
                     self.match_count += 1;
                } // TODO: messaging of some sort when no players failure
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
                    if response.lost_focus() && pui.input(|i| i.key_pressed(egui::Key::Enter))
                        && self.player_edit_result.len() > 0 {
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

            let mut mat_index = 0;
            for mat in &(matches.clone()) {
                let mut components = mat.components.clone();

                egui::Window::new(format!("Match {}", mat.id))
                    .show(ctx, |mui| {
                        let mut alternatives: Vec<&str> = vec![];
                        for player in &players {
                            alternatives.push(&(player.name));
                        }
                        mui.horizontal(
                            |hui| {
                                egui::ComboBox::from_id_source(mat.id)
                                    .selected_text(alternatives[self.selected])
                                    .show_index(
                                        hui,
                                        &mut self.selected,
                                        alternatives.len(),
                                        |i| alternatives[i]
                                    );
                                
                                let skip = repeat_component(&(components.clone()), (&players[self.selected]).clone());
                                if hui.button("Hello").clicked() && !skip {
                                    components.push(
                                            MatchComponent {
                                            player: (&players[self.selected]).clone(),
                                            score: 0,
                                        }
                                    );
                                }
                            }
                        );
                        for component in &components {
                            mui.label(component.player.name.clone());
                        }
                    }
                );

                matches[mat_index].components = components;
                mat_index += 1;
            }
        });

        self.players = players;
        self.matches = matches;
    }
}