use goal_log::{tip, description};
use std::cell::RefCell;

#[description("This exercise demonstrates how to create a RefCell containing an integer and mutate its value using \
the borrow_mut() method.")]
#[tip("RefCell isn’t really a 'smart pointer' because it isn’t a pointer")]
#[tip("RefCell allows you to mutate data even when the RefCell itself is not mutable, thanks to interior mutability. \
This is especially useful when you're dealing with APIs that require immutable references but you still need to change \
some data.")]
pub fn basic_refcell_usage() {

    let number = RefCell::new(10);
    *number.borrow_mut() += 5;

    println!("The number is now: {}", number.borrow());
}

#[description("This exercise shows how RefCell enforces Rust's borrowing rules at runtime. Trying to borrow a value \
immutably while a mutable borrow is still active will panic. Use an inner block or drop() to explicitly end the mutable \
borrow before borrowing immutably.")]
#[tip("RefCell does not allow multiple overlapping borrows that would violate Rust's safety model. Even though the borrow \
checker allows it at compile time, RefCell will panic at runtime if you try to borrow mutably and immutably at the same \
time. This protects against data races in single-threaded code.")]
#[tip("To end a RefCell borrow early, use a nested scope or call drop(borrow). Rust will only release the borrow when the \
last handle (Ref or RefMut) goes out of scope.")]
pub fn refcell_exclusive_borrowing_runtime_check() {
    use std::cell::RefCell;

    let value = RefCell::new(100);

    {
        // Start a mutable borrow in a limited scope
        let mut first_borrow = value.borrow_mut();
        *first_borrow += 1;
        // first_borrow is dropped here when it goes out of scope
    }

    // Now it's safe to borrow immutably
    println!("Value after mutation: {}", value.borrow());
}

#[description("In this exercise, you'll define a struct that uses RefCell to allow mutation of one of its fields even \
when the struct instance is immutable.")]
#[tip("Placing RefCell inside a struct is a common way to bypass the usual restrictions of immutability in Rust. \
It’s particularly handy when designing APIs where you want the struct to appear immutable but still allow internal \
changes.")]
pub fn refcell_inside_struct() {

    struct Counter {
        count: RefCell<u32>,
    }

    let my_counter = Counter {
        count: RefCell::new(0),
    };

    *my_counter.count.borrow_mut() += 1;
    println!("Counter is now: {}", my_counter.count.borrow());
}


#[description("This exercise demonstrates how try_borrow_mut can be used to safely attempt a mutable borrow without \
causing a panic. Make sure to release any existing mutable borrow before attempting a second one. Use block scoping to \
control the lifetime of borrows.")]
#[tip("Even try_borrow_mut will fail if another mutable or immutable borrow is active. You must ensure that all previous \
borrows have ended (gone out of scope) before attempting a new one.")]
pub fn refcell_error_handling() {

    let data = RefCell::new(vec![1, 2, 3]);

    // Start a mutable borrow in a limited scope
    {
        let _first = data.borrow_mut(); // Borrow ends at the end of this block
        // Do something with _first if needed
    }

    // Now we attempt a second mutable borrow using try_borrow_mut
    let second_attempt = data.try_borrow_mut();

    match second_attempt {
        Ok(mut second) => {
            // borrowed is active
            second.push(4)
        }, // <- borrowed is dropped here
        Err(e) => println!("Could not borrow mutably: {}", e),
    }

    println!("Data: {:?}", data.borrow());
}