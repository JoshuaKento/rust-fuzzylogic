use std::{borrow::Borrow, collections::HashMap, hash::Hash};

// Public APIs used by this module:
// - `prelude::*`: common scalar, error types, and traits (e.g., `Float`, `Result`, `FuzzyError`).
// - `Variable`: crisp variable with named fuzzy terms and domain validation.
use crate::{prelude::*, variable::Variable};

/// Antecedent abstract syntax tree (AST) for fuzzy rules.
///
/// This enum composes atomic predicates using the default Min–Max family:
/// - AND: `min(a, b)`
/// - OR:  `max(a, b)`
/// - NOT: `1 - a`
///
/// Each atomic predicate refers to a variable name and a term name
/// (e.g., `var = "temp"`, `term = "hot"`).
#[derive(Debug, Clone, PartialEq)]
pub enum Antecedent {
    /// Atomic predicate: membership of `term` for variable `var`.
    Atom { var: String, term: String },
    /// Conjunction: `min(left, right)` with the default operator family.
    And(Box<Self>, Box<Self>),
    /// Disjunction: `max(left, right)` with the default operator family.
    Or(Box<Self>, Box<Self>),
    /// Negation: `1 - value` with the default operator family.
    Not(Box<Self>),
}

/// Evaluate a fuzzy antecedent to a membership degree in [0, 1].
///
/// Uses the default Min–Max operator family (AND=min, OR=max, NOT=1−x).
///
/// Parameters:
/// - `ant`: antecedent AST to evaluate.
/// - `input`: crisp inputs keyed by variable name; key type `KI` must borrow as `str`.
/// - `vars`: variables keyed by name; key type `KV` must borrow as `str`.
///
/// Type bounds:
/// - `KI: Eq + Hash + Borrow<str>`
/// - `KV: Eq + Hash + Borrow<str>`
///
/// Returns `Ok(y)` with `y ∈ [0, 1]` on success, or an error if a variable or
/// input is missing, a term is unknown, or the input is outside the variable domain.
///
/// Complexity is linear in the AST size; recursion depth equals AST height.
pub fn eval_antecedent<KI, KV>(
    ant: &Antecedent,
    input: &HashMap<KI, Float>,
    vars: &HashMap<KV, Variable>,
) -> Result<Float>
where
    KI: Eq + Hash + Borrow<str>,
    KV: Eq + Hash + Borrow<str>,
{
    // Recursive evaluation according to the default Min–Max family.
    match ant {
        Antecedent::Atom { var, term } => {
            let v = vars.get(var.as_str()).ok_or(FuzzyError::NotFound {
                space: crate::error::MissingSpace::Var,
                key: var.clone(),
            })?;
            let x = *input.get(var.as_str()).ok_or(FuzzyError::NotFound {
                space: crate::error::MissingSpace::Input,
                key: term.clone(),
            })?;
            v.eval(term.as_str(), x)
        }
        Antecedent::And(a, b) => {
            let a = eval_antecedent(a, input, vars)?;
            let b = eval_antecedent(b, input, vars)?;
            Ok(a.min(b))
        }
        Antecedent::Or(a, b) => {
            let a = eval_antecedent(a, input, vars)?;
            let b = eval_antecedent(b, input, vars)?;
            Ok(a.max(b))
        }
        Antecedent::Not(a) => {
            let a = eval_antecedent(a, input, vars)?;
            Ok(1.0 - a)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::membership::triangular::Triangular;
    use crate::prelude::*;
    use crate::term::Term;
    use crate::variable::Variable;

    #[test]
    fn red_antecedent_and_not_behavior() {
        let eps = crate::Float::EPSILON;

        // Build a variable with two terms: cold and hot.
        let mut temp = Variable::new(-10.0, 10.0).unwrap();
        temp.insert_term(
            "cold",
            Term::new("cold", Triangular::new(-10.0, -5.0, 0.0).unwrap()),
        )
        .unwrap();
        temp.insert_term(
            "hot",
            Term::new("hot", Triangular::new(0.0, 5.0, 10.0).unwrap()),
        )
        .unwrap();

        let mut vars: HashMap<&str, Variable> = HashMap::new();
        vars.insert("temp", temp);

        // Crisp input for temp
        let mut inputs: HashMap<&str, crate::Float> = HashMap::new();
        inputs.insert("temp", 7.5);

        // AST: (temp is hot) AND NOT (temp is cold)
        let ast = crate::rule::Antecedent::And(
            Box::new(crate::rule::Antecedent::Atom {
                var: "temp".into(),
                term: "hot".into(),
            }),
            Box::new(crate::rule::Antecedent::Not(Box::new(
                crate::rule::Antecedent::Atom {
                    var: "temp".into(),
                    term: "cold".into(),
                },
            ))),
        );

        // Expected with defaults: min(hot(7.5), 1 - cold(7.5))
        let hot = Triangular::new(0.0, 5.0, 10.0).unwrap().eval(7.5);
        let cold = Triangular::new(-10.0, -5.0, 0.0).unwrap().eval(7.5);
        let expected = hot.min(1.0 - cold);

        let y = crate::rule::eval_antecedent(&ast, &inputs, &vars).unwrap();
        assert!((y - expected).abs() < eps);
    }

    // RED: OR behavior using the same variable at a different crisp value.
    #[test]
    fn red_antecedent_or_behavior() {
        // Variable setup
        let mut temp = Variable::new(-10.0, 10.0).unwrap();
        temp.insert_term(
            "cold",
            Term::new("cold", Triangular::new(-10.0, -5.0, 0.0).unwrap()),
        )
        .unwrap();
        temp.insert_term(
            "hot",
            Term::new("hot", Triangular::new(0.0, 5.0, 10.0).unwrap()),
        )
        .unwrap();

        let mut vars: HashMap<&str, Variable> = HashMap::new();
        vars.insert("temp", temp);

        let mut inputs: HashMap<&str, crate::Float> = HashMap::new();
        inputs.insert("temp", -5.0);

        // AST: (temp is cold) OR (temp is hot)
        let ast = crate::rule::Antecedent::Or(
            Box::new(crate::rule::Antecedent::Atom {
                var: "temp".into(),
                term: "cold".into(),
            }),
            Box::new(crate::rule::Antecedent::Atom {
                var: "temp".into(),
                term: "hot".into(),
            }),
        );

        // Expected with defaults: max(cold(-5), hot(-5)) = 1.0
        let cold = Triangular::new(-10.0, -5.0, 0.0).unwrap().eval(-5.0);
        let hot = Triangular::new(0.0, 5.0, 10.0).unwrap().eval(-5.0);
        let expected = cold.max(hot);

        let y = crate::rule::eval_antecedent(&ast, &inputs, &vars).unwrap();
        assert!((y - expected).abs() < crate::Float::EPSILON);
    }
}
