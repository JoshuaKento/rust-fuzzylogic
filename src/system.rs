use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use crate::{
    aggregate::aggregation, defuzz::defuzzification, error::MissingSpace, mamdani::Rule,
    prelude::*, variable::Variable,
};

/// Validation: ensure all consequents/antecedents reference existing vars/terms.
pub fn validate_rules<KV>(rules: &[Rule], vars: &HashMap<KV, Variable>) -> Result<()>
where
    KV: Eq + Hash + Borrow<str>,
{
    for r in rules {
        r.validate(vars)?;
    }
    Ok(())
}

pub trait Evaluator {
    fn evaluate<KI>(&self, input: &HashMap<KI, Float>) -> Result<HashMap<String, Float>>
    where
        KI: Eq + Hash + Borrow<str>;
}

/// A comprehensive struct for handling variables, rules and a uniform sampler cohesively.
pub struct System {
    pub vars: HashMap<String, Variable>,
    pub rules: Vec<Rule>,
    pub sampler: UniformSampler,
}

impl System {
    /// Create a new system and validate rules against provided variables.
    pub fn new(
        vars: HashMap<String, Variable>,
        rules: Vec<Rule>,
        sampler: UniformSampler,
    ) -> Result<Self> {
        // Validate without consuming the collections first.
        for r in &rules {
            r.validate(&vars)?;
        }
        Ok(Self {
            vars,
            rules,
            sampler,
        })
    }
}

impl Evaluator for System {
    fn evaluate<KI>(&self, input: &HashMap<KI, Float>) -> Result<HashMap<String, Float>>
    where
        KI: Eq + Hash + Borrow<str>,
    {
        let myu = aggregation(&self.rules, input, &self.vars, &self.sampler)?;
        defuzzification(&myu, &self.vars)
    }
}

// Builder for a System.
pub struct SystemBuilder {
    vars: HashMap<String, Variable>,
    rules: Vec<Rule>,
    sampler_n: usize,
}

impl SystemBuilder {
    /// Initialize a new SystemBuilder.
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            rules: Vec::new(),
            sampler_n: UniformSampler::DEFAULT_N,
        }
    }

    /// Configure the uniform sampler resolution.
    pub fn uniform_sampler(mut self, n: usize) -> Result<Self> {
        self.sampler_n = UniformSampler::new(n)?.n;
        Ok(self)
    }

    /// Insert a variable by name with a numeric domain.
    pub fn var(mut self, name: impl Into<String>, min: Float, max: Float) -> Result<Self> {
        let name = name.into();
        if self.vars.contains_key(&name) {
            return Err(FuzzyError::BadArity);
        }
        self.vars.insert(name, Variable::new(min, max)?);
        Ok(self)
    }

    /// Insert a term into an existing variable.
    pub fn term(
        mut self,
        var: &str,
        term: impl Into<String>,
        mf: impl MembershipFn + Send + Sync + 'static,
    ) -> Result<Self> {
        let var_ref = self.vars.get_mut(var).ok_or(FuzzyError::NotFound {
            space: MissingSpace::Var,
            key: var.to_string(),
        })?;

        let name: String = term.into();
        let t = Term::new(name.clone(), mf);
        var_ref.insert_term(&name, t)?;

        Ok(self)
    }

    /// Push a single rule into the builder.
    pub fn rule(mut self, rule: Rule) -> Result<Self> {
        self.rules.push(rule);
        Ok(self)
    }

    /// Finalize and build a System with a `UniformSampler`.
    pub fn build(self) -> Result<System> {
        System::new(self.vars, self.rules, UniformSampler::new(self.sampler_n)?)
    }
}
