mod idents;
use idents::*;

use eframe::egui;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Deref;

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

#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Player {
    ident: PlayerIdent,
    name: String,
}

static PLAYER_IDENT_GENERATOR: PlayerIdentGenerator = PlayerIdentGenerator::new();

impl Player {
    fn new(player_name: &str) -> Self {
        Player {
            ident: PLAYER_IDENT_GENERATOR.next_ident(),
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

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct MatchIdentType;

type MatchIdent = Ident<MatchIdentType>;
type MatchIdentGenerator = IdentGenerator<MatchIdentType>;

#[derive(Clone)]
struct Match {
    ident: MatchIdent,
    components: Vec<MatchComponent>,
    winner: Option<PlayerIdent>,
}

static MATCH_IDENT_GENERATOR: MatchIdentGenerator = MatchIdentGenerator::new();

impl Match {
    fn new() -> Self {
        Match {
            ident: MATCH_IDENT_GENERATOR.next_ident(),
            components: vec![],
            winner: None,
        }
    }
}

#[derive(Default)]
struct CompeteApp {
    players: HashMap<PlayerIdent, Player>,
    matches: HashMap<MatchIdent, Match>,
    player_edit_result: String,
    selected: HashMap<MatchIdent, usize>,
}

impl CompeteApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn repeat_component(components: &[MatchComponent], player: PlayerIdent) -> bool {
    components.iter().any(|mc| mc.player == player)
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("Compete-inator");
            cui.separator();
            if cui.button("Create Match").clicked() && !self.players.is_empty() {
                let new_match = Match::new();
                let new_ident = new_match.ident;
                self.matches.insert(new_ident, new_match);
                self.selected.insert(new_ident, 0);
            } // TODO: messaging of some sort when no players failure
            egui::Window::new("Players").show(ctx, |pui| {
                let mut dead_players = vec![];
                for ident in self.players.keys().cloned().sorted() {
                    pui.horizontal(|hui| {
                        hui.label(&self.players[&ident].name);
                        if hui.button("🗑").clicked() {
                            dead_players.push(ident);
                            for sel in self.selected.values_mut() {
                                *sel = 0;
                            }
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

            for (mid, mat) in &mut self.matches {
                egui::Window::new(format!("Match {:?}", mid.deref())).show(ctx, |mui| {
                    if let Some(winner) = mat.winner {
                        let versus: String = mat
                            .components
                            .iter()
                            .map(|c| {
                                if self.players.contains_key(&c.player) {
                                    self.players[&c.player].name.clone()
                                } else {
                                    "<DELETED>".to_string()
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(" vs ");
                        mui.label(versus);
                        if self.players.contains_key(&winner) {
                            mui.label(format!("{} won!", self.players[&winner].name));
                        } else {
                            mui.label("This match concluded.");
                        }
                    } else {
                        if !self.players.is_empty() {
                            mui.horizontal(|hui| {
                                let player_values = &self
                                    .players
                                    .values()
                                    .cloned()
                                    .sorted()
                                    .collect::<Vec<Player>>();
                                let mut current_selected = self.selected[mid]; // TODO: shouldn't need this before/after thing
                                egui::ComboBox::from_id_source(mat.ident)
                                    .selected_text(&player_values[self.selected[mid]].name)
                                    .show_index(
                                        hui,
                                        &mut current_selected,
                                        self.players.len(),
                                        |i| &player_values[i].name,
                                    );
                                *self.selected.entry(*mid).or_insert(current_selected) =
                                    current_selected; // TODO: above
                                let skip = repeat_component(
                                    &mat.components,
                                    player_values[self.selected[mid]].ident,
                                );
                                if hui.button("Add").clicked() && !skip {
                                    mat.components.push(MatchComponent::new(
                                        player_values[self.selected[mid]].ident,
                                    ));
                                }
                            });
                        }
                        for component in &mat.components {
                            mui.horizontal(|hui| {
                                let ident = component.player;
                                if self.players.contains_key(&ident) {
                                    hui.label(&self.players[&ident].name);
                                } else {
                                    hui.label("<DELETED>");
                                }
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
