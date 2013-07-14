use std::num::Bounded;
use nalgebra::traits::transformation::{Transformation, Transform, Transformable};
use nalgebra::traits::rotation::Rotate;
use bounding_volume::aabb::AABB;
use bounding_volume::has_bounding_volume::HasBoundingVolume;

/**
 * Implicit description of a plane.
 *
 *   - `V`: type of the plane center and normal.
 */
#[deriving(Eq, ToStr, Clone)]
pub struct Plane<V>
{
  priv center: V,
  priv normal: V
}

impl<V> Plane<V>
{
  /// Builds a new plane from its center and its normal.
  #[inline]
  pub fn new(center: V, normal: V) -> Plane<V>
  { Plane { center: center, normal: normal } }
}


impl<V: Clone> Plane<V>
{
  /// The plane normal.
  #[inline]
  pub fn normal(&self) -> V
  { self.normal.clone() }

  /// The plane center.
  #[inline]
  pub fn center(&self) -> V
  { self.center.clone() }
}

impl<V, M: Transform<V> + Rotate<V>> Transformation<M> for Plane<V>
{
  #[inline]
  fn transformation(&self) -> M
  { fail!("Not yet implemented") } // deduce a transformation from the normal

  #[inline]
  fn inv_transformation(&self) -> M
  { fail!("Not yet implemented") } // deduce a transformation from the normal


  #[inline]
  fn transform_by(&mut self, transform: &M)
  {
    self.center = transform.transform_vec(&self.center);
    self.normal = transform.rotate(&self.normal);
  }
}

impl<V: Clone> Transform<V> for Plane<V>
{
  #[inline]
  fn transform_vec(&self, v: &V) -> V
  { v.clone() } // FIXME: we shit a little bit here =)

  #[inline]
  fn inv_transform(&self, v: &V) -> V
  { v.clone() } // FIXME: we shit a little bit here =)
}

impl<V, M: Transform<V> + Rotate<V>> Transformable<M, Plane<V>> for Plane<V>
{
  #[inline]
  fn transformed(&self, transform: &M) -> Plane<V>
  { Plane::new(transform.transform_vec(&self.center),
               transform.rotate(&self.normal)) }
}

// FIXME: these is something bad here…
// Since we cannot implement HasBoundingVolume twice, we wont be able to
// implement any other bounding volume… That’s bad.
impl<V: Bounded + Neg<V> + Ord + Clone>
    HasBoundingVolume<AABB<V>> for Plane<V>
{
  fn bounding_volume(&self) -> AABB<V>
  { AABB::new(-Bounded::max_value::<V>(), Bounded::max_value()) }
}