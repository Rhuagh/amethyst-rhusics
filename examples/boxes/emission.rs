use std::marker;
use std::time::Instant;

use amethyst::assets::Handle;
use amethyst::core::math as na;
use amethyst::core::{/*GlobalTransform,*/ Transform};
use amethyst::ecs::prelude::{Entities, Entity, Join, ReadExpect, System, WriteStorage};
use amethyst::renderer::{Material, Mesh};
use amethyst_rhusics::{AsTransform, Convert};
//use shred_derive::SystemData;
use shred::SystemData;
use shred::World;
use shred::ResourceId;
use cgmath::{Array, EuclideanSpace, InnerSpace, Rotation, Zero};
use collision::{Bound, ComputeBound, Primitive, Union};
//use rand::Rand;
use rand::prelude::*;
use rhusics_core::{
    BodyPose, CollisionMode, CollisionShape, CollisionStrategy, Inertia, Mass, PhysicalEntity,
    Pose, Velocity,
};
use rhusics_ecs::PhysicalEntityParts;

use super::{Emitter, Graphics, ObjectType};
use rand::distributions::Standard;

/// Primitive emission system.
///
/// Will spawn new primitives regularly, based on the `Emitter`s in the `World`.
///
/// Showcase how to use `RigidBodyParts` to setup new collidable entities
///
/// ### Type parameters:
///
/// - `P`: Collision primitive (see `collision::primitive` for more information)
/// - `B`: Bounding volume (usually `Aabb2`, `Aabb3` or `Sphere`)
/// - `R`: Rotational quantity (`Basis2` or `Quaternion`)
/// - `A`: Angular velocity quantity (`Scalar` or `Vector3`)
/// - `I`: Inertia tensor (`Scalar` or `Matrix3`)
pub struct EmissionSystem<P, B, R, A, I> {
    primitive: P,
    m: marker::PhantomData<(B, R, A, I)>,
}

impl<P, B, R, A, I> EmissionSystem<P, B, R, A, I> {
    pub fn new(primitive: P) -> Self {
        Self {
            primitive,
            m: marker::PhantomData,
        }
    }
}

impl<'a, P, B, R, A, I> System<'a> for EmissionSystem<P, B, R, A, I>
where
    B: Bound<Point = P::Point> + Union<B, Output = B> + Clone + Send + Sync + 'static,
    P: Primitive + ComputeBound<B> + Clone + Send + Sync + 'static,
    P::Point:
        EuclideanSpace<Scalar = f32> + Convert<Output = na::Vector3<f32>> + Send + Sync + 'static,
    <P::Point as EuclideanSpace>::Diff: /*Distribution<<P::Point as EuclideanSpace>::Diff> +*/ InnerSpace + Array + Send + Sync + 'static,
    R: Rotation<P::Point> + Convert<Output = na::UnitQuaternion<f32>> + Send + Sync + 'static,
    A: Clone + Copy + Zero + Send + Sync + 'static,
    I: Inertia + Send + Sync + 'static,
    Standard: Distribution<<P::Point as EuclideanSpace>::Diff>,
{
    type SystemData = (
        Entities<'a>,
        EmitterParts<'a, P, B, R, A, I>,
        ReadExpect<'a, Graphics>,
        WriteStorage<'a, Emitter<P::Point>>,
    );

    fn run(&mut self, (entities, mut parts, graphics, mut emitters): Self::SystemData) {
        let now = Instant::now();
        for emitter in (&mut emitters).join() {
            if (now - emitter.last_emit) > emitter.emission_interval {
                emit_box::<P, B, R, A, I>(
                    entities.create(),
                    &mut parts,
                    graphics.mesh.clone(),
                    &emitter,
                    &self.primitive,
                );
                emitter.last_emit = now.clone();
                emitter.emitted += 1;
            }
        }
    }
}

fn emit_box<P, B, R, A, I>(
    entity: Entity,
    parts: &mut EmitterParts<P, B, R, A, I>,
    mesh: Handle<Mesh>,
    emitter: &Emitter<P::Point>,
    primitive: &P,
) where
    B: Bound<Point = P::Point> + Union<B, Output = B> + Clone + Send + Sync + 'static,
    P: Primitive + ComputeBound<B> + Clone + Send + Sync + 'static,
    P::Point:
        EuclideanSpace<Scalar = f32> + Convert<Output = na::Vector3<f32>> + Send + Sync + 'static,
    <P::Point as EuclideanSpace>::Diff: /*Distribution<<P::Point as EuclideanSpace>::Diff> +*/ InnerSpace + Array + Send + Sync + 'static,
    R: Rotation<P::Point> + Convert<Output = na::UnitQuaternion<f32>> + Send + Sync + 'static,
    A: Clone + Copy + Zero + Send + Sync + 'static,
    I: Inertia + Send + Sync + 'static,
    Standard: Distribution<<P::Point as EuclideanSpace>::Diff>,
{
    /*use rand;*/
    /*use rand::Rng;*/
    /*use rand::rngs::StdRng;*/

    // Determine the size of the box based on the size of our
    // primitive (which we set when we bundled the box simulation bundle).
    let primitive_bound = primitive.compute_bound();
    // conversion no longer converts the z component, since we
    // use that in 2D for ordering and hiding. We have to supply
    // a default z value (even in 3D)
    let base_vector = na::Vector3::<f32>::zero();
    let bound_min:na::Vector3<f32> = primitive_bound.min_extent().convert(base_vector);
    let bound_max:na::Vector3<f32> = primitive_bound.max_extent().convert( base_vector);
    let size = bound_max - bound_min;

    let offset:<P::Point as EuclideanSpace>::Diff =
        StdRng::from_entropy().sample(Standard);
    let speed: <P::Point as EuclideanSpace>::Scalar =
        rand::thread_rng().gen_range(-10.0, 10.0);

    let position = emitter.location + offset;
    let pose = BodyPose::new(position, R::one());
    let mut transform = pose.as_transform();
    transform.set_scale(size);
    //+ log::info!("emitting box at {:?}", transform.translation());
    parts.object_type.insert(entity, ObjectType::Box).unwrap();
    parts.mesh.insert(entity, mesh).unwrap();
    parts
        .material
        .insert(entity, emitter.material.clone())
        .unwrap();
    // parts
    //     .global
    //     .insert(entity, /*Global*/Transform::default())
    //     .unwrap();
    parts.local.insert(entity, transform).unwrap();
    parts
        .physical_entity
        .dynamic_entity(
            entity,
            CollisionShape::<P, BodyPose<P::Point, R>, B, ObjectType>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                primitive.clone(),
            ),
            pose,
            Velocity::<<P::Point as EuclideanSpace>::Diff, A>::from_linear(offset * speed),
            PhysicalEntity::default(),
            Mass::<f32, I>::new(1.),
        ).unwrap();
}

#[derive(SystemData)]
pub struct EmitterParts<'a, P, B, R, A, I>
where
    B: Bound<Point = P::Point> + Union<B, Output = B> + Clone + Send + Sync + 'static,
    P: Primitive + ComputeBound<B> + Clone + Send + Sync + 'static,
    P::Point:
        EuclideanSpace<Scalar = f32> + Convert<Output = na::Vector3<f32>> + Send + Sync + 'static,
    <P::Point as EuclideanSpace>::Diff: /*Distribution<<P::Point as EuclideanSpace>::Diff> +*/ InnerSpace + Array + Send + Sync + 'static,
    R: Rotation<P::Point> + Convert<Output = na::UnitQuaternion<f32>> + Send + Sync + 'static,
    A: Clone + Copy + Zero + Send + Sync + 'static,
    I: Inertia + Send + Sync + 'static,
    Standard: Distribution<<P::Point as EuclideanSpace>::Diff>,
{
    pub object_type: WriteStorage<'a, ObjectType>,
    pub mesh: WriteStorage<'a, Handle<Mesh>>,
    pub material: WriteStorage<'a, Handle<Material>>,
    /*pub global: WriteStorage<'a, GlobalTransform>,*/
    pub local: WriteStorage<'a, Transform>,
    pub physical_entity: PhysicalEntityParts<
        'a,
        P,
        ObjectType,
        R,
        <P::Point as EuclideanSpace>::Diff,
        A,
        I,
        B,
        BodyPose<P::Point, R>,
    >,
}
