mod idents;
use idents::*;

use std::collections::HashMap;
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

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct PlayerIdentType;

type PlayerIdent = Ident<PlayerIdentType>;
type PlayerIdentGenerator = IdentGenerator<PlayerIdentType>;

#[derive(Default, Clone)]
struct Player {
    ident: PlayerIdent,
    name: String,
}

static PLAYER_IDENT_GENERATOR: PlayerIdentGenerator =
    PlayerIdentGenerator::new();

impl Player {
    fn new(player_name: &str) -> Self {
        let player_ident = PLAYER_IDENT_GENERATOR.next_ident();
        Player {
            ident: player_ident,
            name: player_name.to_owned(),
        }
    }
}

#[derive(Default, Clone)]
struct MatchComponent {
    player: PlayerIdent,
}

impl MatchComponent {
    fn new(player: PlayerIdent) -> Self {
        MatchComponent { player }
    }
}

#[derive(Clone)]
struct Match {
    ident: u32,
    components: Vec<MatchComponent>,
    winner: Option<PlayerIdent>,
}

impl Match {
    fn new(match_id: u32) -> Self {
        Match {
            ident: match_id,
            components: vec![],
            winner: None,
        }
    }
}

#[derive(Default)]
struct CompeteApp {
    players: HashMap<PlayerIdent, Player>,
    next_match_id: u32,
    matches: Vec<Match>,
    player_edit_result: String,
    selected: usize,
}

impl CompeteApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
/* TODO: delete
fn delete_player_by_id(players: &mut Vec<Player>, player_id: PlayerIdent) {
    players.retain(|p| p.ident != player_id);
}
*/
fn repeat_component(components: &[MatchComponent], player: PlayerIdent) -> bool {
    components.iter().any(|mc| mc.player == player)
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
                let mut dead_players = vec![];
                for (ident, player) in self.players.clone() {
                    pui.horizontal(|hui| {
                        hui.label(&player.name);
                        if hui.button("🗑").clicked() {
                            dead_players.push(ident);
                        }
                    });
                }
                for player_id in dead_players {
                    self.players.remove(&player_id);
                }
                let response = pui.add(
                    egui::TextEdit::singleline(&mut self.player_edit_result)
                        .hint_text("Enter player name..."),
                );
                if response.lost_focus()
                    && pui.input(|i| i.key_pressed(egui::Key::Enter))
                    && !self.player_edit_result.is_empty()
                {
                    let new_player = Player::new(&self.player_edit_result);
                    self.players.insert(new_player.ident, new_player);
                    self.player_edit_result = "".to_string();
                }
            });

            for mat in &mut self.matches {
                egui::Window::new(format!("Match {}", mat.ident)).show(ctx, |mui| {
                    if let Some(winner) = mat.winner {
                        let versus: String = mat
                            .components
                            .iter()
                            .map(|c| {
                                self.players[&c.player].name.clone()
                            })
                            .collect::<Vec<String>>()
                            .join(" vs ");
                        mui.label(versus);
                        mui.label(format!("{} won!", self.players[&winner].name));
                    } else {
                        mui.horizontal(|hui| {
                            let player_values = &self.players.values().cloned().collect::<Vec<Player>>();
                            egui::ComboBox::from_id_source(mat.ident)
                                .selected_text(&player_values[self.selected].name)
                                .show_index(hui, &mut self.selected, self.players.len(), |i| {
                                    &player_values[i].name
                            });
                            let skip = repeat_component(&mat.components, player_values[self.selected].ident);
                            if hui.button("Add").clicked() && !skip {
                                mat.components.push(MatchComponent::new(player_values[self.selected].ident));
                            }
                        });
                        for component in &mat.components {
                            mui.horizontal(|hui| {
                                let ident = component.player;
                                let player = &self.players[&ident];
                                hui.label(&player.name);
                                if hui.button("Declare Winner").clicked() {
                                    assert!(mat.winner.is_none(), "too many winners");
                                    mat.winner = Some(ident);
                                }
                            });
                        }
                    }
                });
            }
        });
    }
}
