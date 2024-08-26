use std::collections::HashMap;

use raylib::{
    color::Color,
    ffi::KeyboardKey,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    engine::{
        game_object::{CollisionShape, GameObject, GameObjectCommon},
        types::XYPair,
    },
    utils,
};

const KB_X_BOOST: f64 = 0.2;
const KB_Y_BOOST: f64 = 16.0;

pub struct Ball {
    radius: f64,
    diameter: f64,
    color: Color,

    common: GameObjectCommon,
}

impl Ball {
    pub fn new(coords: XYPair, radius: f64, color_hex: &str) -> Self {
        let diameter = radius * 2.0;
        let color = utils::generic::hex_to_color(color_hex);
        let interested_keys = vec![KeyboardKey::KEY_A, KeyboardKey::KEY_D, KeyboardKey::KEY_W];

        let common = GameObjectCommon {
            coords,
            interested_keys,
            ..GameObjectCommon::default()
        };

        Self {
            color,
            radius,
            diameter,

            common,
        }
    }
}

impl GameObject for Ball {
    fn weight_factor(&self) -> f64 {
        0.8
    }

    fn bounciness(&self) -> f64 {
        0.6
    }

    fn collision_shape(&self) -> CollisionShape {
        CollisionShape::Circle(self.radius)
    }

    fn common(&mut self) -> &mut GameObjectCommon {
        &mut self.common
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(
            self.common.coords.x as i32 + self.radius as i32,
            self.common.coords.y as i32 + self.radius as i32,
            self.radius as f32,
            self.color,
        );
    }

    fn handle_input(&mut self, keys: HashMap<KeyboardKey, bool>) {
        if let Some(true) = keys.get(&KeyboardKey::KEY_A) {
            self.common.velocities.x -= KB_X_BOOST;
        }

        if let Some(true) = keys.get(&KeyboardKey::KEY_D) {
            self.common.velocities.x += KB_X_BOOST;
        }

        // jump if we are on the ground AND have 0 or lesser y velocity
        if let Some(true) = keys.get(&KeyboardKey::KEY_W) {
            if let Some(info) = &self.common.object_info {
                if self.common.velocities.y < 0.0
                    && self.common.coords.y + self.diameter == info.window_size.height as f64
                {
                    self.common.velocities.y -= KB_Y_BOOST;
                }
            }
        }
    }
}
