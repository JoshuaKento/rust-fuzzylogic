use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use crate::{
    aggregate::aggregation,
    defuzz::defuzzification,
    error::{self, FuzzyError},
    mamdani::Rule,
    sampler::UniformSampler,
    variable::Variable,
    Float,
};

// Container for fuzzy variables, rules, and intermediate membership data.
pub struct RuleSpace {
    vars: HashMap<String, Variable>,
    agg_memberships: HashMap<String, Vec<Float>>,
    rules: Vec<Rule>,
}

impl RuleSpace {
    /// Create a rule space with the supplied variables and rules.
    pub fn new(vars: HashMap<String, Variable>, rules: Vec<Rule>) -> error::Result<Self> {
        if vars.is_empty() || rules.is_empty() {
            return Err(FuzzyError::EmptyInput);
        } else {
            return Ok(Self {
                vars: vars,
                agg_memberships: HashMap::new(),
                rules: rules,
            });
        }
    }

    /// Append additional rules to the existing rule set.
    pub fn add_rules(&mut self, rules: &mut Vec<Rule>) -> error::Result<&mut Self> {
        if rules.is_empty() {
            Err(FuzzyError::EmptyInput)
        } else {
            let _ = &mut self.rules.append(rules);
            return Ok(self);
        }
    }

    /// Run the aggregation step for all rules with the provided crisp inputs.
    pub fn aggregate<KI>(
        &mut self,
        input: &HashMap<KI, Float>,
        sampler: &UniformSampler,
    ) -> error::Result<()>
    where
        KI: Eq + Hash + Borrow<str>,
    {
        //let rules = std::mem::take(&mut self.rules);
        let agg_memberships = aggregation(&self.rules, input, &self.vars, sampler)?;
        self.agg_memberships = agg_memberships;

        Ok(())
    }

    /// Aggregate and then defuzzify each output variable using the supplied sampler.
    pub fn defuzzify<KI>(
        &mut self,
        input: &HashMap<KI, Float>,
        sampler: &UniformSampler,
    ) -> error::Result<HashMap<String, Float>>
    where
        KI: Eq + Hash + Borrow<str>,
    {
        let _ = self.aggregate(input, sampler)?;
        //let agg_memberships = std::mem::take(&mut self.agg_memberships);
        Ok(defuzzification(&self.agg_memberships, &self.vars)?)
    }
    //is there a nessecity?
    //pub fn consequent_keys() {}
}
