#[cfg(test)]
mod tests {

    use rust_fuzzylogic::{
        aggregate::aggregation, antecedent::Antecedent, mamdani::Consequent, mamdani::Rule,
        prelude::*, variable::Variable,
    };
    use std::{borrow::Borrow, collections::HashMap, hash::Hash};

    #[test]
    fn aggregation_test() {
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

        let mut Fanspeed = Variable::new(0.0, 10.0).unwrap();
        Fanspeed
            .insert_term(
                "High",
                Term::new("High", Triangular::new(5.0, 7.5, 10.0).unwrap()),
            )
            .unwrap();
        Fanspeed
            .insert_term(
                "Low",
                Term::new("hot", Triangular::new(0.0, 2.5, 5.0).unwrap()),
            )
            .unwrap();

        let mut Pumpspeed = Variable::new(0.0, 100.0).unwrap();
        Pumpspeed
            .insert_term(
                "High",
                Term::new("High", Triangular::new(80.0, 90.0, 100.0).unwrap()),
            )
            .unwrap();
        Pumpspeed
            .insert_term(
                "Low",
                Term::new("hot", Triangular::new(0.0, 10.0, 20.0).unwrap()),
            )
            .unwrap();

        let mut vars: HashMap<&str, Variable> = HashMap::new();
        vars.insert("temp", temp);
        vars.insert("Fanspeed", Fanspeed);
        vars.insert("Pumpspeed", Pumpspeed);

        let ast = Antecedent::And(
            Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "hot".into(),
            }),
            Box::new(Antecedent::Not(Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "cold".into(),
            }))),
        );

        let ast_2 = Antecedent::And(
            Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "cold".into(),
            }),
            Box::new(Antecedent::Not(Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "hot".into(),
            }))),
        );

        let csqt_1 = Consequent {
            var: "Fanspeed".to_string(),
            term: "High".to_string(),
        };

        let csqt_2 = Consequent {
            var: "Pumpspeed".to_string(),
            term: "High".to_string(),
        };

        let csqt_3 = Consequent {
            var: "Fanspeed".to_string(),
            term: "Low".to_string(),
        };

        let csqt_4 = Consequent {
            var: "Pumpspeed".to_string(),
            term: "Low".to_string(),
        };

        let rule = Rule {
            antecedent: ast,
            consequent: vec![csqt_1, csqt_2],
        };
        let rule_2 = Rule {
            antecedent: ast_2,
            consequent: vec![csqt_3, csqt_4],
        };

        let mut inputs: HashMap<&str, Float> = HashMap::new();
        inputs.insert("temp", 7.5);

        let sampler = UniformSampler::default();

        let alpha = rule.activation(&inputs, &vars).unwrap();

        let rules: Vec<Rule> = vec![rule, rule_2];

        let aggregate = aggregation(rules, &inputs, &vars, sampler);

        println!("{:?}", aggregate)
    }
}
