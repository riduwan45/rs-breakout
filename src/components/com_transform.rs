use crate::{
    math::{
        mat4::Mat4,
        vec3::Vec3,
        quat::Quat,
    },
    game::Game,
};

#[derive(Clone, Copy)]
pub struct Transform {
    // Matrix relative to the world
    pub world: Mat4,
    // World to self matrix
    pub self_mat: Mat4,

    // local translation relative to the parent
    pub translation: Vec3,
    // local rotation relative to the parent
    pub rotation: Quat,
    // local scale relative to the parent
    pub scale: Vec3,

    pub entity_id: usize,

    // TODO: in original Goodluck Trannsform::parent is a Transform
    // instance. Will this work as well?
    pub parent: Option<usize>,
    // TODO: This needs a little bit more thinking.
    // pub children: Vec<Transform>,
    pub dirty: bool,
}

impl Transform {
    pub fn empty() -> Transform {
        Transform {
            world: Mat4::empty(),
            self_mat: Mat4::empty(),
            translation: Vec3::empty(),
            rotation: Quat::empty(),
            scale: Vec3::empty(),

            entity_id: 0,
            parent: None,
            // children: Default::default(),
            dirty: true,
        }
    }

    pub fn new(translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) -> fn(&mut Game, usize) -> () {
        let initial_translation = match translation {
            Some(translation_value) => { translation_value },
            None => { Vec3::empty() }
        };

        fn transform_mixin(game: &mut Game, entity: usize) -> () {
            game.world[entity] = 2;

            game.transform[entity] = Transform {
                world: Mat4::empty(),
                self_mat: Mat4::empty(),
                translation: initial_translation,
                rotation: Quat::empty(),
                scale: Vec3::empty(),

                entity_id: entity,
                parent: None,
                // children: Default::default(),
                dirty: true,
            };
        }

        transform_mixin
    }
}