//Possible Defuzz methods => Bisecctor of Area, Centroid, ...
use crate::{prelude::*, variable::Variable};
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

pub fn defuzzification<KV>(
    myu: HashMap<String, Vec<Float>>,
    vars: &HashMap<KV, Variable>,
) -> Result<HashMap<String, Float>>
where
    KV: Eq + Hash + Borrow<str>,
{
    let mut result_map: HashMap<String, Float> = HashMap::new();
    for (i, j) in myu {
        let num = j.len();
        let (var_min, var_max) = vars.get(&i).ok_or(FuzzyError::EmptyInput)?.domain();
        let step = (var_max - var_min) / (num as Float - 1.0);

        let (mut sum_myux, mut sum_myu): (Float, Float) = (0.0, 0.0);
        let mut l: usize = 0;
        for k in j {
            let x = var_min + step * l as Float;
            sum_myux = sum_myux + (x * k);
            sum_myu = sum_myu + k;
            l += 1;
        }
        result_map.insert(i, sum_myux / sum_myu);
    }

    return Ok(result_map);
}
