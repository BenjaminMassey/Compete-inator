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
    fn new(player_name: &str, player_id: i32) -> Self {
        Player {
            id: player_id,
            name: player_name.to_owned(),
        }
    }
}

#[derive(Default, Clone)]
struct MatchComponent {
    player: Player,
}

impl MatchComponent {
    fn new(player: &Player) -> Self {
        MatchComponent {
            player: player.clone(),
        }
    }
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
}

#[derive(Default)]
struct CompeteApp {
    next_player_id: i32,
    players: Vec<Player>,
    next_match_id: i32,
    matches: Vec<Match>,
    player_edit_result: String,
    selected: usize,
}

impl CompeteApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn delete_player_by_id(players: &mut Vec<Player>, player_id: i32) {
    players.retain(|p| p.id != player_id);
}

fn repeat_component(components: &[MatchComponent], player: &Player) -> bool {
    components.iter().any(|mc| &mc.player == player)
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("Compete-inator");
            cui.separator();
            if cui.button("Create Match").clicked() && !self.players.is_empty() {
                self.matches.push(Match::new(self.next_match_id));
                self.next_match_id += 1;
            } // TODO: messaging of some sort when no players failure
            egui::Window::new("Players").show(ctx, |pui| {
                for player in &(self.players.clone()) {
                    pui.horizontal(|hui| {
                        hui.label(player.name.clone());
                        if hui.button("🗑").clicked() {
                            delete_player_by_id(&mut self.players, player.id);
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
                    self.players.push(Player::new(
                        &self.player_edit_result,
                        self.next_player_id,
                    ));
                    self.next_player_id += 1;
                    self.player_edit_result = "".to_string();
                }
            });

            for mat in &mut self.matches {
                egui::Window::new(format!("Match {}", mat.id)).show(ctx, |mui| {
                    if mat.winner.is_some() {
                        let versus: String = mat.components
                            .iter()
                            .map(|c| c.player.name.clone())
                            .collect::<Vec<String>>()
                            .join(" vs ");
                        mui.label(versus);
                        mui.label(format!("{} won!", mat.winner.clone().unwrap().name));
                    } else {
                        let mut alternatives: Vec<&str> = vec![];
                        for player in &self.players {
                            alternatives.push(&(player.name));
                        }
                        mui.horizontal(|hui| {
                            egui::ComboBox::from_id_source(mat.id)
                                .selected_text(alternatives[self.selected])
                                .show_index(hui, &mut self.selected, alternatives.len(), |i| {
                                    alternatives[i]
                                });

                            let skip = repeat_component(
                                &mat.components,
                                &self.players[self.selected],
                            );
                            if hui.button("Add").clicked() && !skip {
                                mat.components.push(MatchComponent::new(
                                    &self.players[self.selected]
                                ));
                            }
                        });
                        for component in &mat.components {
                            mui.horizontal(|hui| {
                                hui.label(component.player.name.clone());
                                if mat.winner.is_none() && hui.button("Declare Winner").clicked() {
                                    mat.winner = Some(component.player.clone());
                                }
                            });
                        }
                    }
                });
            }
        });
    }
}
