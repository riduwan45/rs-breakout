use crate::{
    components::{
        Has,
    },
    game::{
        MAX_ENTITIES,
        Game,
    }
};

const QUERY: u32 = Has::ControlBlock as u32 | Has::Collide as u32;

pub fn sys_control_block(game: &mut Game, delta: f32) {
    for i in 0..MAX_ENTITIES {
        if (game.world[i] & QUERY) == QUERY {
            update(game, i);
        }
    }

}

fn update(game: &mut Game, entity: usize) {
    if let (
        Some(collide),
    ) = (
        game.collide[entity],
    ) {

        if let Some(_) = collide.collision {
            game.destroy(entity);
        }
    }
}
