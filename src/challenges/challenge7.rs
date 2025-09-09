//### Challenge 7: "Generic Value Processor"
//Create a generic struct that can work with different numeric types (f32, f64, i32).
//It should perform membership calculations while preserving the original type.
//Handle the conversion challenges between types.

//**Learning:** Generics, trait bounds, type constraints

//sketch
//Struct for processing value
struct GenericValueProcessor<T>
where T: Copy + Into<f64> + PartialOrd,
{
    l: T,
    c: T,
    r: T,
}

struct triangular{l:f64, c:f64, r:f64}

impl<T> GenericValueProcessor<T> where T: Copy + Into<f64> + PartialOrd,
{
    //Implement Individual Functions here 


    //define a triangular membership function with validation(l<c<r)
    pub fn triangular(l:T,c:T,r:T)-> Self{
        Self{l, c, r}
    }

    //convert types while preserving the original type as well
    fn raw_min(&self)-> T {
        self.l
    }

    //calculate Membership degree
    fn degree<U>(&self, x:U)-> f64 where U: Into<f64>{
        let x = x.into();
        let l = self.l.into();
        let c = self.c.into();
        let r = self.r.into();

        if x <= l || x >= r {
            return 0.0;
        }
        if (x - c).abs() < f64::EPSILON {
            return 1.0;
        }
        if x < c {
            (x - l) / (c - l)
        } else {
            (r - x) / (r - c)
        }
    }

}






//Implementation Idea 1
//Convert it To a Ceartain type(e.g. f64) and after calculation convert it back to the original type

//





#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time helper: succeeds only when both args share the same type.
    fn assert_same_type<T>(_a: &T, _b: &T) {}

    #[test]
    fn i32_membership() {
        let tri = GenericValueProcessor::triangular(0, 5, 10);
        assert_eq!(tri.degree(0), 0.0);
        assert_eq!(tri.degree(5), 1.0);
        assert_eq!(tri.degree(10), 0.0);
        assert_same_type(&tri.raw_min(), &0);
    }

    #[test]
    fn f32_midpoint() {
        let tri = GenericValueProcessor::triangular(0.0_f32, 5.0, 10.0);
        let mid = tri.degree(2.5);
        assert!((mid - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn f64_three_quarters() {
        let tri = GenericValueProcessor::triangular(0.0_f64, 5.0, 10.0);
        assert_eq!(tri.degree(7.5), 0.5);
    }
}