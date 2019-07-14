use crate::{
    math::{
        vec3::Vec3,
        quat::Quat,
    },
    game::Game,
};

pub struct Blueprint {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<Vec3>,
    pub using: Vec<Box<Fn(&mut Game, usize) -> ()>>,
}
