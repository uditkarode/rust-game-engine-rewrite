use components::{
    drawable::{self, Drawable},
    position::Position,
};
use draw::draw_drawable;
use drawables::player::player_canvas_size;
use raylib::prelude::*;
use resources::{
    elapsed_time::ElapsedTime, projectile_speed::ProjectileSpeed, score::Score,
    window_size::WindowSize,
};
use systems::*;

use bevy_ecs::prelude::*;
use raylib::ffi::{KeyboardKey, TraceLogLevel};

mod components;
mod drawables;
mod resources;
mod systems;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    let mut world = World::default();

    let window_size = WindowSize {
        width: 1280.0,
        height: 720.0,
    };

    // resources
    {
        world.insert_resource(window_size.clone());
        world.insert_resource(ElapsedTime::default());
        world.insert_resource(ProjectileSpeed(5.0));
        world.insert_resource(Score(0));
    }

    let (mut rl, thread) = raylib::init()
        .size(window_size.width as i32, window_size.height as i32)
        .title("Space Invaders")
        .build();

    // raylib confs
    {
        rl.set_target_fps(120);
        rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
        rl.set_trace_log(TraceLogLevel::LOG_NONE);
    }

    let mut schedule = Schedule::default();

    // systems
    {
        schedule.add_systems((
            (spawn_enemy::spawn_enemy, enemy_fire::enemy_fire),
            (
                apply_velocity::apply_velocity,
                (
                    handle_window_collisions::handle_window_collisions,
                    handle_projectile_collisions::handle_projectile_collisions,
                ),
                remove_oob_entities::remove_oob_entities,
            )
                .chain(),
        ));
    }

    // spawns
    {
        world.spawn((
            components::identifiers::Player,
            components::position::Position { x: 400.0, y: 640.0 },
            components::collision_shape::CollisionShape::Rectangle(
                player_canvas_size().x,
                player_canvas_size().y,
            ),
            drawable::Drawable {
                canvas_size: player_canvas_size(),
                kind: drawable::DrawableKind::Player,
            },
            components::velocity::Velocity::default(),
        ));
    }

    while !rl.window_should_close() {
        // RaylibHandler-needing logic
        {
            handle_input::handle_input(&mut world, &window_size, &rl);
            track_time::track_time(&mut world, &rl);
        }

        schedule.run(&mut world);

        // get all drawables
        let mut drawables = Vec::new();
        for (drawable, position) in world.query::<(&Drawable, &Position)>().iter(&world) {
            drawables.push((drawable.clone(), position.clone()));
        }

        let mut textures = Vec::new();
        for (drawable, position) in drawables {
            let mut render_texture = rl
                .load_render_texture(
                    &thread,
                    drawable.canvas_size.x as u32,
                    drawable.canvas_size.y as u32,
                )
                .unwrap();

            {
                let mut rl_ref = &mut rl;
                let mut d = rl_ref.begin_texture_mode(&thread, &mut render_texture);
                draw_drawable(&mut world, &drawable, &mut d);
            }

            textures.push((render_texture, position));
        }

        // draw all textures
        let mut d = rl.begin_drawing(&thread);
        for (_, (texture, pos)) in textures.iter().enumerate() {
            d.draw_texture(texture.texture(), pos.x as i32, pos.y as i32, Color::WHITE);
        }

        // clear at the end of frame draw
        d.clear_background(Color::BLACK);
    }

    Ok(())
}
