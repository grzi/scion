use std::time::Duration;
use hecs::Entity;

use scion::core::components::animations::{Animation, AnimationModifier, Animations};
use scion::core::components::Hide;
use scion::core::components::maths::camera::Camera;
use scion::core::components::maths::collider::{Collider, ColliderMask, ColliderType};
use scion::core::components::maths::coordinates::Coordinates;
use scion::core::components::maths::hierarchy::Parent;
use scion::core::components::maths::transform::{Transform, TransformBuilder};
use scion::core::components::tiles::atlas::data::{TilemapAtlas, TileObjectClass};
use scion::core::components::tiles::atlas::importer::load_tilemap;
use scion::core::components::tiles::sprite::Sprite;
use scion::core::components::tiles::tileset::Tileset;
use scion::core::resources::audio::PlayConfig;
use scion::core::resources::inputs::types::{Input, KeyCode};
use scion::core::resources::time::TimerType;
use scion::core::scene::Scene;
use scion::core::world::{GameData, World};
use scion::utils::file::app_base_path_join;
use scion::utils::maths::Vector;

use crate::scripts::ship::{add_ship, Ship};

pub const ACTIVE_LVL_FLAG: &str = "current_scene_is_menu";
pub const CURRENT_LEVEL: &str = "current-level";

#[derive(Default)]
pub struct LevelScene {
    ship: Option<Entity>,
    explosion: Option<Entity>,
    exploded: bool,
    global_scale_modifier: f32
}

impl Scene for LevelScene {
    fn on_start(&mut self, data: &mut GameData) {
        self.compute_pixel_perfect_scaling(data);

        data.game_state_mut().set_bool(ACTIVE_LVL_FLAG, true);
        let lvl = data.game_state().get_text(CURRENT_LEVEL).unwrap();
        let (atlas, _entity) = load_tilemap(data, &lvl, TransformBuilder::new().with_scale(self.global_scale_modifier).build());


        add_colliders(data, &atlas, self.global_scale_modifier);
        let ship_entity = add_ship(data, &atlas, self.global_scale_modifier);

        let camera_transform = Transform::from_xy(-318., -388.);
        let dimension = data.window().dimensions();
        let dpi = data.window().dpi() as f32;
        data.push((
            Camera::new(dimension.0 as f32 / dpi, dimension.1 as f32 / dpi),
            camera_transform,
            Parent(ship_entity)
        ));
        self.ship = Some(ship_entity);
        let _ = data.timers().add_timer("s", TimerType::Cyclic, 0.2);
        let _ = data.timers().add_timer("reset", TimerType::Manual, 0.5);
    }

    fn on_update(&mut self, data: &mut GameData) {
        let cycle_sound = data.timers().get_timer("s").unwrap().cycle();
        if !self.exploded {
            let up = data.inputs().input_pressed(&Input::Key(KeyCode::Up));
            let left = data.inputs().input_pressed(&Input::Key(KeyCode::Left));
            let right = data.inputs().input_pressed(&Input::Key(KeyCode::Right));
            if up {
                let (s, a, t) = data.entry_mut::<(&mut Ship, &mut Animations, &Transform)>(self.ship.unwrap()).expect("");
                if !a.animation_running("booster") {
                    a.loop_animation("booster");
                }
                let mut angle_modified = t.global_angle().to_degrees() % 380.;
                if angle_modified < 0. {
                    angle_modified = 360. + angle_modified;
                }

                let y_force = compute_y_force(angle_modified);
                let x_force = compute_x_force(angle_modified);
                s.y_force += 0.04 * y_force;
                s.x_force += 0.04 * x_force;
                s.is_landed = false;
                if cycle_sound > 0 {
                    /*let _r = data
                        .audio()
                        .play(app_base_path_join("examples/starlight-1961/assets/fire.ogg"),
                              PlayConfig { volume: 0.1, looped: false, category: None });*/
                }
            } else {
                let (s, a, sp) = data.entry_mut::<(&mut Ship, &mut Animations, &mut Sprite)>(self.ship.unwrap()).expect("");
                a.stop_all_animation(true);
                sp.set_tile_nb(0);
                if !s.is_landed {
                    s.y_force += 0.02;
                }
            }

            if left {
                let (s, t) = data.entry_mut::<(&mut Ship, &mut Transform)>(self.ship.unwrap()).expect("");
                if !s.is_landed {
                    t.append_angle(-0.04);
                }
            }

            if right {
                let (s, t) = data.entry_mut::<(&mut Ship, &mut Transform)>(self.ship.unwrap()).expect("");
                if !s.is_landed {
                    t.append_angle(0.04);
                }
            }

            let mut should_explose = false;
            let mut tr = Transform::default();

            {

                let (ship, t, c) = data.entry_mut::<(&mut Ship, &mut Transform, &Collider)>(self.ship.unwrap()).expect("");
                t.append_vector(Vector::new(ship.x_force, ship.y_force));
                if !ship.is_landed && c.is_colliding() {
                    should_explose = true;
                    tr = TransformBuilder::new().with_xy(t.global_translation().x() -16., t.global_translation().y()-16.).with_scale(self.global_scale_modifier).build();
                }

            }

            if should_explose {
                data.timers().get_timer("reset").unwrap().reset();
                let _ = data.add_components(self.ship.unwrap(), (Hide, ));
                self.exploded = true;
                let explose_ref = data.assets_mut().register_tileset(Tileset::new("explose".to_string(),
                                                                                  app_base_path_join("examples/starlight-1961/assets/explosion.png"),
                                                                                  4, 1, 26, 24));

                self.explosion = Some(data.push((
                    Sprite::new(0),
                    explose_ref,
                    Animations::single("explode", Animation::running(Duration::from_millis(300), vec![AnimationModifier::sprite(vec![0, 1, 2, 3], 3)])),
                    tr,
                )));
                /*let _r = data
                    .audio()
                    .play(app_base_path_join("examples/starlight-1961/assets/explosion.ogg"),
                          PlayConfig { volume: 0.1, looped: false, category: None });*/
            }
        } else if self.explosion.is_some() {
            let finished_explosion = {
                let animation = data.entry_mut::<&mut Animations>(self.explosion.unwrap()).expect("");
                !animation.any_animation_running()
            };
            if finished_explosion {
                let _r = data.remove(self.explosion.take().unwrap());
            }
        } else {
            let ended = data.timers().get_timer("reset").unwrap().ended();
            if ended {
                data.scene_controller().switch::<LevelScene>();
            }
        }
    }

    fn on_stop(&mut self, data: &mut GameData) {
        data.game_state_mut().set_bool(ACTIVE_LVL_FLAG, false);
        data.clear();
    }
}

impl LevelScene {
    fn compute_pixel_perfect_scaling(&mut self, data: &mut GameData) {
        let desired_scaling = 4.0;
        let dpi = data.window().dpi();
        let real_scaling = if dpi % 1.0 != 0. {
            (desired_scaling / (desired_scaling * dpi)) * desired_scaling
        } else {
            desired_scaling
        };
        self.global_scale_modifier = real_scaling as f32;
    }
}

fn add_colliders(data: &mut GameData, atlas: &TilemapAtlas, global_scale_modifier: f32) {
    atlas.get_objects().iter().filter(|o| o.get_class() == &TileObjectClass::Collider)
        .for_each(|collider| {
            if collider.is_rect() {
                let rec = collider.get_rect();
                data.push((
                    Collider::new(ColliderMask::Landscape, vec![], ColliderType::Rectangle((rec.width() * global_scale_modifier) as usize, (rec.height() * global_scale_modifier) as usize)),
                    TransformBuilder::new().with_xy(collider.get_position().x() * global_scale_modifier, collider.get_position().y() * global_scale_modifier).build()
                ));
            } else {
                let pol: Vec<Coordinates> = collider.get_polygon().iter().map(|c| Coordinates::new(c.x() * global_scale_modifier, c.y() * global_scale_modifier)).collect();
                data.push((
                    Collider::new(ColliderMask::Landscape, vec![], ColliderType::Polygon(pol)),
                    TransformBuilder::new().with_xy(collider.get_position().x() * global_scale_modifier, collider.get_position().y() * global_scale_modifier).build()
                ));
            }
        });
}

fn compute_y_force(angle_degrees: f32) -> f32 {
    if angle_degrees > 270. {
        -1. * (angle_degrees - 270.) / 90.
    } else if angle_degrees < 90. {
        -1. * (90. - angle_degrees) / 90.
    } else if angle_degrees > 90. && angle_degrees <= 180. {
        (angle_degrees - 90.) / 90.
    } else if angle_degrees > 180. && angle_degrees <= 270. {
        (90. - (angle_degrees - 180.)) / 90.
    } else {
        0.
    }
}

fn compute_x_force(angle_degrees: f32) -> f32 {
    if angle_degrees > 270. {
        -1. * (1. - (angle_degrees - 270.) / 90.)
    } else if angle_degrees < 90. {
        angle_degrees / 90.
    } else if angle_degrees > 90. && angle_degrees <= 180. {
        1. - (angle_degrees - 90.) / 90.
    } else if angle_degrees > 180. && angle_degrees <= 270. {
        -1. * (1. - (270. - angle_degrees) / 90.)
    } else {
        0.
    }
}

