#[cfg(test)]
mod tests {

    use rust_fuzzylogic::{
        aggregate::aggregation,
        antecedent::Antecedent,
        defuzz::defuzzification,
        mamdani::{Consequent, Rule},
        prelude::*,
        variable::Variable,
    };
    use std::collections::HashMap;
    #[test]
    fn end_to_end_test() {
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

        let mut fanpspeed = Variable::new(0.0, 10.0).unwrap();
        fanpspeed
            .insert_term(
                "High",
                Term::new("High", Triangular::new(5.0, 7.5, 10.0).unwrap()),
            )
            .unwrap();
        fanpspeed
            .insert_term(
                "Low",
                Term::new("hot", Triangular::new(0.0, 2.5, 5.0).unwrap()),
            )
            .unwrap();

        let mut pumpspeed = Variable::new(0.0, 100.0).unwrap();
        pumpspeed
            .insert_term(
                "High",
                Term::new("High", Triangular::new(80.0, 90.0, 100.0).unwrap()),
            )
            .unwrap();
        pumpspeed
            .insert_term(
                "Low",
                Term::new("hot", Triangular::new(0.0, 10.0, 20.0).unwrap()),
            )
            .unwrap();

        let mut vars: HashMap<&str, Variable> = HashMap::new();
        vars.insert("temp", temp);
        vars.insert("fanpspeed", fanpspeed);
        vars.insert("pumpspeed", pumpspeed);

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
            var: "fanpspeed".to_string(),
            term: "High".to_string(),
        };

        let csqt_2 = Consequent {
            var: "pumpspeed".to_string(),
            term: "High".to_string(),
        };

        let csqt_3 = Consequent {
            var: "fanpspeed".to_string(),
            term: "Low".to_string(),
        };

        let csqt_4 = Consequent {
            var: "pumpspeed".to_string(),
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

        let rules: Vec<Rule> = vec![rule, rule_2];

        let aggregate = aggregation(&rules, &inputs, &vars, &sampler).unwrap();

        //println!("{:?}", aggregate);

        let centroid = defuzzification(&aggregate, &vars);

        println!("{:?}", centroid);
    }
}
