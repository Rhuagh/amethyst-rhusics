extern crate amethyst_core;
extern crate collision;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate shrev;
extern crate specs;

pub use self::bundle::{BasicPhysicsBundle2, BasicPhysicsBundle3, DefaultBasicPhysicsBundle2,
                       DefaultBasicPhysicsBundle3};
pub use self::sync::{time_sync, AsTransform, Convert};
pub use self::system::{PoseTransformSyncSystem, PoseTransformSyncSystem2, PoseTransformSyncSystem3};

mod bundle;
mod system;
mod sync;
