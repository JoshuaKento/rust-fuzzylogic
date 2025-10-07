use crate::error::*;
use crate::*;

pub mod gaussian;
pub mod trapezoidal;
pub mod triangular;

pub use gaussian::Gaussian;
pub use triangular::Triangular;

pub trait MembershipFn {
    fn eval(&self, x: crate::Float) -> crate::Float;
}

///validation function to check that the order in the tiangular or trapezoidal apexes are correct.
fn validate_order(vals: &[Float]) -> Result<()> {
    for i in 0..vals.len() - 1 {
        if vals[i + 1] <= vals[i] {
            return Err(FuzzyError::BadArity);
        }
    }
    Ok(())
}

///Calculate the slope. delta is the change amount.(Either 1.0 or -1.0 by definition.)
fn slope(value: Float, left: Float, right: Float, delta: Float) -> Float {
    (delta * (value - left) / (right - left) + ((-1.0 * delta + 1.0) / 2.0)).clamp(0.0, 1.0)
}

//simple unit testing for validation
#[cfg(test)]
mod tests {
    use crate::membership::validate_order;

    #[test]
    fn test_validation() {
        assert_eq!(
            validate_order(&[0.0, 1.1, 0.5]),
            Err(crate::error::FuzzyError::BadArity)
        );
    }
}
