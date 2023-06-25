use thomas::{
    Alignment, GameCommand, GameCommandsArg, Identity, IntCoords2d, Query, QueryResultList, Rgb,
    System, SystemsGenerator, Text, UiAnchor, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    components::{GameManager, Player},
    DISTANCE_MARKER_PIECE_NAME,
};

const SCORE_TAG_ID: &str = "score-tag";

pub struct HudSystemsGenerator {}
impl SystemsGenerator for HudSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (EVENT_INIT, System::new(vec![], add_tags)),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new()
                            .has_where::<Identity>(|id| &id.id == SCORE_TAG_ID)
                            .has::<Text>(),
                        Query::new().has::<GameManager>(),
                    ],
                    update_tags,
                ),
            ),
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
}

fn update_tags(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [score_tag_results, game_manager_results, ..] = &results[..] {
        let mut score_tag = score_tag_results.get_only_mut::<Text>();
        let game_manager = game_manager_results.get_only::<GameManager>();

        score_tag.value = format!("Score: {}", game_manager.score);
    }
}
