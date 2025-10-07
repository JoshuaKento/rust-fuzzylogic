use super::{Float, MembershipFn, slope, validate_order};

///Struct for trapezoidal membership function.
///Initialize by calling the new() function.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Trapezoidal {
    left_leg: Float,
    left_base: Float,
    right_base: Float,
    right_leg: Float,
}

impl MembershipFn for Trapezoidal {
    ///Evaluates the membership value for the input x against the membership struct.
    fn eval(&self, x: Float) -> Float {
        let eps = crate::Float::EPSILON;

        //out of bounds check
        if x <= self.left_leg {
            return 0.0;
        }
        if x >= self.right_leg {
            return 0.0;
        }

        //calculation within the membership function
        if (x - self.left_base).abs() < eps || (x - self.right_base).abs() < eps {
            1.0
        } else if x < self.left_base {
            slope(x, self.left_leg, self.left_base, 1.0)
        } else if x > self.left_base && x < self.right_base {
            1.0
        } else {
            slope(x, self.right_base, self.right_leg, -1.0)
        }
    }
}

impl Trapezoidal {
    ///Initializes the struct. Note that it requires left_leg < left_base < right_base < right_leg.
    pub fn new(ll: Float, lb: Float, rb: Float, rl: Float) -> crate::error::Result<Self> {
        validate_order(&[ll, lb, rb, rl])?;
        return Ok(Trapezoidal {
            left_leg: ll,
            left_base: lb,
            right_base: rb,
            right_leg: rl,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trapezoidal() {
        let membership_func = Trapezoidal::new(-1.0, 0.0, 1.0, 2.0);
        let eps = crate::Float::EPSILON;

        assert_eq!(
            Trapezoidal::new(0.0, 0.0, -1.0, 0.0),
            Err(crate::error::FuzzyError::BadArity)
        );

        assert!((membership_func.clone().unwrap().eval(0.0) - 1.0).abs() < eps);
        assert!((membership_func.clone().unwrap().eval(1.0) - 1.0).abs() < eps);
        assert!((membership_func.clone().unwrap().eval(-0.5) - 0.5).abs() < eps);
        assert!((membership_func.unwrap().eval(2.0)).abs() < eps);
    }
}
