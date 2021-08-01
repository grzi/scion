use scion::core::components::animations::{Animation, AnimationModifier};
use std::time::Duration;

pub fn explode() -> Animation {
    Animation::new(Duration::from_millis(2000),
                   vec![
                       AnimationModifier::sprite(vec![64,64,64,64,64, 25,25,25,25,25, 38,38,38,38,38, 104, 105, 106, 107, 108, 109, 110, 111, 112], 12)
                   ], false)
}