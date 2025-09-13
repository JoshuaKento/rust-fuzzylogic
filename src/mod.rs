pub trait MembershipFn {
    fn eval(&self, x: crate::Float) -> crate::Float;
}
pub mod triangular;
pub mod trapezoidal;
pub mod gaussian;

