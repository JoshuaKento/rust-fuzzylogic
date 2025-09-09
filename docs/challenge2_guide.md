# Challenge 2 Guide: "Temperature Classifier"

## 🌟 What You're Building
You're going to create a "temperature classifier" that can decide if temperatures are "cold", "warm", or "hot". Think of it like a smart thermometer that doesn't just tell you the temperature, but also tells you what that temperature *means*.

## 🎯 Challenge Overview
Build a struct called `TemperatureSet` that stores a name (like "cold" or "warm") and the boundary points for that category. Then create methods to check if any given temperature belongs to that category.

## 💡 New Rust Concepts You'll Learn

### What is a Struct?
A struct is like a container that holds related information together. Think of it like a form with different fields.

**Example - A Car struct:**
```rust
struct Car {
    brand: String,
    model: String,
    year: u32,
    mileage: f64,
}

// Creating a car
let my_car = Car {
    brand: String::from("Toyota"),
    model: String::from("Camry"), 
    year: 2020,
    mileage: 25000.5,
};

// Accessing fields
println!("My car is a {} {}", my_car.year, my_car.brand);
```

**Example - A Person struct:**
```rust
#[derive(Debug)]  // This lets you print the struct easily
struct Person {
    name: String,
    age: u32,
    height_cm: f64,
}

let alice = Person {
    name: String::from("Alice"),
    age: 25,
    height_cm: 165.5,
};

println!("{:?}", alice);  // Prints: Person { name: "Alice", age: 25, height_cm: 165.5 }
```

### What are Methods?
Methods are functions that belong to a struct. It's like giving your struct special abilities.

**Example - Dog struct with methods:**
```rust
struct Dog {
    name: String,
    breed: String,
    age: u32,
}

impl Dog {
    // Method that doesn't change the dog
    fn bark(&self) {
        println!("{} says: Woof! Woof!", self.name);
    }
    
    // Method that tells us something about the dog
    fn is_puppy(&self) -> bool {
        self.age < 2
    }
    
    // Method that changes the dog  
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("{} is now {} years old!", self.name, self.age);
    }
}

// Using the methods
let mut my_dog = Dog {
    name: String::from("Rex"),
    breed: String::from("Golden Retriever"),
    age: 1,
};

my_dog.bark();           // Rex says: Woof! Woof!
println!("{}", my_dog.is_puppy());  // true
my_dog.have_birthday();  // Rex is now 2 years old!
```

**Example - Calculator struct:**
```rust
struct Calculator {
    result: f64,
}

impl Calculator {
    // Create a new calculator
    fn new() -> Calculator {
        Calculator { result: 0.0 }
    }
    
    // Add a number
    fn add(&mut self, value: f64) {
        self.result += value;
    }
    
    // Get the current result
    fn get_result(&self) -> f64 {
        self.result
    }
    
    // Reset to zero
    fn clear(&mut self) {
        self.result = 0.0;
    }
}

// Using the calculator
let mut calc = Calculator::new();
calc.add(5.0);
calc.add(3.0);
println!("Result: {}", calc.get_result());  // Result: 8
```

### impl Blocks
An `impl` block is where you write the methods for your struct. Think of it as the "instruction manual" for what your struct can do.

**Example - Understanding &self vs &mut self:**
```rust
struct BankAccount {
    balance: f64,
    account_number: String,
}

impl BankAccount {
    // &self = "I just want to READ the data, not change it"
    fn get_balance(&self) -> f64 {
        self.balance  // Just looking at the balance
    }
    
    fn get_account_number(&self) -> &String {
        &self.account_number  // Just looking at the account number
    }
    
    // &mut self = "I want to CHANGE the data"
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;  // Changing the balance
        println!("Deposited ${}, new balance: ${}", amount, self.balance);
    }
    
    fn withdraw(&mut self, amount: f64) -> bool {
        if amount <= self.balance {
            self.balance -= amount;  // Changing the balance
            println!("Withdrew ${}, new balance: ${}", amount, self.balance);
            true
        } else {
            println!("Insufficient funds!");
            false
        }
    }
}

// Using the bank account
let mut account = BankAccount {
    balance: 100.0,
    account_number: String::from("123456789"),
};

println!("Balance: ${}", account.get_balance());  // Reading - uses &self
account.deposit(50.0);                          // Changing - uses &mut self  
account.withdraw(25.0);                         // Changing - uses &mut self
```

**Key Points:**
- `&self` = "I'm just looking" (read-only)
- `&mut self` = "I'm going to change something" (read-write)
- The struct needs to be `mut` if you want to call `&mut self` methods

## 🔍 Step-by-Step Planning

### Step 1: Design Your Struct
Think about what information a temperature classifier needs:
- A name (like "cold", "warm", "hot")
- The three boundary points (left, center, right)

### Step 2: Create Methods
Your struct should be able to:
- Check if a temperature belongs to its category
- Maybe create new temperature sets easily
- Perhaps display information about itself

### Step 3: Test Different Categories
Create instances for:
- Cold temperatures (maybe 0°C, 10°C, 20°C)
- Warm temperatures (maybe 15°C, 25°C, 35°C)
- Hot temperatures (maybe 30°C, 40°C, 50°C)

## 🎨 Design Decisions to Consider

### Naming Your Fields
Choose clear, descriptive names:
- `name` vs `category` vs `label`
- `left_bound` vs `min_temp` vs `start`
- `center` vs `peak` vs `ideal`

### Method Naming
Think about what sounds natural:
- `is_member()` vs `belongs()` vs `matches()`
- `classify()` vs `evaluate()` vs `check()`

### String Handling
You'll encounter Rust's string types. Here's the difference with examples:

**String vs &str:**
```rust
// String = owned, can be changed, lives as long as you need
struct Book {
    title: String,        // The struct OWNS this string
    author: String,       // Can be modified, moved around
    pages: u32,
}

fn create_book() -> Book {
    Book {
        title: String::from("The Rust Book"),    // Creates owned String
        author: "Steve Klabnik".to_string(),     // Another way to create String
        pages: 500,
    }
}

// &str = borrowed, read-only, temporary reference
fn print_book_info(title: &str, author: &str) {  // Borrows strings temporarily
    println!("Book: {} by {}", title, author);
}

let my_book = create_book();
print_book_info(&my_book.title, &my_book.author);  // Borrow the strings

// You can also create &str directly from text
let message: &str = "Hello, world!";  // This lives in the program's memory
```

**When to use which:**
- **Use `String` in structs** - when the struct should own the text
- **Use `&str` in function parameters** - when you just want to read the text
- **Use `&str` for string literals** - like `"hello"` in your code

## 🧪 Testing Strategies

### Test Your Struct Creation
- Can you create new temperature sets?
- Do the fields store the right values?
- Can you access the information you stored?

### Test Your Methods
- Does "cold" correctly identify cold temperatures?
- What happens at boundary values?
- Do overlapping categories work as expected?

### Test Edge Cases
- Very high or low temperatures
- Temperatures exactly at boundary points
- Invalid boundary configurations

## 📋 Rust Best Practices for This Challenge

### Struct Organization
- Put related data together
- Use meaningful field names
- Consider what should be public vs private

### Method Design
- Keep methods focused on one task
- Use `&self` when you don't need to modify the struct
- Return meaningful types (bool, f64, etc.)

### Error Prevention
- Think about what could go wrong
- Validate your boundary points make sense
- Handle edge cases gracefully

### Code Style
- Use consistent naming (snake_case for functions, PascalCase for structs)
- Add documentation comments with `///`
- Group related functionality together

## 🤔 Common Beginner Questions

**Q: Should I use String or &str for the name?**
A: For struct fields, usually `String` because the struct owns the data

**Q: How do I make my struct printable?**
A: Add `#[derive(Debug)]` above your struct definition

**Q: Can I reuse my triangular_membership function?**
A: Absolutely! Import it and use it in your methods

**Q: What if two categories overlap?**
A: That's normal in fuzzy logic! A temperature can be 60% "warm" and 30% "hot"

## 🎯 Success Criteria

By the end of this challenge, you should have:
- A working `TemperatureSet` struct
- Methods that classify temperatures correctly
- Test code that proves everything works
- Clean, readable code with good names

## 🚀 Extension Ideas (After You Finish)

- Create a `TemperatureClassifier` that holds multiple sets
- Add a method that finds the "best match" category
- Support different temperature scales (Celsius, Fahrenheit)
- Add validation to ensure boundary points are in order

## 💪 Remember
- Start with the simplest version that works
- Write tests early to check your progress
- Use the Rust compiler as your guide - it will help you!
- Don't be afraid to refactor and improve your design

This challenge will teach you fundamental Rust concepts that you'll use everywhere: structs, methods, and organizing code. Take your time and make sure you understand each part before moving on!

**You've got this! 🦀**