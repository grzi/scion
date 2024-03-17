use hecs::QueryMut;

use scion::core::components::material::Material;

use scion::core::components::maths::transform::{Transform, TransformBuilder};
use scion::core::components::ui::ui_image::UiImage;
use scion::core::world::{GameData, World};
use scion::utils::file::app_base_path_join;

/*
    This file is responsible of :
        - exposing the system that will handle parallax backgrounds
        - exposing the function that can be called to add into the world the parallax backgrounds
*/

// Adding entities
pub fn add_parallax_backgrounds(game_data: &mut GameData) {
    let background1 = game_data.assets_mut().register_material(Material::Texture(app_base_path_join("examples/starlight-1961/assets/background.png")));
    let background2 = game_data.assets_mut().register_material(Material::Texture(app_base_path_join("examples/starlight-1961/assets/background2.png")));
    let background3 = game_data.assets_mut().register_material(Material::Texture(app_base_path_join("examples/starlight-1961/assets/background3.png")));
    let mut backgrounds = vec![
        (BackgroundParallax { speed: -0.55, y_reset: -800., y_start: 0. }, background1),
        (BackgroundParallax { speed: -0.55, y_reset: 0., y_start: 800. }, background1),
        (BackgroundParallax { speed: -0.4, y_reset: -800., y_start: 0. }, background2),
        (BackgroundParallax { speed: -0.4, y_reset: 0., y_start: 800. }, background2),
        (BackgroundParallax { speed: -0.1, y_reset: -800., y_start: 0. }, background3),
        (BackgroundParallax { speed: -0.1, y_reset: 0., y_start: 800. }, background3),
    ];
    backgrounds.drain(0..backgrounds.len()).for_each(|b| {
        game_data.push((
            UiImage::new(352., 400.),
            b.1,
            TransformBuilder::new().with_xy(0., b.0.y_start).with_scale(2.0).build(),
            b.0
        )
        );
    });
}

// Component struct
pub struct BackgroundParallax {
    pub speed: f32,
    pub y_start: f32,
    pub y_reset: f32,
}


// System function
pub fn apply_parallax_system(game_data: &mut GameData) {
    for (_, (transform, parallax)) in backgrounds_query(game_data) {
        if transform.global_translation().y() <= parallax.y_reset {
            transform.set_y(parallax.y_start);
        } else {
            transform.append_y(parallax.speed);
        }
    }
}

fn backgrounds_query(game_data: &mut GameData) -> QueryMut<(&mut Transform, &BackgroundParallax)> {
    game_data.query_mut::<(&mut Transform, &BackgroundParallax)>()
}

