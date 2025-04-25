use goal_log::{tip, description};
use std::cell::RefCell;
use std::rc::Rc;

#[description("Use Rc<RefCell<T>> to create a value that can be mutated even when shared. Clone the Rc and mutate the inner value from multiple owners.")]
#[tip("Rc is used for shared ownership, while RefCell enables interior mutability. You cannot use &mut with Rc, so RefCell is needed for mutation.")]
#[tip("Calling .borrow_mut() gives mutable access to the inner value, but it will panic at runtime if already borrowed mutably or immutably elsewhere.")]
fn basic_shared_mutability() {

    let shared_number = Rc::new(RefCell::new(0));

    let a = Rc::clone(&shared_number);
    let b = Rc::clone(&shared_number);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 2;

    println!("Final value: {}", shared_number.borrow());
}
