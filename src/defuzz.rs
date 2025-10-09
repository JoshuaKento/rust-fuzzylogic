// Defuzzification utilities for collapsing aggregated membership values.
use crate::{prelude::*, variable::Variable};
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

/// Defuzzify aggregated membership samples using the centroid of area method.
pub fn defuzzification<KV>(
    agg_memberships: &HashMap<String, Vec<Float>>,
    vars: &HashMap<KV, Variable>,
) -> Result<HashMap<String, Float>>
where
    KV: Eq + Hash + Borrow<str>,
{
    let mut result_map: HashMap<String, Float> = HashMap::new();
    for (i, j) in agg_memberships {
        let num = j.len();
        let (var_min, var_max) = vars.get(&i).ok_or(FuzzyError::EmptyInput)?.domain();
        let step = (var_max - var_min) / (num as Float - 1.0);

        let (mut sum_agg_memberships_x, mut sum_agg_memberships): (Float, Float) = (0.0, 0.0);
        let mut l: usize = 0;
        for k in j {
            let x = var_min + step * l as Float;
            sum_agg_memberships_x = sum_agg_memberships_x + (x * k);
            sum_agg_memberships = sum_agg_memberships + k;
            l += 1;
        }
        result_map.insert(i.to_string(), sum_agg_memberships_x / sum_agg_memberships);
    }

    return Ok(result_map);
}
