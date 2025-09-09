///Calculates the 適応度 of a given value to a triangular membership function.
///
/// Returns a f64 value between 0 and 1 where value represents 適応度
pub fn triangular_membership(value: f64, left: f64, center: f64, right: f64) -> f64 {
    //out of bounds check
    if value <= left {return 0.0;}
    if value >= right {return 0.0;}

    //calculation of fuzzy values within triangle
    //一次方程式を整理すると、下記の変数宣言で取っている形となります。
    //TODO: error handing for invalid cases(i.e.: left > center > right)
    if value == center{ return 1.0;}
    if value < center { 
        let upslope: f64 = (value - left)/(center-left);    
        return upslope;
    } else {
        let downslope: f64 = -(value - right)/(right-center);    
        return downslope;
    }
}

#[cfg(test)]
mod tests {
    use super::*;  // This imports your function

    #[test]
    fn test_center_point() {
        let result = triangular_membership(10.0, 0.0, 10.0, 20.0);
        assert_eq!(result, 1.0);
    }
    
    #[test] 
    fn test_left_slope() {
        let result = triangular_membership(5.0, 0.0, 10.0, 20.0);
        assert_eq!(result, 0.5);
    }
}
