use thomas::{
    GameCommandsArg, Priority, Query, QueryResultList, System, SystemsGenerator, TerminalCamera,
    TerminalTransform, EVENT_UPDATE,
};

use crate::components::FollowCamera;

pub struct FollowCameraSystemsGenerator {}
impl SystemsGenerator for FollowCameraSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![(
            EVENT_UPDATE,
            System::new_with_priority(
                Priority::lowest(),
                vec![
                    Query::new()
                        .has::<FollowCamera>()
                        .has::<TerminalTransform>(),
                    Query::new()
                        .has_where::<TerminalCamera>(|cam| cam.is_main)
                        .has::<TerminalTransform>(),
                ],
                update_positions,
            ),
        )]
    }
}

fn update_positions(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [fixed_entities_results, main_camera_results, ..] = &results[..] {
        let main_cam_transform = main_camera_results.get_only::<TerminalTransform>();

        for fixed_entity_result in fixed_entities_results {
            let follow_cam = fixed_entity_result.components().get::<FollowCamera>();
            let mut transform = fixed_entity_result
                .components()
                .get_mut::<TerminalTransform>();

            transform.coords =
                follow_cam.base_position + main_cam_transform.coords + follow_cam.offset;
        }
    }
}
