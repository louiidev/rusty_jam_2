use std::cmp::{max, min};

use hecs::World;

use crate::components::{Player, Position};

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    for (_entity, (player, pos)) in world.query_mut::<(&Player, &mut Position)>() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}
