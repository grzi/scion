use std::collections::HashSet;

use legion::{world::SubWorld, EntityStore, IntoQuery};
use scion::{
    core::{
        components::{
            maths::transform::Transform,
            tiles::{
                tilemap::{TileInfos, Tilemap, TilemapInfo},
                tileset::Tileset,
            },
        },
        game_layer::{GameLayer, GameLayerController, SimpleGameLayer},
        legion_ext::ScionResourcesExtension,
        resources::events::{topic::TopicConfiguration, PollConfiguration, SubscriberId},
    },
    legion::{Entity, Resources, World},
    utils::maths::{Dimensions, Position},
};

use crate::{
    inputs_layer::Line,
    main_layer::{init_balls, Ball, JezzBallAssets},
    utils::cases_asset,
};

#[derive(Default)]
pub struct TilemapUpdateLayer {
    tilemap: Option<Entity>,
    subscriber_id: Option<SubscriberId>,
}

impl SimpleGameLayer for TilemapUpdateLayer {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        self.tilemap = Some(add_tilemap(world, resources));
        let _r = resources.events().create_topic("TILEMAP_UPDATE", TopicConfiguration::default());
        self.subscriber_id = Some(
            resources.events().subscribe("TILEMAP_UPDATE", PollConfiguration::default()).unwrap(),
        );
    }

    fn update(&mut self, world: &mut World, resources: &mut Resources) {
        let (mut world_t, mut world_s) = world.split::<&mut Tilemap>();
        let mut tilemap_entry = world_t.entry_mut(*self.tilemap.as_ref().unwrap()).unwrap();
        let tilemap = tilemap_entry.get_component_mut::<Tilemap>().unwrap();
        let mut tilemap_modified = false;
        resources
            .events()
            .poll::<(usize, usize, usize, usize)>(&self.subscriber_id.as_ref().unwrap())
            .unwrap()
            .into_iter()
            .for_each(|(x_min, x_max, y_min, y_max)| {
                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        tilemap_modified = true;
                        tilemap.modify_sprite_tile(Position::new(x, y, 0), 2, &mut world_s);
                    }
                }
            });

        if tilemap_modified {
            let (world_b, mut world_c) = world_s.split::<(&Ball, &Transform)>();

            let ball_pos: Vec<(usize, usize)> = <(&Ball, &Transform)>::query()
                .iter(&world_b)
                .map(|(_, t)| {
                    (
                        (t.translation().x() as usize - 10) / 16,
                        (t.translation().y() as usize - 10) / 16,
                    )
                })
                .collect();
            let mut pathfinded_cases: HashSet<Position> = Default::default();
            ball_pos.iter().for_each(|(pos_x, pos_y)| {
                let mut visited = HashSet::new();
                pathfind_from(Position::new(*pos_x, *pos_y, 0), &world_c, tilemap, &mut visited)
                    .drain()
                    .for_each(|e| {
                        pathfinded_cases.insert(e);
                    });
            });

            for i in 0..68 {
                for j in 0..38 {
                    let tmp_pos = Position::new(i, j, 0);
                    if !pathfinded_cases.contains(&tmp_pos) {
                        tilemap.modify_sprite_tile(tmp_pos, 2, &mut world_c);
                    }
                }
            }

            if pathfinded_cases.len() < 150 {
                let mut controller = resources.get_mut::<GameLayerController>().unwrap();
                controller.replace_layer(
                    "TILEMAP_LAYER",
                    GameLayer::weak::<TilemapUpdateLayer>("TILEMAP_LAYER"),
                );
            }
        }
    }

    fn on_stop(&mut self, world: &mut World, resources: &mut Resources) {
        world.remove(self.tilemap.unwrap());
        let mut to_delete: Vec<Entity> =
            <(Entity, &Line)>::query().iter(world).map(|(e, _)| *e).collect();
        to_delete.drain(0..to_delete.len()).for_each(|e| {
            world.remove(e);
        });

        let assets = resources.get::<JezzBallAssets>().unwrap();
        init_balls(world, &assets);
    }
}

fn pathfind_from(
    pos: Position,
    world: &SubWorld,
    tilemap: &Tilemap,
    visited: &mut HashSet<Position>,
) -> HashSet<Position> {
    let mut res = HashSet::new();
    let sides = compute_sides(&pos);
    sides.iter().for_each(|side_pos| {
        if !visited.contains(side_pos) {
            visited.insert(side_pos.clone());
            let sprite = tilemap.retrieve_sprite_tile(side_pos, &world).unwrap_or(2);
            if sprite != 2 {
                res.insert(side_pos.clone());
                let recursive_call_result =
                    pathfind_from(side_pos.clone(), world, tilemap, visited);
                recursive_call_result.iter().for_each(|e| {
                    res.insert(e.clone());
                });
            }
        }
    });
    res
}

fn compute_sides(pos: &Position) -> Vec<Position> {
    let mut res = Vec::new();

    if pos.x() > 0 {
        res.push(Position::new(pos.x() - 1, pos.y(), 0));
    }

    if pos.x() < 67 {
        res.push(Position::new(pos.x() + 1, pos.y(), 0));
    }

    if pos.y() > 0 {
        res.push(Position::new(pos.x(), pos.y() - 1, 0));
    }

    if pos.y() < 37 {
        res.push(Position::new(pos.x(), pos.y() + 1, 0));
    }
    res
}

fn add_tilemap(world: &mut World, resources: &mut Resources) -> Entity {
    let cases_asset = resources.assets().register_tileset(Tileset::new(cases_asset(), 3, 2, 16));
    let infos = TilemapInfo::new(
        Dimensions::new(68, 38, 1),
        Transform::from_xyz(10., 10., 10),
        cases_asset,
    );
    Tilemap::create(infos, world, |position| {
        let line = position.y() % 2;
        let column = position.x() % 2;
        TileInfos::new(Some(line * 3 + column), None)
    })
}
