use ambient_api::{
    components::core::{
        app::main_scene,
        game_objects::player_camera,
        player::player,
        primitives::cube,
        rendering::color,
        transform::{lookat_center, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub async fn main() -> EventResult {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    spawn_query(player()).bind(move |players| {
        // For each player joining, spawn a random colored box somewhere
        for _ in players {
            Entity::new()
                .with_merge(make_transformable())
                .with_default(cube())
                .with(translation(), rand::random())
                .with(color(), rand::random())
                .spawn();
        }
    });

    EventOk
}
