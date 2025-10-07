// Mamdani-style implication and rule behavior primitives.

use std::{borrow::Borrow, collections::HashMap, hash::Hash};

#[cfg(feature = "inference-mamdani")]
use crate::{
    antecedent::{Antecedent, eval_antecedent},
    error::{FuzzyError, MissingSpace},
    prelude::*,
    sampler::UniformSampler,
    variable::Variable,
};

/// Implication modes supported by a Mamdani rule.
pub enum Implication {
    Clip,
    Product,
}

/// Output clause of a fuzzy rule referencing a linguistic variable and term.
pub struct Consequent {
    pub var: String,
    pub term: String,
    //pub weight: Float,
    //pub imp: Implication,
}

/// Full fuzzy rule pairing an antecedent with one or more consequents.
pub struct Rule {
    pub antecedent: Antecedent,
    pub consequent: Vec<Consequent>,
}

//Mamdani Inference Engine
#[cfg(feature = "inference-mamdani")]
impl Rule {
    /// Evaluate the antecedent against crisp input values to obtain activation.
    pub fn activation<KI, KV>(
        &self,
        input: &HashMap<KI, Float>,
        vars: &HashMap<KV, Variable>,
    ) -> Result<Float>
    where
        KI: Eq + Hash + Borrow<str>,
        KV: Eq + Hash + Borrow<str>,
    {
        eval_antecedent(&self.antecedent, input, vars)
    }

    /// Apply the selected implication operator to produce discretized membership outputs.
    pub fn implicate<KV>(
        &self,
        alpha: Float,
        vers: &HashMap<KV, Variable>,
        sampler: &UniformSampler,
    ) -> Result<HashMap<String, Vec<Float>>>
    where
        KV: Eq + Hash + Borrow<str>,
    {
        let mut result_map: HashMap<String, Vec<Float>> = HashMap::new();

        for i in 0..self.consequent.len() {
            let mut result_vec = vec![0.0; sampler.n];

            let (dom_min, dom_max) = vers
                .get(&self.consequent[i].var.as_str())
                .ok_or(FuzzyError::NotFound {
                    space: MissingSpace::Var,
                    key: self.consequent[i].term.clone(),
                })?
                .domain();

            let step = (dom_max - dom_min) / sampler.n as Float;

            for k in 0..sampler.n {
                let x = dom_min + (k as Float * step);
                result_vec[k] = vers
                    .get(&self.consequent[i].var.as_str())
                    .ok_or(FuzzyError::NotFound {
                        space: MissingSpace::Var,
                        key: self.consequent[i].term.clone(),
                    })?
                    .eval(&self.consequent[i].term, x)?
                    .min(alpha);
            }

            result_map.insert(self.consequent[i].var.to_string(), result_vec);
        }
        return Ok(result_map);
        //TODO: Return type should be hashmap<string, Vec<Float>> where string signifies the variable(eg "fanspeed")
    }
}
