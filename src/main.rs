use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Compete-inator",
        native_options,
        Box::new(|cc| Box::new(CompeteApp::new(cc))),
    )
    .unwrap();
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
            name: player_name,
        }
    }
}

#[derive(Default, Clone)]
struct MatchComponent {
    player: Player,
    #[allow(unused)]
    score: i32, // TODO: use this
}

#[derive(Clone)]
struct Match {
    id: i32,
    components: Vec<MatchComponent>,
    winner: Option<Player>,
}

impl Match {
    fn new(match_id: i32) -> Self {
        Match {
            id: match_id,
            components: vec![],
            winner: None,
        }
    }
    fn _new_with_players(players: &Vec<Player>, match_id: i32) -> Self {
        let mut c = vec![];
        for player in players {
            c.push(MatchComponent {
                player: player.clone(),
                score: 0,
            });
        }
        Match {
            id: match_id,
            components: c,
            winner: None,
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
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn _get_player_by_name(vector: &Vec<Player>, player_name: String) -> Option<Player> {
    for item in vector {
        if item.name == player_name.clone() {
            return Some(item.clone());
        }
    }
    None
}

fn delete_player_by_id(vector: &Vec<Player>, player_id: i32) -> Vec<Player> {
    let mut result: Vec<Player> = vec![];
    for item in vector {
        if item.id != player_id {
            result.push(item.clone());
        }
    }
    result
}

fn _players_to_strings(players: &Vec<Player>) -> Vec<String> {
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut players = self.players.clone();
        let mut matches = self.matches.clone();

        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("Compete-inator");
            cui.separator();
            if cui.button("Create Match").clicked() && !players.is_empty() {
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
                        .hint_text("Enter player name..."),
                );
                if response.lost_focus()
                    && pui.input(|i| i.key_pressed(egui::Key::Enter))
                    && !self.player_edit_result.is_empty()
                {
                    players.push(Player::new(
                        self.player_edit_result.clone(),
                        self.player_count,
                    ));
                    self.player_count += 1;
                    self.player_edit_result = "".to_string();
                }
            });

            for mat in &mut matches {
                let mut components = mat.components.clone();
                let mut winner: Option<Player> = mat.winner.clone();

                egui::Window::new(format!("Match {}", mat.id)).show(ctx, |mui| {
                    if winner.is_some() {
                        let versus: String = components
                            .iter()
                            .map(|c| c.player.name.clone())
                            .collect::<Vec<String>>()
                            .join(" vs ");
                        mui.label(versus);
                        mui.label(format!("{} won!", winner.clone().unwrap().name));
                    } else {
                        let mut alternatives: Vec<&str> = vec![];
                        for player in &players {
                            alternatives.push(&(player.name));
                        }
                        mui.horizontal(|hui| {
                            egui::ComboBox::from_id_source(mat.id)
                                .selected_text(alternatives[self.selected])
                                .show_index(hui, &mut self.selected, alternatives.len(), |i| {
                                    alternatives[i]
                                });

                            let skip = repeat_component(
                                &(components.clone()),
                                players[self.selected].clone(),
                            );
                            if hui.button("Add").clicked() && !skip {
                                components.push(MatchComponent {
                                    player: players[self.selected].clone(),
                                    score: 0,
                                });
                            }
                        });
                        for component in &components {
                            mui.horizontal(|hui| {
                                hui.label(component.player.name.clone());
                                if winner.is_none() && hui.button("Declare Winner").clicked() {
                                    winner = Some(component.player.clone());
                                }
                            });
                        }
                    }
                });

                mat.components = components;
                mat.winner = winner;
            }
        });

        self.players = players;
        self.matches = matches;
    }
}
