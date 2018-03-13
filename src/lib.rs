//! Integration of amethyst and rhusics

#![warn(missing_docs)]

pub extern crate collision;
pub extern crate rhusics_core;
pub extern crate rhusics_ecs;

extern crate amethyst_core;
extern crate amethyst_renderer;
extern crate shrev;
extern crate specs;

pub use self::bundle::{BasicPhysicsBundle2, BasicPhysicsBundle3, SpatialPhysicsBundle2,
                       SpatialPhysicsBundle3};
pub use self::default::{DefaultBasicPhysicsBundle2, DefaultBasicPhysicsBundle3,
                        DefaultSpatialPhysicsBundle2, DefaultSpatialPhysicsBundle3,
                        PoseTransformSyncSystem2, PoseTransformSyncSystem3};
pub use self::pick::{pick_ray, pick_ray_ndc};
pub use self::sync::{time_sync, AsTransform, Convert};
pub use self::system::PoseTransformSyncSystem;

mod bundle;
mod default;
mod pick;
mod sync;
mod system;
