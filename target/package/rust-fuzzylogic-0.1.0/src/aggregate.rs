// Aggregation utilities for combining rule outputs across consequents.

use crate::{mamdani::Rule, prelude::*, variable::Variable};
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

/// Combine two membership sample vectors by taking the pointwise maximum.
pub fn elements_max(data: &mut Vec<Float>, src: &Vec<Float>) {
    for (d, s) in data.iter_mut().zip(src) {
        *d = d.max(*s)
    }
}

/// Aggregate the contributions of all rules into output membership functions.
pub fn aggregation<KI, KV>(
    rules: Vec<Rule>,
    input: &HashMap<KI, Float>,
    vars: &HashMap<KV, Variable>,
    sampler: UniformSampler,
) -> Result<HashMap<String, Vec<Float>>>
where
    KI: Eq + Hash + Borrow<str>,
    KV: Eq + Hash + Borrow<str>,
{
    let mut implicated_map: HashMap<String, Vec<Float>> = HashMap::new();
    for i in 0..rules.len() {
        let alpha = rules[i].activation(&input, &vars)?;
        let implicated = rules[i].implicate(alpha, vars, &sampler)?;

        for (k, v) in implicated {
            implicated_map
                .entry(k)
                .and_modify(|cur| elements_max(cur, &v))
                .or_insert(v);
        }
    }

    return Ok(implicated_map);
}
