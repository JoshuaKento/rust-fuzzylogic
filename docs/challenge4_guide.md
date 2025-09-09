# Challenge 4 Guide: "Shape Shifter"

## 🌟 What You're Building
You're going to create a system that can handle different types of membership function shapes - not just triangular ones! Think of it like having a toolbox where each tool (shape) is different but they all do similar jobs. Your system should be able to work with triangular, trapezoidal, and Gaussian shapes seamlessly.

## 🎯 Challenge Overview
Create an enum called `MembershipShape` with variants for different shape types. Each variant should store the parameters needed for that specific shape. Then implement a method that can calculate membership values regardless of which shape type you're working with.

## 💡 New Rust Concepts You'll Learn

### What are Enums with Data?
Enums in Rust are incredibly powerful - they're not just simple lists like in other languages. Each variant can hold different types of data!

**Example - Basic Enum:**
```rust
enum Color {
    Red,
    Green,
    Blue,
}

// Using it
let favorite_color = Color::Blue;
```

**Example - Enum with Data:**
```rust
enum Message {
    Quit,                           // No data
    Move { x: i32, y: i32 },       // Struct-like data
    Write(String),                  // Tuple-like data
    ChangeColor(u8, u8, u8),       // Multiple values
}

// Creating instances
let msg1 = Message::Quit;
let msg2 = Message::Move { x: 10, y: 20 };
let msg3 = Message::Write(String::from("Hello"));
let msg4 = Message::ChangeColor(255, 0, 128);
```

**Example - Geometric Shapes:**
```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.14159 * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

// Usage
let circle = Shape::Circle { radius: 5.0 };
let rect = Shape::Rectangle { width: 10.0, height: 8.0 };
println!("Circle area: {}", circle.area());
println!("Rectangle area: {}", rect.area());
```

### Pattern Matching with Match
The `match` expression is Rust's superpower for handling different enum variants:

**Example - Matching Different Cases:**
```rust
enum Vehicle {
    Car { fuel_type: String, doors: u8 },
    Bike { has_motor: bool },
    Plane { passenger_capacity: u32 },
}

fn describe_vehicle(vehicle: &Vehicle) -> String {
    match vehicle {
        Vehicle::Car { fuel_type, doors } => {
            format!("A {} car with {} doors", fuel_type, doors)
        }
        Vehicle::Bike { has_motor: true } => {
            String::from("A motorcycle")
        }
        Vehicle::Bike { has_motor: false } => {
            String::from("A bicycle")
        }
        Vehicle::Plane { passenger_capacity } => {
            format!("A plane that holds {} passengers", passenger_capacity)
        }
    }
}

// Usage
let my_car = Vehicle::Car { 
    fuel_type: String::from("electric"), 
    doors: 4 
};
let my_bike = Vehicle::Bike { has_motor: false };

println!("{}", describe_vehicle(&my_car));
println!("{}", describe_vehicle(&my_bike));
```

**Example - Handling Options:**
```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    let numbers = vec![1, 3, 4, 7, 8];
    
    match find_first_even(&numbers) {
        Some(even_number) => println!("Found even number: {}", even_number),
        None => println!("No even numbers found"),
    }
}
```

### Understanding Different Mathematical Shapes
Before implementing, understand what each membership function shape looks like:

**Triangular Function:**
- Has three points: left, center (peak), right
- Forms a triangle shape: goes up linearly, then down linearly
- Most basic and common shape

**Trapezoidal Function:**
- Has four points: left, left_top, right_top, right
- Forms a trapezoid: goes up, stays flat at top, then goes down
- Good for representing ranges with a "definitely belongs" region

**Gaussian (Bell Curve) Function:**
- Has two parameters: center (mean) and width (standard deviation)
- Forms a smooth bell curve
- Never actually reaches zero, but gets very close
- Good for natural phenomena

## 🔍 Step-by-Step Planning

### Step 1: Design Your Enum Structure
Think about what data each shape needs:
- What parameters does a triangular function need?
- What about trapezoidal? (Hint: it needs more points)
- What about Gaussian? (Hint: it's quite different from the others)

Consider different ways to structure your enum:
```rust
// Option 1: Named fields
enum MembershipShape {
    Triangular { left: f64, center: f64, right: f64 },
    // ... other variants
}

// Option 2: Tuple fields
enum MembershipShape {
    Triangular(f64, f64, f64),
    // ... other variants
}

// Option 3: Mixed approach
enum MembershipShape {
    Triangular { left: f64, center: f64, right: f64 },
    Gaussian(f64, f64), // center, std_dev
}
```

### Step 2: Implement the Calculation Logic
Your enum should have a method that calculates membership regardless of shape:
```rust
impl MembershipShape {
    fn calculate_membership(&self, value: f64) -> f64 {
        match self {
            // Handle each shape type differently
            // Each case will have different math
        }
    }
}
```

### Step 3: Handle Edge Cases
Consider what happens when:
- Input value is far outside the shape's range
- Parameters define invalid shapes (like left > right)
- Floating point precision issues occur
- Division by zero might happen

### Step 4: Create Helper Methods
Consider additional useful methods:
```rust
impl MembershipShape {
    // Method to check if shape parameters are valid
    fn is_valid(&self) -> bool {
        // Validation logic for each shape type
    }
    
    // Method to get the peak membership value location
    fn peak_location(&self) -> f64 {
        // Different for each shape type
    }
    
    // Method to get the range where membership > 0
    fn active_range(&self) -> (f64, f64) {
        // Shape-specific logic
    }
}
```

## 🧪 Testing Strategies

### Test Each Shape Type
Create comprehensive tests for all shapes:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_basic_cases() {
        let tri = MembershipShape::Triangular { 
            left: 0.0, center: 5.0, right: 10.0 
        };
        
        // Test key points
        assert_eq!(tri.calculate_membership(0.0), 0.0);   // Left edge
        assert_eq!(tri.calculate_membership(5.0), 1.0);   // Center peak
        assert_eq!(tri.calculate_membership(10.0), 0.0);  // Right edge
        
        // Test intermediate points
        assert_eq!(tri.calculate_membership(2.5), 0.5);   // Midway up
        assert_eq!(tri.calculate_membership(7.5), 0.5);   // Midway down
        
        // Test outside range
        assert_eq!(tri.calculate_membership(-1.0), 0.0);  // Below range
        assert_eq!(tri.calculate_membership(11.0), 0.0);  // Above range
    }

    #[test]
    fn test_trapezoidal_flat_top() {
        let trap = MembershipShape::Trapezoidal { 
            left: 0.0, left_top: 3.0, right_top: 7.0, right: 10.0 
        };
        
        // Test that the flat top region all returns 1.0
        assert_eq!(trap.calculate_membership(3.0), 1.0);
        assert_eq!(trap.calculate_membership(5.0), 1.0);
        assert_eq!(trap.calculate_membership(7.0), 1.0);
    }

    #[test]
    fn test_gaussian_smooth_curve() {
        let gauss = MembershipShape::Gaussian { center: 0.0, std_dev: 1.0 };
        
        // Test center point
        assert_eq!(gauss.calculate_membership(0.0), 1.0);
        
        // Test symmetry
        let left_val = gauss.calculate_membership(-1.0);
        let right_val = gauss.calculate_membership(1.0);
        assert!((left_val - right_val).abs() < 0.0001);
    }
}
```

### Test Edge Cases
```rust
#[test]
fn test_invalid_parameters() {
    // What happens with invalid triangular shape?
    let bad_tri = MembershipShape::Triangular { 
        left: 10.0, center: 5.0, right: 0.0  // Wrong order!
    };
    
    // How should your system handle this?
}

#[test]
fn test_extreme_values() {
    let tri = MembershipShape::Triangular { 
        left: 0.0, center: 5.0, right: 10.0 
    };
    
    // Test with very large/small numbers
    assert_eq!(tri.calculate_membership(f64::INFINITY), 0.0);
    assert_eq!(tri.calculate_membership(f64::NEG_INFINITY), 0.0);
    
    // Test with NaN
    let result = tri.calculate_membership(f64::NAN);
    assert!(result.is_nan() || result == 0.0);  // Either is acceptable
}
```

### Test Practical Usage
```rust
#[test]
fn test_mixed_shape_usage() {
    let shapes = vec![
        MembershipShape::Triangular { left: 0.0, center: 25.0, right: 50.0 },
        MembershipShape::Trapezoidal { left: 20.0, left_top: 40.0, right_top: 60.0, right: 80.0 },
        MembershipShape::Gaussian { center: 50.0, std_dev: 10.0 },
    ];
    
    let test_value = 30.0;
    
    for (i, shape) in shapes.iter().enumerate() {
        let membership = shape.calculate_membership(test_value);
        println!("Shape {}: membership = {}", i, membership);
        assert!(membership >= 0.0 && membership <= 1.0);
    }
}
```

## 📋 Best Practices for This Challenge

### Enum Design Guidelines
- **Use descriptive variant names** that clearly indicate the shape type
- **Choose appropriate data structures** - named fields vs tuples based on clarity
- **Consider parameter validation** - should invalid shapes be allowed?
- **Think about extensibility** - how easy is it to add new shapes later?

### Pattern Matching Best Practices
```rust
// Good: Exhaustive matching
match shape {
    MembershipShape::Triangular { left, center, right } => {
        // Handle triangular case
    }
    MembershipShape::Trapezoidal { left, left_top, right_top, right } => {
        // Handle trapezoidal case
    }
    MembershipShape::Gaussian { center, std_dev } => {
        // Handle Gaussian case
    }
    // Rust ensures you handle all cases!
}

// Good: Use guards for additional conditions
match shape {
    MembershipShape::Triangular { left, center, right } if left < center && center < right => {
        // Handle valid triangular shape
    }
    MembershipShape::Triangular { .. } => {
        // Handle invalid triangular shape
    }
    // ... other cases
}

// Good: Extract common logic
impl MembershipShape {
    fn calculate_membership(&self, value: f64) -> f64 {
        // Validate input first (common to all shapes)
        if value.is_nan() {
            return 0.0;
        }
        
        match self {
            // Then handle each shape specifically
            MembershipShape::Triangular { left, center, right } => {
                self.triangular_membership(value, *left, *center, *right)
            }
            // ... other cases
        }
    }
    
    fn triangular_membership(&self, value: f64, left: f64, center: f64, right: f64) -> f64 {
        // Actual triangular calculation
    }
}
```

### Mathematical Implementation Tips
- **Handle division by zero** carefully, especially in triangular calculations
- **Use appropriate floating-point comparisons** - don't use `==` for f64
- **Consider numerical stability** for edge cases
- **Think about performance** - avoid expensive operations if possible

### Error Handling Philosophy
```rust
// Option 1: Return 0.0 for invalid cases (simple)
fn calculate_membership(&self, value: f64) -> f64 {
    if !self.is_valid() {
        return 0.0;
    }
    // ... calculation
}

// Option 2: Use Result type (more explicit)
fn calculate_membership(&self, value: f64) -> Result<f64, String> {
    if !self.is_valid() {
        return Err("Invalid shape parameters".to_string());
    }
    // ... calculation
    Ok(result)
}

// Option 3: Panic on invalid construction (fail-fast)
impl MembershipShape {
    fn new_triangular(left: f64, center: f64, right: f64) -> Self {
        assert!(left <= center && center <= right, "Invalid triangular parameters");
        MembershipShape::Triangular { left, center, right }
    }
}
```

## 🎨 Design Patterns to Explore

### Pattern 1: Simple and Direct
```rust
enum MembershipShape {
    Triangular(f64, f64, f64),  // left, center, right
    Trapezoidal(f64, f64, f64, f64), // left, left_top, right_top, right
    Gaussian(f64, f64), // center, std_dev
}

impl MembershipShape {
    fn calculate_membership(&self, value: f64) -> f64 {
        match self {
            MembershipShape::Triangular(left, center, right) => {
                // Direct calculation
            }
            // ... other cases
        }
    }
}
```

### Pattern 2: Named Fields for Clarity
```rust
enum MembershipShape {
    Triangular { 
        left: f64, 
        center: f64, 
        right: f64 
    },
    Trapezoidal { 
        left: f64, 
        left_top: f64, 
        right_top: f64, 
        right: f64 
    },
    Gaussian { 
        center: f64, 
        std_dev: f64 
    },
}
```

### Pattern 3: Builder Methods
```rust
impl MembershipShape {
    fn triangular(left: f64, center: f64, right: f64) -> Self {
        MembershipShape::Triangular { left, center, right }
    }
    
    fn trapezoidal(left: f64, left_top: f64, right_top: f64, right: f64) -> Self {
        MembershipShape::Trapezoidal { left, left_top, right_top, right }
    }
    
    fn gaussian(center: f64, std_dev: f64) -> Self {
        MembershipShape::Gaussian { center, std_dev }
    }
}

// Usage
let temp_cold = MembershipShape::triangular(0.0, 10.0, 20.0);
let temp_warm = MembershipShape::trapezoidal(15.0, 20.0, 30.0, 35.0);
let temp_hot = MembershipShape::gaussian(40.0, 5.0);
```

## 🤔 Common Beginner Questions

**Q: "How do I decide between named fields and tuple fields?"**
A: Use named fields when the meaning isn't obvious from position. `Gaussian { center, std_dev }` is clearer than `Gaussian(f64, f64)`.

**Q: "What if I want to add more shape types later?"**
A: That's the beauty of enums! Just add new variants. The compiler will tell you everywhere you need to update your match statements.

**Q: "Should I validate parameters when creating shapes?"**
A: It depends on your design philosophy. Validating early (at construction) catches errors sooner but requires more careful API design.

**Q: "How do I handle the math for Gaussian curves?"**
A: The formula involves exponentials and squares. Look up the mathematical definition of a Gaussian/normal distribution function. Don't worry if it seems complex - break it into small steps!

**Q: "What's the difference between `match` and `if let`?"**
A: `match` is exhaustive (handles all cases), `if let` is for when you only care about one specific case.

## 📊 Evaluation Criteria

### Core Functionality (40 points)
- [ ] Enum defined with appropriate variants for all three shape types
- [ ] Each variant stores the necessary parameters for its shape
- [ ] `calculate_membership` method implemented for all shapes
- [ ] Mathematical calculations are correct for each shape type

### Pattern Matching (25 points)
- [ ] Proper use of `match` expressions to handle different enum variants
- [ ] Exhaustive pattern matching (all cases handled)
- [ ] Clean, readable match arm implementations
- [ ] Appropriate use of destructuring in patterns

### Code Quality (20 points)
- [ ] Clear, descriptive naming for enum variants and fields
- [ ] Proper error handling for edge cases
- [ ] Well-organized code structure
- [ ] Appropriate use of Rust conventions

### Edge Case Handling (15 points)
- [ ] Handles invalid input values (NaN, infinity, etc.)
- [ ] Deals with invalid shape parameters appropriately
- [ ] Numerical stability for boundary conditions
- [ ] Consistent behavior across all shape types

### Testing (Bonus 10 points)
- [ ] Comprehensive test coverage for all shape types
- [ ] Edge case testing
- [ ] Clear, descriptive test names
- [ ] Tests verify both correctness and error handling

## 🚀 Extension Ideas (After Completion)

1. **Custom Shape Types**: Add support for step functions, sigmoid curves, or custom polynomial shapes
2. **Shape Validation**: Implement comprehensive parameter validation with custom error types
3. **Shape Composition**: Allow combining multiple shapes with boolean operations (union, intersection)
4. **Optimization**: Implement caching for expensive calculations like Gaussian functions
5. **Serialization**: Make your shapes serializable to/from JSON or other formats
6. **Visualization**: Generate data points that could be plotted to visualize the shapes

## 💪 Remember: The Enum Journey

Enums with data are one of Rust's most powerful features:
- **They're type-safe**: The compiler prevents you from accessing the wrong variant's data
- **They're expressive**: You can model complex data relationships clearly
- **They're efficient**: Rust optimizes enum memory layout automatically
- **They enable pattern matching**: One of Rust's most elegant features

**Take your time to understand pattern matching - it's a superpower that will make your Rust code both safer and more expressive! 🦀💪**

This challenge will teach you to think in terms of algebraic data types, which is a fundamental skill for writing idiomatic Rust code. Master this, and you'll see elegant solutions everywhere!