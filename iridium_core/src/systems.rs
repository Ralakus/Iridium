#![allow(dead_code)]
#![allow(unused_imports)]
use super::components as components;
use super::specs as specs;
use specs::prelude::*;

struct TransformSystem;

impl<'a> System<'a> for TransformSystem {
    type SystemData = (
        WriteStorage<'a, components::Transfrom>,
        ReadStorage<'a,  components::Parent>
    );

    
    fn run(&mut self, (_trans, _parent): Self::SystemData) {

    }

}