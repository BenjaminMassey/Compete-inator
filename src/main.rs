mod idents;
use idents::*;

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

use std::sync::Mutex;
static PLAYER_IDENT_GENERATOR: Mutex<PlayerIdentGenerator> =
    Mutex::new(PlayerIdentGenerator::new());

impl Player {
    fn new(player_name: &str) -> Self {
        let mut idg = PLAYER_IDENT_GENERATOR.lock().unwrap();
        let player_ident = idg.next().unwrap();
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
        MatchComponent {
            player,
        }
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
    next_player_id: u32,
    players: Vec<Player>,
    next_match_id: u32,
    matches: Vec<Match>,
    player_edit_result: String,
    selected: PlayerIdent,
}

impl CompeteApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn delete_player_by_id(players: &mut Vec<Player>, player_id: PlayerIdent) {
    players.retain(|p| p.ident != player_id);
}

fn get_player_by_id(players: &[Player], player_id: PlayerIdent) -> &Player {
    players.iter().find(|p| p.ident == player_id).unwrap()
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
                self.matches.push(Match::new(self.next_match_id));
                self.next_match_id += 1;
            } // TODO: messaging of some sort when no players failure
            egui::Window::new("Players").show(ctx, |pui| {
                let mut dead_players = vec![];
                for player in &self.players {
                    pui.horizontal(|hui| {
                        hui.label(&player.name);
                        if hui.button("🗑").clicked() {
                            dead_players.push(player.ident);
                        }
                    });
                }
                for player_id in dead_players {
                    delete_player_by_id(&mut self.players, player_id);
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
                    ));
                    self.next_player_id += 1;
                    self.player_edit_result = "".to_string();
                }
            });

            for mat in &mut self.matches {
                egui::Window::new(format!("Match {}", mat.ident)).show(ctx, |mui| {
                    if let Some(winner) = mat.winner {
                        let versus: String = mat.components
                            .iter()
                            .map(|c| {
                                let player = get_player_by_id(
                                    &self.players,
                                    c.player,
                                );
                                player.name.clone()
                            })
                            .collect::<Vec<String>>()
                            .join(" vs ");
                        mui.label(versus);
                        let winner = get_player_by_id(&self.players, winner);
                        mui.label(format!("{} won!", winner.name));
                    } else {
                        let mut alternatives: Vec<&Player> = vec![];
                        let mut selected = None;
                        for (i, player) in self.players.iter().enumerate() {
                            alternatives.push(player);
                            if player.ident == self.selected {
                                selected = Some(i);
                            }
                        }
                        let mut selected = selected.unwrap();
                        mui.horizontal(|hui| {
                            egui::ComboBox::from_id_source(mat.ident)
                                .selected_text(&alternatives[selected].name)
                                .show_index(hui, &mut selected, alternatives.len(), |i| {
                                    &alternatives[i].name
                                });
                            self.selected = alternatives[selected].ident;
                            let skip = repeat_component(
                                &mat.components,
                                self.selected,
                            );
                            if hui.button("Add").clicked() && !skip {
                                mat.components.push(MatchComponent::new(
                                    self.selected
                                ));
                            }
                        });
                        for component in &mat.components {
                            mui.horizontal(|hui| {
                                let player = get_player_by_id(&self.players, component.player);
                                hui.label(&player.name);
                                if hui.button("Declare Winner").clicked() {
                                    assert!(mat.winner.is_none(), "too many winners");
                                    mat.winner = Some(player.ident);
                                }
                            });
                        }
                    }
                });
            }
        });
    }
}
