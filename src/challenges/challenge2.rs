use  super::challenge1::triangular_membership;
pub struct TemperatureSet{
    pub name: String,
    pub triangle_left: f64,
    pub triangle_center: f64,
    pub triangle_right: f64,
}

impl TemperatureSet{
    ///Compares a given f64 to the membership function and Calculates the match with the Set
    pub fn what_is_this_temperature(&self, value: f64){
        let degree = triangular_membership(value, self.triangle_left, self.triangle_center, self.triangle_right);
        println!("a temperature of {} is {} degree of {}", value, degree, self.name);
    }

    pub fn get_membership_degree(&self, value:f64) -> f64{
        let degree = triangular_membership(value, self.triangle_left, self.triangle_center, self.triangle_right);
        return degree
    }

    ///Adjust the Triangle point of the membership function
    pub fn adjust(&mut self, left:f64, center:f64, right:f64 ){
        self.triangle_left = left;
        self.triangle_center = center;
        self.triangle_right =  right;
    }

    ///Prints each data to line out
    pub fn info(&self){
        println!("name:{}, triangle_left:{}, triangle_center:{}, triangle_right:{}", self.name, self.triangle_left, self.triangle_center, self.triangle_right);
    }

}

#[cfg(test)]
mod tests {
    use super::*;  // This imports your function
    //use  super::challenge1::triangular_membership;

    #[test]
    fn test_adjustments(){
        let mut test1=   TemperatureSet{
            name: String::from("Hot"),
            triangle_left: 27.0,
            triangle_center: 32.0,
            triangle_right: 36.0,
        };
        test1.info();
        test1.adjust(25.9, 30.0, 40.0);
        test1.info();
    }

    #[test]
    fn test_edge_cases(){
        let mut test1=   TemperatureSet{
            name: String::from("Hot"),
            triangle_left: 27.0,
            triangle_center: 32.0,
            triangle_right: 36.0,
        };
        test1.adjust(1.0,2.0,3.0);
        test1.what_is_this_temperature(1.0);
        test1.what_is_this_temperature(2.0);
        test1.what_is_this_temperature(3.0);
        test1.what_is_this_temperature(2999991.0);
    }
    





}
