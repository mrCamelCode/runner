use thomas::{Component, IntCoords2d};

#[derive(Component)]
pub struct FollowCamera {
    pub base_position: IntCoords2d,
    pub offset: IntCoords2d,
}
