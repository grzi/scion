use scion::{
    core::{
        components::maths::transform::Transform,
        resources::{
            inputs::{inputs_controller::InputsController, types::KeyCode},
            time::Timers,
        },
    },
    legion::{system, world::SubWorld, Query},
};

use crate::{
    components::{Bloc, BlocKind, BLOC_SIZE, BOARD_WIDTH},
    resources::{TetrisResource, TetrisState},
};

#[system]
pub fn move_piece(
    #[resource] inputs: &InputsController,
    #[resource] timers: &mut Timers,
    #[resource] tetris: &mut TetrisResource,
    world: &mut SubWorld,
    query: &mut Query<(&mut Bloc, &mut Transform)>,
) {
    handle_acceleration(inputs, timers);

    let movement_timer = timers
        .get_timer("action_reset_timer")
        .expect("Missing a mandatory timer in the game : action_reset_timer");

    let movement = read_movements_actions(inputs);
    if movement_timer.ended() {
        let should_move = movement != 0 && {
            let mut res = true;
            let mut static_values: Vec<(i32, i32)> = Vec::new();
            let mut piece_values: Vec<(i32, i32)> = Vec::new();
            for (bloc, transform) in query.iter_mut(world) {
                let t = (
                    (transform.translation().x() / BLOC_SIZE) as i32,
                    (transform.translation().y() / BLOC_SIZE) as i32,
                );
                match bloc.kind {
                    BlocKind::Moving => piece_values.push(t),
                    _ => static_values.push(t),
                };
            }

            for (x, y) in piece_values.iter() {
                for (xx, yy) in static_values.iter() {
                    if y == yy && *x == (xx - movement) as i32 {
                        res = false;
                        break;
                    }
                }
                if x + movement == 0 || x + movement == (BOARD_WIDTH + 1) as i32 {
                    res = false;
                    break;
                }
            }

            res
        };

        if should_move {
            movement_timer.reset();
            if let TetrisState::MOVING(x, y) = tetris.state {
                tetris.state = TetrisState::MOVING(x + movement, y);
            };
            for (bloc, transform) in query.iter_mut(world) {
                match bloc.kind {
                    BlocKind::Moving => {
                        transform.append_translation(movement as f32 * BLOC_SIZE, 0.);
                    }
                    _ => {}
                };
            }
        }
    }
}

fn handle_acceleration(inputs: &InputsController, timers: &mut Timers) {
    if inputs.key_pressed(&KeyCode::Down) {
        timers
            .get_timer("piece")
            .expect("Missing a mandatory timer in the game : piece")
            .change_cycle(0.025);
    } else {
        timers
            .get_timer("piece")
            .expect("Missing a mandatory timer in the game : piece")
            .change_cycle(0.5);
    }
}

fn read_movements_actions(inputs: &InputsController) -> i32 {
    ({
        if inputs.key_pressed(&KeyCode::Left) {
            -1
        } else {
            0
        }
    }) + ({
        if inputs.key_pressed(&KeyCode::Right) {
            1
        } else {
            0
        }
    })
}
