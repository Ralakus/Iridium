use super::na as na;
use super::specs::prelude::*;
use super::specs_hierarchy::Parent as HParent;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transfrom {
    pub local:  na::geometry::Transform3<f32>,
    pub global: na::geometry::Transform3<f32>,
}

impl Default for Transfrom {
    fn default() -> Self {
        Transfrom {
            local:  na::geometry::Transform3::identity(),
            global: na::geometry::Transform3::identity()
        }
    }
}

impl Component for Transfrom {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}





/// Component for defining a parent entity.
///
/// The entity with this component *has* a parent, rather than *is* a parent.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Parent {
    /// The parent entity
    pub entity: Entity,
}

impl Component for Parent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl HParent for Parent {
    fn parent_entity(&self) -> Entity {
        self.entity
    }
}