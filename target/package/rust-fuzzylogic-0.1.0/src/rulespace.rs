use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use crate::{
    aggregate::aggregation,
    defuzz::defuzzification,
    error::{self},
    mamdani::Rule,
    sampler::UniformSampler,
    variable::Variable,
    Float,
};

// Container for fuzzy variables, rules, and intermediate membership data.
pub struct RuleSpace {
    pub vars: HashMap<String, Variable>,
    pub myu: HashMap<String, Vec<Float>>,
    pub rules: Vec<Rule>,
}

impl RuleSpace {
    /// Create a rule space with the supplied variables and rules.
    pub fn new(self, vars: HashMap<String, Variable>, rules: Vec<Rule>) -> Self {
        return Self {
            vars: vars,
            myu: HashMap::new(),
            rules: rules,
        };
    }

    /// Append additional rules to the existing rule set.
    pub fn add_rules(mut self, rules: &mut Vec<Rule>) {
        self.rules.append(rules);
    }

    /// Run the aggregation step for all rules with the provided crisp inputs.
    pub fn aggregate<KI>(
        &mut self,
        input: &HashMap<KI, Float>,
        sampler: UniformSampler,
    ) -> error::Result<()>
    where
        KI: Eq + Hash + Borrow<str>,
    {
        let rules = std::mem::take(&mut self.rules);
        let myu = aggregation(rules, input, &self.vars, sampler)?;
        self.myu = myu;

        Ok(())
    }

    /// Aggregate and then defuzzify each output variable using the supplied sampler.
    pub fn defuzzificate<KI>(
        &mut self,
        input: &HashMap<KI, Float>,
        sampler: UniformSampler,
    ) -> crate::error::Result<HashMap<String, Float>>
    where
        KI: Eq + Hash + Borrow<str>,
    {
        let _ = self.aggregate(input, sampler);
        let myu = std::mem::take(&mut self.myu);
        Ok(defuzzification(myu, &self.vars)?)
    }
    //is there a nessecity?
    //pub fn consequent_keys() {}
}
