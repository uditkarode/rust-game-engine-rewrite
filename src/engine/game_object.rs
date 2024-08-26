use std::collections::HashMap;

use raylib::ffi::KeyboardKey;

use crate::utils::generic::TextureDrawer;

use super::{
    constants::DEFAULT_COLLISION_DAMPING_FACTOR,
    types::{ObjectInfo, XYPair},
};

// collision shape
pub enum CollisionShape {
    Circle(f32),
}

// game object common
#[derive(Default, Clone)]
pub struct GameObjectCommon {
    pub coords: XYPair,
    pub velocities: XYPair,
    pub interested_keys: Vec<KeyboardKey>,
    pub object_info: Option<ObjectInfo>,
}

// game object
pub trait GameObject {
    fn common(&mut self) -> &mut GameObjectCommon;

    fn canvas_size(&self) -> XYPair;

    fn weight_factor(&self) -> f32;

    fn bounciness(&self) -> f32 {
        DEFAULT_COLLISION_DAMPING_FACTOR
    }

    fn collision_shape(&self) -> CollisionShape;

    fn draw(&self, d: &mut TextureDrawer);

    fn handle_input(&mut self, _key_status: HashMap<KeyboardKey, bool>) {}
}
