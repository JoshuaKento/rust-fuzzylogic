use super::{Float, FuzzyError, MembershipFn};

///validation function to check if sd is greater than 0.0 for Gaussian function.
fn validate_positive(val: Float) -> Result<(), FuzzyError> {
    if val <= 0.0 {
        return Err(FuzzyError::OutOfBounds);
    }
    Ok(())
}

///Struct for gaussian membership function.
///Initialize by calling the new() function.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Gaussian {
    ///sd is also commonly called sigma or standard deviation.
    sd: Float,
    mean: Float,
    neg_two_sigma_sq: Float,
}

impl MembershipFn for Gaussian {
    ///Evaluates the membership value for the input x against the membership struct.
    fn eval(&self, x: Float) -> Float {
        return ((x - self.mean).powi(2) / self.neg_two_sigma_sq).exp();
    }
}

impl Gaussian {
    ///Initializes the struct. Note that it requires sd > 0.0
    pub fn new(sd: Float, mean: Float) -> crate::error::Result<Self> {
        validate_positive(sd)?;
        //precalculate and save constant "neg_two_sigma_sq" for performance at eval().
        return Ok(Gaussian {
            sd: sd,
            mean: mean,
            neg_two_sigma_sq: (-2.0 * sd.powi(2)),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::membership::gaussian::*;

    #[test]
    fn test_gaussian() {
        let eps = crate::Float::EPSILON;
        assert_eq!(Gaussian::new(0.0, 0.0), Err(FuzzyError::OutOfBounds));
        let membership = Gaussian::new(1.0, 0.0);

        assert!(membership.clone().unwrap().eval(100.0).abs() < eps);
        assert!((membership.clone().unwrap().eval(0.0) - 1.0).abs() < eps);
        assert_eq!(
            membership.clone().unwrap().eval(-1.0),
            membership.clone().unwrap().eval(1.0)
        );
    }
}
