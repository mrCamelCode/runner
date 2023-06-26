use std::rc::Rc;

use thomas::{
    Alignment, GameCommand, GameCommandsArg, Identity, IntCoords2d, Query, QueryResult,
    QueryResultList, Rgb, System, SystemsGenerator, Text, UiAnchor, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    add_defeat_text, add_paused_text, add_start_playing_text, add_victory_text,
    components::{GameManager, Player},
    DEFEAT_TEXT_NAME, DISTANCE_MARKER_PIECE_NAME, EVENT_DEFEAT, EVENT_GAME_PAUSE_STATE_CHANGE,
    EVENT_RESTART, EVENT_VICTORY, PAUSED_TEXT_NAME, PLAYER_DISPLAY, START_PLAYING_TEXT_NAME,
    VICTORY_TEXT_NAME,
};

const SCORE_TAG_ID: &str = "score-tag";
const LIVES_TAG_ID: &str = "lives-tag";

pub struct HudSystemsGenerator {}
impl SystemsGenerator for HudSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], add_tags)),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new().has::<Player>(),
                        Query::new()
                            .has_where::<Identity>(|id| &id.id == SCORE_TAG_ID)
                            .has::<Text>(),
                        Query::new()
                            .has_where::<Identity>(|id| &id.id == LIVES_TAG_ID)
                            .has::<Text>(),
                    ],
                    update_tags,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has_where::<GameManager>(|gm| gm.is_playing()),
                        Query::new()
                            .has_where::<Identity>(|id| &id.name == START_PLAYING_TEXT_NAME),
                    ],
                    remove_start_playing_text,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has_where::<GameManager>(|gm| !gm.is_game_over()),
                        Query::new().has_where::<Identity>(|id| &id.name == VICTORY_TEXT_NAME),
                        Query::new().has_where::<Identity>(|id| &id.name == DEFEAT_TEXT_NAME),
                    ],
                    remove_game_over_text,
                ),
            ),
            (
                EVENT_GAME_PAUSE_STATE_CHANGE,
                System::new(
                    vec![
                        Query::new().has::<GameManager>(),
                        Query::new().has_where::<Identity>(|id| &id.name == PAUSED_TEXT_NAME),
                    ],
                    update_paused_text,
                ),
            ),
            (EVENT_VICTORY, System::new(vec![], make_victory_text)),
            (EVENT_DEFEAT, System::new(vec![], make_defeat_text)),
        ]
    }
}

fn add_tags(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(Text {
            anchor: UiAnchor::BottomLeft,
            justification: Alignment::Left,
            value: String::from(""),
            offset: IntCoords2d::zero(),
            background_color: None,
            foreground_color: Some(Rgb::white()),
        }),
        Box::new(Identity {
            id: String::from(SCORE_TAG_ID),
            name: String::from(""),
        }),
    ]));

    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
        Box::new(Text {
            anchor: UiAnchor::BottomRight,
            justification: Alignment::Right,
            value: String::from(""),
            offset: IntCoords2d::zero(),
            background_color: None,
            foreground_color: Some(Rgb::white()),
        }),
        Box::new(Identity {
            id: String::from(LIVES_TAG_ID),
            name: String::from(""),
        }),
    ]));

    add_start_playing_text(Rc::clone(&commands));
}

fn update_tags(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [game_manager_results, player_results, score_tag_results, lives_tag_results, ..] =
        &results[..]
    {
        let player = player_results.get_only::<Player>();
        let game_manager = game_manager_results.get_only::<GameManager>();

        let mut score_tag = score_tag_results.get_only_mut::<Text>();
        let mut lives_tag = lives_tag_results.get_only_mut::<Text>();

        score_tag.value = format!("Score: {}", game_manager.score);
        lives_tag.value = format!(
            "Lives: {}",
            (0..player.lives)
                .map(|_| String::from(PLAYER_DISPLAY))
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

fn remove_start_playing_text(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [playing_game_manager_results, start_playing_text_results, ..] = &results[..] {
        if !playing_game_manager_results.is_empty() {
            for start_playing_text_result in start_playing_text_results {
                commands.borrow_mut().issue(GameCommand::DestroyEntity(
                    *start_playing_text_result.entity(),
                ));
            }
        }
    }
}

fn update_paused_text(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [game_manager_results, paused_text_results, ..] = &results[..] {
        let game_manager = game_manager_results.get_only::<GameManager>();

        if game_manager.is_paused() {
            add_paused_text(Rc::clone(&commands));
        } else {
            for paused_text_result in paused_text_results {
                commands
                    .borrow_mut()
                    .issue(GameCommand::DestroyEntity(*paused_text_result.entity()));
            }
        }
    }
}

fn make_victory_text(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    add_victory_text(Rc::clone(&commands));
}

fn make_defeat_text(_: Vec<QueryResultList>, commands: GameCommandsArg) {
    add_defeat_text(Rc::clone(&commands));
}

fn remove_game_over_text(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [not_game_over_game_manager_results, victory_text_results, defeat_text_results, ..] =
        &results[..]
    {
        if !not_game_over_game_manager_results.is_empty() {
            let destroy = |result: &QueryResult| {
                commands
                    .borrow_mut()
                    .issue(GameCommand::DestroyEntity(*result.entity()));
            };

            victory_text_results.iter().for_each(destroy);
            defeat_text_results.iter().for_each(destroy);
        }
    }
}
