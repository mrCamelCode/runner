use thomas::{Component, IntCoords2d};

#[derive(Component)]
pub struct FixedToCamera {
    pub base_position: IntCoords2d,
    pub offset: IntCoords2d,
}
