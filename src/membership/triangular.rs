use super::{Float, FuzzyError, MembershipFn, slope, validate_order};

///Struct for triangular membership function.
///Initialize by calling the new() function.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Triangular {
    left: Float,
    center: Float,
    right: Float,
}

impl MembershipFn for Triangular {
    ///Evaluates the membership value for the input x against the membership struct.
    fn eval(&self, x: Float) -> Float {
        //out of bounds check
        if x <= self.left {
            return 0.0;
        }
        if x >= self.right {
            return 0.0;
        }

        //calculation within membership function
        if (x - self.center).abs() < 1e-9 {
            1.0
        } else if x < self.center {
            slope(x, self.left, self.center, 1.0)
        } else {
            slope(x, self.center, self.right, -1.0)
        }
    }
}

impl Triangular {
    ///Initializes the struct. Note that it requires left < center < right.
    pub fn new(l: Float, c: Float, r: Float) -> crate::error::Result<Self> {
        validate_order(&[l, c, r])?;
        return Ok(Triangular {
            left: l,
            center: c,
            right: r,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular() {
        let membership_func = Triangular::new(-1.0, 0.0, 1.0);
        let eps = crate::Float::EPSILON;

        assert_eq!(Triangular::new(0.0, 0.0, -1.0), Err(FuzzyError::BadArity));

        assert!((membership_func.clone().unwrap().eval(0.0) - 1.0).abs() < eps);
        assert!((membership_func.clone().unwrap().eval(0.5) - 0.5).abs() < eps);
        assert!((membership_func.unwrap().eval(1.0)).abs() < eps);
    }
}
