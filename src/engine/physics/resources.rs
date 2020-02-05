use nalgebra::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::DefaultBodySet;
use nphysics2d::object::DefaultColliderSet;
use nphysics2d::world::DefaultGeometricalWorld;
use nphysics2d::world::DefaultMechanicalWorld;
use std::fmt::Debug;

pub struct MyMechanicalWorld(pub DefaultMechanicalWorld<f64>);

impl Default for MyMechanicalWorld {
    fn default() -> Self {
        MyMechanicalWorld {
            0: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
        }
    }
}

impl Debug for MyMechanicalWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct MyGeometricalWorld(pub DefaultGeometricalWorld<f64>);

impl Default for MyGeometricalWorld {
    fn default() -> Self {
        MyGeometricalWorld {
            0: DefaultGeometricalWorld::new(),
        }
    }
}

impl Debug for MyGeometricalWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct MyForceGeneratorSet(pub DefaultForceGeneratorSet<f64>);

impl Default for MyForceGeneratorSet {
    fn default() -> Self {
        MyForceGeneratorSet {
            0: DefaultForceGeneratorSet::new(),
        }
    }
}

impl Debug for MyForceGeneratorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct MyBodySet(pub DefaultBodySet<f64>);

impl Default for MyBodySet {
    fn default() -> Self {
        MyBodySet {
            0: DefaultBodySet::new(),
        }
    }
}

impl Debug for MyBodySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct MyColliderSet(pub DefaultColliderSet<f64>);

impl Default for MyColliderSet {
    fn default() -> Self {
        MyColliderSet {
            0: DefaultColliderSet::new(),
        }
    }
}

impl Debug for MyColliderSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct MyJointConstraintSet(pub DefaultJointConstraintSet<f64>);

impl Default for MyJointConstraintSet {
    fn default() -> Self {
        MyJointConstraintSet {
            0: DefaultJointConstraintSet::new(),
        }
    }
}

impl Debug for MyJointConstraintSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
