pub struct temp_temp{
    pub temp_vec : Vec<f64>,
    min: f64,
    max: f64,
    avg: f64, 
}

impl temp_temp{
    pub fn new(init_vec: Vec<f64>) -> Self{
        let sum: f64 = init_vec.iter().sum();
        let avg: f64 =  sum / init_vec.len() as f64;

        let min =
        init_vec.iter().copied().fold(f64::INFINITY,
        f64::min);

        let max =
        init_vec.iter().copied().fold(f64::NEG_INFINITY,
        f64::max);

        temp_temp { temp_vec: init_vec, min: min, max: max, avg: avg }
    }

    pub fn tempvector_avg(&self)  -> (f64){
        self.avg
    }

    pub fn get_min(&self)  -> (f64){
        self.min
    }

    pub fn get_max(&self)  -> (f64){
        self.max
    }
}


/// Average of Every value within f64 Vector
pub fn vector_avg(input: Vec<f64>)  -> (f64, Vec<f64>){
    let temp = temp_temp::new(input);

    //return the result along with the input
    (temp.avg, temp.temp_vec)
}

#[cfg(test)]
mod tests {
    use super::*;  // This imports your function
    #[test]
    fn test1(){
        
        let test_vec = vec![23.3, 25.5, 26.9, 27.0, 28.0];
        //    let temp = temp_temp::new(input);

        let test_temp = temp_temp::new(test_vec.clone());

        let result= test_temp.tempvector_avg();

        println!("{}", result);
        println!("{:?}", test_temp.get_max());
        println!("{:?}", test_temp.get_min());

        let (result, test_vec) = vector_avg(test_vec);
        println!("{}", result);
        println!("{:?}", test_vec);
    }
}
