

pub trait Evaluable{
    fn membership(&self, value: f64) -> f64;
}

#[derive(Debug)]
pub enum ShapeError { SDNonPositive, BadTriangle, BadTrapezoid}

pub struct gaussian { mean: f64, sd: f64,}
impl gaussian {
    pub fn try_new(mean:f64, sd:f64) -> Result<Self, ShapeError>{
        if sd > 0.0 {Ok(Self{mean, sd})}
        else {Err(ShapeError::SDNonPositive)}
    }
}
impl Evaluable for gaussian{
    fn membership(&self, value: f64) -> f64{
        ((value - self.mean).powi(2) / (-2.0*self.sd.powi(2))).exp()    
    }
}

pub struct triangle {l:f64, c:f64, r:f64,}

impl triangle{
    pub fn try_new(l:f64,c:f64,r:f64) -> Result<Self, ShapeError>{
        if l < c && c < r {Ok(Self{l, c, r})}
        else{ Err(ShapeError::BadTriangle)}
    }
}

impl Evaluable for triangle {
    fn membership(&self, value: f64) -> f64{
        if value < self.l{return 0.0;}
        if value > self.r{return 0.0;}
        if value == self.c{return 1.0;}
        if value < self.c { (value - self.l)/(self.c-self.l)}
        else              {-(value - self.r)/(self.r-self.c)}
    }
}

pub struct trapezoid {  ll:f64,  lb:f64,  rb:f64,  rl:f64}

impl trapezoid {
    fn try_new(ll:f64, lb:f64, rb:f64, rl:f64) -> Result<Self, ShapeError>{
        if ll < lb && lb < rb && rb < rl{ Ok(Self{ll, lb, rb, rl})}
        else                            { Err(ShapeError::BadTrapezoid)}
    }
}

impl Evaluable for trapezoid {
    fn membership(&self, value: f64) -> f64 {
        if value < self.ll{return 0.0;}
        if value > self.rl{return 0.0;}
        if value > self.lb && value <= self.rb{return 1.0;}
        if value < self.lb{ (value - self.ll)/(self.lb-self.ll)}
        else              {-(value - self.rl)/(self.rl-self.rb)}
    }
}

pub fn evaluate_vector(ns: &[Box<dyn Evaluable>], value: f64) -> Vec<f64>{
        ns.iter().map(|n| n.membership(value)).collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test01() -> Result<(), ShapeError>{
        let memb_1 = triangle::try_new(2.0, 2.5, 3.5);
        let memb_2 = gaussian::try_new(2.0, 1.5);
        let memb_3 = trapezoid::try_new(2.9, 3.0, 4.0, 5.0);

        let ns: Vec<Box< dyn Evaluable>> = vec![
            Box::new(memb_1?), 
            Box::new(memb_2?),
            Box::new(memb_3?),
        ];
        
        let mut i = 0.0;
        while i < 5.5 {
            println!("{:?}", evaluate_vector(&ns , i));
            i+=0.5;
        }
        Ok(())
    }

    #[test]
    fn test02() -> Result<(), ShapeError>{
        assert!(triangle::try_new(0.0, -1.0, 99999999.0).is_err());
        assert!(gaussian::try_new(2.0, 0.0).is_err());
        assert!(trapezoid::try_new(-99999.0, 39.0, 4.0, -5.0).is_err());
        //assert_eq!(evaluate_vector(&ns, 0.5), vec![-1.0; 3]);
        Ok(())
    }
}