#[derive(Debug, Clone)]
pub enum MembershipShape{
    Triangle {
        left: f64, 
        center:f64,
        right:f64
    },
    Trapezoid {
        left_leg:f64, 
        left_base:f64, 
        right_base:f64, 
        right_leg:f64
    },
    Gaussian {
        mean:f64,
        sd:f64
    },
}

#[derive(Debug)]
pub enum ShapeError {
    BadTriangle,
    BadTrapezoid,
    BadGaussian,
}


impl MembershipShape{

    //Initialization
    pub fn triangle(l:f64,c:f64,r:f64) -> Result<Self, ShapeError>{
        if l < c && c < r {Ok(Self::Triangle{left:l, center:c, right:r})}
        else {Err(ShapeError::BadTriangle)}
    }
    pub fn trapezoid(l_l:f64,l_b:f64,r_b:f64, r_l:f64) -> Result<Self, ShapeError>{
        if l_l < l_b && l_b < r_b && r_b < r_l {Ok(Self::Trapezoid {left_leg: (l_l), left_base: (l_b), right_base: (r_b), right_leg: (r_l)})}
        else {Err(ShapeError::BadTrapezoid)}
    }
    pub fn gaussian(m:f64,sd:f64) -> Result<Self, ShapeError>{
        if sd> 0.0 {Ok(Self::Gaussian { mean: m, sd: sd })}
        else {Err(ShapeError::BadGaussian)}
    }



    ///Calculates the membership value for each of the shape defined in enum Membershipshape.
    ///TODO: error handing for invalid cases(i.e.: left > center > right)
    pub fn membership(&self, value:f64) -> f64{

        ///helper function for calculating slopes.
        ///delta is the displacement of y.(since this is a membership function, its only going to be either 1.0 or -1.0)
        fn slope(value: f64, left:f64, right:f64, delta:f64) -> f64{
            (delta*(value - left)/(right - left)+ ((-1.0*delta + 1.0) / 2.0)).clamp(0.0, 1.0)
        }

        match self {
            //membership calculation for Triangular Membership Function
            MembershipShape::Triangle { left , center, right } => {
                //out of bounds check
                if value <= *left  {return 0.0}
                if value >= *right {return 0.0}

                //calcultation within membership function
                if (value - *center).abs() < 1e-9      {1.0}
                else if value < *center {slope(value, *left, *center,1.0)}
                else                    {slope(value, *center, *right, -1.0)}
            },

            //membership calculation for Trapezoidal Membership Function
            MembershipShape::Trapezoid { left_leg, left_base, right_base, right_leg } => {
                //out of bounds check
                if value <= *left_leg  {return 0.0}
                if value >= *right_leg {return 0.0}
 
                //calculation within the membership function
                if (value - *left_base).abs() < 1e-9  || (value - *right_base).abs() < 1e-9    {1.0}
                else if value < *left_base                       {slope(value, *left_leg, *left_base, 1.0)}
                else if value > *left_base && value < *right_base{1.0}
                else                                             {slope(value, *right_base, *right_leg, -1.0)}
            },

            //membership calculation for Gaussian Membership funciton.
            MembershipShape::Gaussian { mean, sd } => {
                ((value - mean).powi(2) / (-2.0*sd.powi(2))).exp()
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    //test for triangle membership calculation
    fn test_triangle(){
        let test_triangle = MembershipShape::triangle(2.0, 3.0, 4.0).unwrap();
        let mut i:f64 = 0.0;
        while i < 5.0 {
        println!("triangular membership value for i={} is {}",i,test_triangle.membership(i));
        i+=0.1;
        }
    }

    #[test]
    //test for trapezoidal membership calculation
    fn test_trapezoid(){
        let test_trapezoid = MembershipShape::trapezoid(1.0, 2.0, 3.0, 4.0).unwrap();
        let mut i:f64 = 0.0;
        while i < 5.0 {
        println!("trapezoid membership value for i={} is {}",i,test_trapezoid.membership(i));
        i+=0.1;
        }
    }

    #[test]
    fn test_gaussian(){
        let test_gaussian = MembershipShape::gaussian(2.5, 1.0).unwrap();
        let mut i:f64 = 0.0;
        while i < 5.0 {
        println!("gaussian membership value for i={} is {}",i,test_gaussian.membership(i));
        i+=0.1;
        }

    }
}