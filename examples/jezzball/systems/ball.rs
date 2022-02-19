use legion::*;
use rand::{thread_rng, Rng};
use scion::core::{
    components::{
        animations::Animations,
        maths::{
            collider::{Collider, ColliderMask},
            transform::Transform,
        },
    },
    resources::audio::{Audio, PlayConfig},
};

use crate::main_scene::{Ball, BallDirection};
use crate::utils::ball_bounce_effect;

#[system(for_each)]
pub fn ball_control(
    ball: &mut Ball,
    transform: &mut Transform,
    collider: &Collider,
    animations: &mut Animations,
    #[resource] audio_player: &mut Audio,
) {
    // Init the ball direction
    if ball.direction.is_none() {
        let direction = random_direction();
        animations.loop_animation(direction.to_string().as_str());
        ball.direction = Some(direction);
    }

    collider.collisions().iter().for_each(|collision| {
        if let ColliderMask::Custom(name) = collision.mask() {
            let new_direction = match (
                name.as_str(),
                &ball.direction.as_ref().expect("A ball has collided without any direction WTF ?!"),
            ) {
                ("BORDER_LEFT", BallDirection::TopLeft) => Some(BallDirection::TopRight),
                ("BORDER_LEFT", BallDirection::BottomLeft) => Some(BallDirection::BottomRight),
                ("BORDER_RIGHT", BallDirection::TopRight) => Some(BallDirection::TopLeft),
                ("BORDER_RIGHT", BallDirection::BottomRight) => Some(BallDirection::BottomLeft),
                ("BORDER_TOP", BallDirection::TopRight) => Some(BallDirection::BottomRight),
                ("BORDER_TOP", BallDirection::TopLeft) => Some(BallDirection::BottomLeft),
                ("BORDER_BOTTOM", BallDirection::BottomRight) => Some(BallDirection::TopRight),
                ("BORDER_BOTTOM", BallDirection::BottomLeft) => Some(BallDirection::TopLeft),
                ("BORDER_CUSTOM_VERTICAL", e) => match e {
                    BallDirection::TopLeft => {
                        if collision.coordinates().x() + 8. <= transform.translation().x() {
                            Some(BallDirection::TopRight)
                        } else {
                            None
                        }
                    }
                    BallDirection::TopRight => {
                        if collision.coordinates().x() > transform.translation().x() {
                            Some(BallDirection::TopLeft)
                        } else {
                            None
                        }
                    }
                    BallDirection::BottomLeft => {
                        if collision.coordinates().x() + 8. <= transform.translation().x() {
                            Some(BallDirection::BottomRight)
                        } else {
                            None
                        }
                    }
                    BallDirection::BottomRight => {
                        if collision.coordinates().x() > transform.translation().x() {
                            Some(BallDirection::BottomLeft)
                        } else {
                            None
                        }
                    }
                },
                ("BORDER_CUSTOM_HORIZONTAL", e) => match e {
                    BallDirection::TopLeft => {
                        if collision.coordinates().y() + 8. <= transform.translation().y() {
                            Some(BallDirection::BottomLeft)
                        } else {
                            None
                        }
                    }
                    BallDirection::TopRight => {
                        if collision.coordinates().y() + 8. <= transform.translation().y() {
                            Some(BallDirection::BottomRight)
                        } else {
                            None
                        }
                    }
                    BallDirection::BottomLeft => {
                        if collision.coordinates().y() > transform.translation().y() {
                            Some(BallDirection::TopLeft)
                        } else {
                            None
                        }
                    }
                    BallDirection::BottomRight => {
                        if collision.coordinates().y() > transform.translation().y() {
                            Some(BallDirection::TopRight)
                        } else {
                            None
                        }
                    }
                },
                (_a, _b) => None,
            };

            if let Some(direction) = new_direction {
                animations.stop_all_animation(true);
                animations.loop_animation(direction.to_string().as_str());
                ball.direction = Some(direction);
                audio_player.play(ball_bounce_effect(), PlayConfig::default());
            }
        }
    })
}

fn random_direction() -> BallDirection {
    match thread_rng().gen_range(0..4) {
        0 => BallDirection::TopRight,
        1 => BallDirection::TopLeft,
        2 => BallDirection::BottomLeft,
        _ => BallDirection::BottomRight,
    }
}
