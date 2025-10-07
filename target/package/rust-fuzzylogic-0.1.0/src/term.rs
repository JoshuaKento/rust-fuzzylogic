use crate::membership::MembershipFn;

//Type: Term holding a boxed membership function.
pub type BoxedMembershipFn = Box<dyn MembershipFn + Send + Sync + 'static>;

pub struct Term {
    name: String,
    mf: BoxedMembershipFn,
}

impl Term {
    ///Creates a new term label wrapper.
    pub fn new<S, M>(name: S, m: M) -> Self
    where
        S: Into<String>,
        M: MembershipFn + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            mf: Box::new(m),
        }
    }

    ///Returns the labeled name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl crate::membership::MembershipFn for Term {
    ///Evaluates the membership degree of input against the wrapped membership.
    fn eval(&self, x: crate::Float) -> crate::Float {
        self.mf.eval(x)
    }
}

#[cfg(test)]
mod tests {
    use crate::term::*;

    #[test]
    fn test_term() {
        // TEST for Term + label functionality.
        // Expect a Term wrapper that carries a string label and delegates eval to the inner membership fn.
        let mf = crate::membership::triangular::Triangular::new(-1.0, 0.0, 1.0).unwrap();
        let term = crate::term::Term::new("cold", mf);

        // The term should expose its name/label and evaluate via the wrapped membership function.
        assert_eq!(term.name(), "cold");
        let x: crate::Float = 0.25;
        let y = term.eval(x);
        assert!(y >= 0.0 && y <= 1.0);
    }
}
