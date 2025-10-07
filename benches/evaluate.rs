// Batch evaluation script for the rust-fuzzylogic crate.
//
// This is a standalone utility placed under `benches/` to avoid impacting the
// public API. It demonstrates building a small fuzzy system and evaluating
// many crisp inputs in a batch, either from a uniform grid or a CSV file.
//
// Usage examples (when compiled as a normal binary, e.g., with harness=false):
// - Grid (default):   evaluate
// - Grid (custom):    evaluate --grid -10 10 1001 --sampler 201
// - From CSV:         evaluate --csv path/to/inputs.csv --sampler 101
//
// Note: By default cargo treats files in `benches/` as benchmark targets. If
// you want to run this directly via `cargo run`, add a `[[bench]]` entry with
// `harness = false` for this file temporarily, or copy it under `examples/`.
// The code itself compiles fine under the default bench harness (it’s gated so
// no duplicate main is defined during `cargo bench`).

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use rust_fuzzylogic::antecedent::Antecedent;
use rust_fuzzylogic::mamdani::{Consequent, Rule};
use rust_fuzzylogic::prelude::*;
use rust_fuzzylogic::variable::Variable;

// ---------- System construction ----------

fn build_vars() -> Result<HashMap<String, Variable>> {
    // Input: temp ∈ [-10, 10] with two terms
    let mut temp = Variable::new(-10.0, 10.0)?;
    temp.insert_term(
        "cold",
        Term::new("cold", Triangular::new(-10.0, -5.0, 0.0)?),
    )?;
    temp.insert_term(
        "hot",
        Term::new("hot", Triangular::new(0.0, 5.0, 10.0)?),
    )?;

    // Output: fanpspeed ∈ [0, 10]
    let mut fanpspeed = Variable::new(0.0, 10.0)?;
    fanpspeed.insert_term(
        "High",
        Term::new("High", Triangular::new(5.0, 7.5, 10.0)?),
    )?;
    fanpspeed.insert_term(
        "Low",
        Term::new("Low", Triangular::new(0.0, 2.5, 5.0)?),
    )?;

    // Output: pumpspeed ∈ [0, 100]
    let mut pumpspeed = Variable::new(0.0, 100.0)?;
    pumpspeed.insert_term(
        "High",
        Term::new("High", Triangular::new(80.0, 90.0, 100.0)?),
    )?;
    pumpspeed.insert_term(
        "Low",
        Term::new("Low", Triangular::new(0.0, 10.0, 20.0)?),
    )?;

    let mut vars: HashMap<String, Variable> = HashMap::new();
    vars.insert("temp".to_string(), temp);
    vars.insert("fanpspeed".to_string(), fanpspeed);
    vars.insert("pumpspeed".to_string(), pumpspeed);
    Ok(vars)
}

fn build_rules() -> Vec<Rule> {
    // Rule 1: IF temp is hot AND NOT cold THEN fanpspeed is High, pumpspeed is High
    let r1 = Rule {
        antecedent: Antecedent::And(
            Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "hot".into(),
            }),
            Box::new(Antecedent::Not(Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "cold".into(),
            }))),
        ),
        consequent: vec![
            Consequent {
                var: "fanpspeed".into(),
                term: "High".into(),
            },
            Consequent {
                var: "pumpspeed".into(),
                term: "High".into(),
            },
        ],
    };

    // Rule 2: IF temp is cold AND NOT hot THEN fanpspeed is Low, pumpspeed is Low
    let r2 = Rule {
        antecedent: Antecedent::And(
            Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "cold".into(),
            }),
            Box::new(Antecedent::Not(Box::new(Antecedent::Atom {
                var: "temp".into(),
                term: "hot".into(),
            }))),
        ),
        consequent: vec![
            Consequent {
                var: "fanpspeed".into(),
                term: "Low".into(),
            },
            Consequent {
                var: "pumpspeed".into(),
                term: "Low".into(),
            },
        ],
    };

    vec![r1, r2]
}

// Evaluate a single crisp input for `temp` using a fresh rule vector each call.
fn evaluate_single(vars: &HashMap<String, Variable>, x: Float, sampler_n: usize) -> Result<HashMap<String, Float>> {
    use rust_fuzzylogic::aggregate::aggregation;
    use rust_fuzzylogic::defuzz::defuzzification;
    use rust_fuzzylogic::sampler::UniformSampler;

    let mut inputs: HashMap<&str, Float> = HashMap::new();
    inputs.insert("temp", x);

    let sampler = UniformSampler::new(sampler_n)?;
    let rules = build_rules();
    let myu = aggregation(rules, &inputs, vars, sampler)?;
    defuzzification(myu, vars)
}

fn parse_f64<T: AsRef<str>>(s: T) -> Option<Float> {
    s.as_ref().trim().parse::<Float>().ok()
}

fn read_csv_inputs<P: AsRef<Path>>(path: P) -> io::Result<Vec<Float>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut xs = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        // Try CSV with header: assume header if first line contains non-numeric
        if i == 0 {
            let header_maybe_num = trimmed
                .split(',')
                .next()
                .and_then(|t| t.trim().parse::<Float>().ok());
            if header_maybe_num.is_none() && trimmed.to_ascii_lowercase().contains("temp") {
                // Header line, skip
                continue;
            }
        }

        // Single column or first column as `temp`
        if let Some(first) = trimmed.split(',').next() {
            if let Some(v) = parse_f64(first) {
                xs.push(v);
            }
        }
    }
    Ok(xs)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Grid,
    Csv,
}

#[cfg(not(test))]
fn main() {
    // Defaults
    let mut mode = Mode::Grid;
    let mut grid_min: Float = -10.0;
    let mut grid_max: Float = 10.0;
    let mut grid_n: usize = 101;
    let mut sampler_n: usize = rust_fuzzylogic::sampler::UniformSampler::DEFAULT_N;
    let mut csv_path: Option<String> = None;

    // Simple CLI parsing
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--grid" => {
                // --grid <min> <max> <n>
                if i + 3 >= args.len() {
                    eprintln!("Missing args for --grid <min> <max> <n>");
                    return;
                }
                if let (Some(a), Some(b), Ok(n)) = (
                    parse_f64(&args[i + 1]),
                    parse_f64(&args[i + 2]),
                    args[i + 3].parse::<usize>(),
                ) {
                    grid_min = a;
                    grid_max = b;
                    grid_n = n;
                    mode = Mode::Grid;
                    i += 4;
                    continue;
                } else {
                    eprintln!("Failed to parse --grid arguments");
                    return;
                }
            }
            "--csv" => {
                // --csv <path>
                if i + 1 >= args.len() {
                    eprintln!("Missing path for --csv <path>");
                    return;
                }
                csv_path = Some(args[i + 1].clone());
                mode = Mode::Csv;
                i += 2;
                continue;
            }
            "--sampler" => {
                // --sampler <n>
                if i + 1 >= args.len() {
                    eprintln!("Missing value for --sampler <n>");
                    return;
                }
                match args[i + 1].parse::<usize>() {
                    Ok(n) if n >= 2 => sampler_n = n,
                    _ => {
                        eprintln!("--sampler must be an integer >= 2");
                        return;
                    }
                }
                i += 2;
                continue;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("Unknown arg: {}", args[i]);
                print_help();
                return;
            }
        }
    }

    let vars = match build_vars() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to build variables: {e}");
            return;
        }
    };

    let mut xs: Vec<Float> = Vec::new();
    match mode {
        Mode::Grid => {
            if grid_n < 2 || !grid_min.is_finite() || !grid_max.is_finite() || grid_min >= grid_max {
                eprintln!("Invalid grid parameters. See --help");
                return;
            }
            let step = (grid_max - grid_min) / (grid_n as Float - 1.0);
            for i in 0..grid_n {
                xs.push(grid_min + i as Float * step);
            }
            // ensure exact max
            if let Some(last) = xs.last_mut() { *last = grid_max; }
        }
        Mode::Csv => {
            let Some(path) = csv_path else {
                eprintln!("--csv <path> is required when using CSV mode");
                return;
            };
            match read_csv_inputs(&path) {
                Ok(v) => xs = v,
                Err(e) => {
                    eprintln!("Failed to read CSV inputs: {e}");
                    return;
                }
            }
            if xs.is_empty() {
                eprintln!("No inputs found in CSV");
                return;
            }
        }
    }

    // Header
    println!("temp,fanpspeed,pumpspeed");

    // Evaluate each input; print CSV row per evaluation
    for x in xs {
        match evaluate_single(&vars, x, sampler_n) {
            Ok(mut m) => {
                // Stable order: print outputs in key order
                let fan = m.remove("fanpspeed").unwrap_or(Float::NAN);
                let pump = m.remove("pumpspeed").unwrap_or(Float::NAN);
                println!("{x},{fan},{pump}");
            }
            Err(e) => {
                eprintln!("Error evaluating x={x}: {e}");
            }
        }
    }
}

#[cfg(not(test))]
fn print_help() {
    eprintln!(
        "Usage: evaluate [--grid <min> <max> <n>] [--csv <path>] [--sampler <n>]\n\
         - Default evaluates a grid over temp in [-10, 10] with 101 points.\n\
         - --csv reads a file with a header containing 'temp' or a single column of values.\n\
         - --sampler controls the discretization used during aggregation/defuzzification."
    );
}
