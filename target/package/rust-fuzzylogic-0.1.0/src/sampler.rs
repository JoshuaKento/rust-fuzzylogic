use crate::{error::FuzzyError, prelude::*, Float};

pub trait Sampler {
    //Trait shape: Returning Result<Vec<Float>> is fine.
    //If you later need performance, consider an iterator or a small wrapper type, but not necessary now.
    fn sample(&self, min: Float, max: Float) -> Result<Vec<Float>>;
}

pub struct UniformSampler {
    pub n: usize,
}

impl Default for UniformSampler {
    fn default() -> Self {
        Self { n: Self::DEFAULT_N }
    }
}

impl UniformSampler {
    pub const DEFAULT_N: usize = 101;

    pub fn new(n: usize) -> Result<Self> {
        if n < 2 {
            return Err(FuzzyError::OutOfBounds);
        }
        Ok(Self { n: n })
    }
}

impl Sampler for UniformSampler {
    fn sample(&self, min: Float, max: Float) -> Result<Vec<Float>> {
        if min >= max {
            return Err(FuzzyError::BadArity);
        }

        if !(min.is_finite() && max.is_finite()) {
            return Err(FuzzyError::BadArity);
        }

        let n = self.n;
        let mut sample: Vec<Float> = Vec::with_capacity(n);
        let step = (max - min) / (n as Float - 1.0);

        for i in 0..n {
            sample.push(min + i as Float * step)
        }
        sample[n - 1] = max;

        Ok(sample)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::FuzzyError;
    use crate::sampler::{Sampler, UniformSampler};
    use crate::Float;

    #[test]
    fn uniform_sampler_two_points_inclusive_endpoints() {
        let s = UniformSampler::new(2).unwrap();
        let min: Float = -3.5;
        let max: Float = 4.5;
        let pts = s.sample(min, max).unwrap();
        assert_eq!(pts.len(), 2);
        assert_eq!(pts[0], min);
        assert_eq!(pts[1], max, "Last point must equal max for n=2");
    }

    #[test]
    fn uniform_sampler_inclusive_endpoints_default() {
        let s = UniformSampler::default();
        let n = UniformSampler::DEFAULT_N;
        let min: Float = -5.0;
        let max: Float = 5.0;
        let pts = s.sample(min, max).unwrap();
        assert_eq!(pts.len(), n);
        assert_eq!(pts.first().copied().unwrap(), min);
        assert_eq!(
            pts.last().copied().unwrap(),
            max,
            "Sampler should include max exactly"
        );
    }

    #[test]
    fn uniform_sampler_spacing_monotonic() {
        let s = UniformSampler::default();
        let min: Float = 0.0;
        let max: Float = 10.0;
        let pts = s.sample(min, max).unwrap();
        assert!(pts.windows(2).all(|w| w[1] >= w[0]));

        // Check approximate uniform spacing consistency across interior points
        let eps = Float::EPSILON * 10.0;
        let base_step = pts[1] - pts[0];
        for i in 2..pts.len() {
            let step = pts[i] - pts[i - 1];
            assert!((step - base_step).abs() <= eps, "Non-uniform step at i={i}");
        }
    }

    #[test]
    fn uniform_sampler_invalid_points_rejected() {
        assert!(matches!(
            UniformSampler::new(0),
            Err(FuzzyError::OutOfBounds)
        ));
        assert!(matches!(
            UniformSampler::new(1),
            Err(FuzzyError::OutOfBounds)
        ));
    }

    #[test]
    fn uniform_sampler_invalid_range_rejected() {
        let s = UniformSampler::default();
        // min > max must error
        assert!(matches!(s.sample(1.0, 0.0), Err(FuzzyError::BadArity)));
        // Degenerate range should be rejected for a sampler that requires >=2 distinct points
        assert!(matches!(s.sample(1.0, 1.0), Err(_)));
    }
}
