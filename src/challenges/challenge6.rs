//### Challenge 6: "Error-Proof Input Parser"
//Write a parser that reads membership function definitions
//from strings like `"triangular(0,5,10)"` or `"gaussian(5,2)"`.
//Handle all possible parsing errors gracefully and return 
//meaningful error messages.

//ToDo: Shape Validation implementaiton within fn from_str...
//      --Error Coverage (ex. Unknown func, Empty Input, ...)
//      nagative case test implementation
//      Function Name Extraction in ParseError::UnknownFunc 


use std::fmt;
use std::str::FromStr;
use crate::challenges::challenge4::MembershipShape;

//Error Definintion
#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    UnknownFunc {name:String},
    BadArity,
    NotNumber,
    BadInput,
}


impl fmt::Display for ParseError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            ParseError::EmptyInput => write!(f,"input is empty"),
            ParseError::UnknownFunc {name} => write!(f,"Unknown Function {} Specified", name),
            ParseError::BadArity => write!(f,"Input Shape/Arity is wrong"),
            ParseError::NotNumber=> write!(f,"Args specified are not numbers"),
            ParseError::BadInput=> write!(f,"UnknownError, Bad Input"),
        }
    }
}
impl std::error::Error for ParseError{}

//Definition of Membership Functions
#[derive(Debug, PartialEq)]
enum Membership{
    Triangle {l:f64,c:f64,r:f64,},
    Trapezoid{ll:f64, lb:f64, rb:f64, rl:f64},
    Gaussian{mean:f64, sd:f64},
}

//Standardizing function for the String input to lowercase, no spaces
fn canonical(s: &str) -> String{
    s.to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

//Implementing Parse Funcitonality
impl FromStr for Membership{
    type Err = ParseError;

    //defines the functionality when parse() is called
    fn from_str(raw: &str) -> Result<Self, Self::Err>{
        let s = canonical(raw);

        //Error When Empty Input
        if s.is_empty(){return Err(ParseError::EmptyInput);}

        //parsing for triangle membership
        if let Some(body) = s.strip_prefix("triangle(").and_then(|b| b.strip_suffix(')')) {
            let nums: Vec<f64> = body.split(',')
                                     .map(str::parse)
                                     .collect::<Result<_,_>>()
                                     .map_err(|_| ParseError::NotNumber)?;
            if nums.len() == 3 {return Ok(Membership::Triangle{l:nums[0], c:nums[1], r:nums[2]})}
            return Err(ParseError::BadArity);
        }

        //parsing for trapezoid membership
        if let Some(body) = s.strip_prefix("trapezoid(").and_then(|b| b.strip_suffix(')')){
            let nums: Vec<f64> = body.split(',')
                                     .map(str::parse)
                                     .collect::<Result<_,_>>()
                                     .map_err(|_| ParseError::NotNumber)?;
            if nums.len() == 4 {return Ok(Membership::Trapezoid{ll:nums[0], lb:nums[1], rb:nums[2], rl:nums[3]})}
            return Err(ParseError::BadArity);
        }

        //parsing for gaussian membership
        if let Some(body) = s.strip_prefix("gaussian(").and_then(|b| b.strip_suffix(')')){
            let nums: Vec<f64> = body.split(',')
                                     .map(str::parse)
                                     .collect::<Result<_,_>>()
                                     .map_err(|_| ParseError::NotNumber)?;
            if nums.len() == 2 {return Ok(Membership::Gaussian{mean:nums[0], sd:nums[1]})}
            return Err(ParseError::BadArity);
        }
        Err(ParseError::BadInput)
    }
}

//calls membership calculation from challenge 4
impl Membership{
    fn calculate(&self, x:f64) -> f64{
        match self {
            Membership::Triangle { l, c, r } => {
                MembershipShape::triangle(*l, *c, *r)
                                 .map(|s| s.membership(x))
                                 .unwrap_or(0.0)
            },
            Membership::Trapezoid { ll , lb, rb, rl} => {
                MembershipShape::trapezoid(*ll, *lb, *rb, *rl)
                                 .map(|s| s.membership(x))
                                 .unwrap_or(0.0)
            },
            Membership::Gaussian { mean, sd } =>{
                MembershipShape::gaussian(*mean, *sd)
                                 .map(|s| s.membership(x))
                                 .unwrap_or(0.0)
            }
        }
    }
}
//Parsing function
//accepted input forms => name(args,...)
//comma separation, ignore white space

//Membership Initialization and calculation
//should import from another challenge
//

 #[cfg(test)]
 mod tests{
    use super::*;
    #[test]
    fn test1() -> Result<(), ParseError>{
        //standard i/o test
        let input_1:Membership = "trapezoid(1,2,3,4)".parse().unwrap();
        let input_2:Membership = "triangle(1,2,3)".parse().unwrap();
        let input_3:Membership = "gaussian(1,1)".parse().unwrap();

        assert_eq!(input_1.calculate(3.0), 1.0);

        assert_eq!(input_2.calculate(2.0), 1.0);

        assert_eq!(input_3.calculate(1.0), 1.0);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), ParseError>{
        //Major Error/Exception Tests
        //case sensitivity test
        let input_1:Membership = "trapezoid(1,2,3,4)".parse()?;
        let input_2:Membership = "TRaPeZOid(1,2,3,4)".parse()?;
        assert_eq!(input_1, input_2);

        //space test
        let input_1:Membership = "t r  a pe  z oid    (1,2,3,4)".parse()?;
        let input_2:Membership = "trapezoid(1,2,3,4)".parse()?;
        assert_eq!(input_1, input_2);

        //f64 formatting test
        let input_1:Membership = "trapezoid(1.0,2.0,3.0,4.0)".parse()?;
        let input_2:Membership = "trapezoid(1,2,3,4)".parse()?;
        assert_eq!(input_1, input_2);

        Ok(())
    }

    #[test]
    fn test3() -> Result<(), ParseError>{
        // negative cases
        // empty input
        assert_eq!("".parse::<Membership>().unwrap_err(), ParseError::EmptyInput);

        // bad arity
        assert_eq!("triangle(1,2)".parse::<Membership>().unwrap_err(), ParseError::BadArity);
        assert_eq!("trapezoid(1,2,3)".parse::<Membership>().unwrap_err(), ParseError::BadArity);
        assert_eq!("gaussian(1)".parse::<Membership>().unwrap_err(), ParseError::BadArity);

        // not a number
        assert_eq!("triangle(1,a,3)".parse::<Membership>().unwrap_err(), ParseError::NotNumber);

        // unknown function name (currently mapped to BadInput)
        assert_eq!("bell(1,2,3)".parse::<Membership>().unwrap_err(), ParseError::BadInput);

        // invalid shape ordering parses but yields 0.0 on calculation
        let bad_tri = "triangle(3,2,1)".parse::<Membership>()?;
        assert_eq!(bad_tri.calculate(2.0), 0.0);

        Ok(())
    }
}
