/**
 * Trait of inversible objects. Typically used to implement matrix inverse.
 */
pub trait Inv
{
  /// Returns the inverse of an element.
  fn inverse(&self) -> Option<Self>;
  /// Inplace version of `inverse`.
  fn invert(&mut self) -> bool;
}