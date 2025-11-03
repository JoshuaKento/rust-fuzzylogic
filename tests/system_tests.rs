use std::collections::HashMap;

use rust_fuzzylogic::antecedent::Antecedent;
use rust_fuzzylogic::error::{FuzzyError, MissingSpace};
use rust_fuzzylogic::mamdani::{Consequent, Rule};
use rust_fuzzylogic::prelude::*;
use rust_fuzzylogic::system::{Evaluator, SystemBuilder};

#[test]
fn system_end_to_end_evaluate_hot() {
    // Build a simple system with one input var (temp) and one output var (fan).
    let sys = SystemBuilder::new()
        .var("temp", -10.0, 10.0)
        .unwrap()
        .term(
            "temp",
            "cold",
            Triangular::new(-10.0, -5.0, 0.0).unwrap(),
        )
        .unwrap()
        .term(
            "temp",
            "hot",
            Triangular::new(0.0, 5.0, 10.0).unwrap(),
        )
        .unwrap()
        .var("fan", 0.0, 10.0)
        .unwrap()
        .term(
            "fan",
            "low",
            Triangular::new(0.0, 2.5, 5.0).unwrap(),
        )
        .unwrap()
        .term(
            "fan",
            "high",
            Triangular::new(5.0, 7.5, 10.0).unwrap(),
        )
        .unwrap()
        // IF temp IS hot THEN fan IS high
        .rule(Rule {
            antecedent: Antecedent::Atom {
                var: "temp".into(),
                term: "hot".into(),
            },
            consequent: vec![Consequent {
                var: "fan".into(),
                term: "high".into(),
            }],
        })
        .unwrap()
        // IF temp IS cold THEN fan IS low
        .rule(Rule {
            antecedent: Antecedent::Atom {
                var: "temp".into(),
                term: "cold".into(),
            },
            consequent: vec![Consequent {
                var: "fan".into(),
                term: "low".into(),
            }],
        })
        .unwrap()
        .build()
        .unwrap();

    // Crisp input: a hot temperature should bias output toward higher fan speed.
    let mut input: HashMap<&str, Float> = HashMap::new();
    input.insert("temp", 7.5);

    let crisp = Evaluator::evaluate(&sys, &input).expect("system evaluation must succeed");
    assert!(crisp.contains_key("fan"));
    let y = crisp["fan"];
    // Output must lie within the fan variable domain [0, 10].
    assert!(y >= 0.0 && y <= 10.0);
    // With temp=7.5 (hot), centroid should be noticeably above midpoint.
    assert!(y > 5.0);
}

#[test]
fn system_build_validation_missing_term_errors() {
    // Create a builder with variable `x` and a single term `a`.
    let b = SystemBuilder::new()
        .var("x", 0.0, 1.0)
        .unwrap()
        .term("x", "a", Triangular::new(0.0, 0.5, 1.0).unwrap())
        .unwrap();

    // Rule references a missing term `missing` in the antecedent.
    let rule = Rule {
        antecedent: Antecedent::Atom {
            var: "x".into(),
            term: "missing".into(),
        },
        consequent: vec![],
    };

    let res = b.rule(rule).unwrap().build();
    // Expect a NotFound error for the missing term referenced by the rule.
    assert!(matches!(
        res,
        Err(FuzzyError::NotFound {
            space: MissingSpace::Term,
            ..
        })
    ));
}

