use std::cell::RefCell;
use goal_log::{tip, description};

#[description("Implement a struct that caches the result of a computation using RefCell to store and mutate the \
cached value internally.")]
#[tip("Caching using RefCell is a classic use case in Rust when you want to defer a computation and store the result, \
while keeping the API clean and immutable from the outside.")]
pub fn refcell_with_pattern() {

    struct ExpensiveCalculator {
        cached_result: RefCell<Option<u32>>,
    }

    impl ExpensiveCalculator {
        fn new() -> Self {
            Self {
                cached_result: RefCell::new(None),
            }
        }

        fn compute(&self) -> u32 {
            if let Some(value) = *self.cached_result.borrow() {
                println!("Returning cached result.");
                return value;
            }

            let result = 42; // Simulated expensive computation
            *self.cached_result.borrow_mut() = Some(result);
            println!("Computed and cached result.");
            result
        }
    }

    let calculator = ExpensiveCalculator::new();
    println!("First call: {}", calculator.compute());
    println!("Second call: {}", calculator.compute());
}
