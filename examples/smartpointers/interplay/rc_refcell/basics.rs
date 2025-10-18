use goal_log::{tip, description};
use std::cell::RefCell;
use std::rc::Rc;

#[description("Use Rc<RefCell<T>> to create a value that can be mutated even when shared. Clone the Rc and mutate the inner value from multiple owners.")]
#[tip("Rc is used for shared ownership, while RefCell enables interior mutability. You cannot use &mut with Rc, so RefCell is needed for mutation.")]
#[tip("Calling .borrow_mut() gives mutable access to the inner value, but it will panic at runtime if already borrowed mutably or immutably elsewhere.")]
pub fn basic_shared_mutability() {
    let shared_number = Rc::new(RefCell::new(0));

    let a = Rc::clone(&shared_number);
    let b = Rc::clone(&shared_number);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 2;

    println!("Final value: {}", shared_number.borrow());
}

#[description("Create a vector of Rc<RefCell<i32>> counters. Mutate several elements through clones and print the final result to verify shared mutability.")]
#[tip("You can store Rc<RefCell<T>> in collections to allow multiple owners to mutate shared values individually or together.")]
pub fn vector_of_shared_counters() {

    let counter = Rc::new(RefCell::new(0));
    let counters = vec![Rc::clone(&counter), Rc::clone(&counter), Rc::clone(&counter)];

    for c in &counters {
        *c.borrow_mut() += 1;
    }

    println!("Shared counter value: {}", counter.borrow());
}

#[description("Define a struct with an Rc<RefCell<T>> field. Create multiple instances sharing the same state, then mutate it from one and observe the effect on all.")]
#[tip("Structs can hold Rc<RefCell<T>> fields, enabling shared, mutable state in object-oriented design patterns.")]
pub fn shared_owner_struct() {

    struct Player {
        score: Rc<RefCell<u32>>,
    }

    let shared_score = Rc::new(RefCell::new(10));

    let player1 = Player {
        score: Rc::clone(&shared_score),
    };
    let player2 = Player {
        score: Rc::clone(&shared_score),
    };

    *player1.score.borrow_mut() += 15;
    println!("Player2 sees score: {}", player2.score.borrow());
}

#[description("Implement a basic linked list where each node contains an Rc<RefCell<Node>> pointing to the next. Update one of the nodes through shared ownership.")]
#[tip("Avoid circular references in lists made of Rc by breaking the cycle or using Weak<T> where appropriate.")]
pub fn linked_list_mutation() {

    type Link = Option<Rc<RefCell<Node>>>;

    struct Node {
        value: i32,
        next: Link,
    }

    let node3 = Rc::new(RefCell::new(Node { value: 3, next: None }));
    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(Rc::clone(&node3)),
    }));
    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        next: Some(Rc::clone(&node2)),
    }));

    node2.borrow_mut().value += 10;

    if let Some(ref n) = node1.borrow().next {
        println!("Second node value: {}", n.borrow().value);
    }
}



#[description("Implement a simple linked list structure using Rc<RefCell> to maintain shared, mutable nodes.")]
#[tip("With Rc<RefCell>, we can create list nodes where each node can be accessed and modified from multiple places.")]
#[tip("Remember to handle borrowing carefully to avoid runtime panics from multiple mutable borrows.")]
pub fn linked_list_example() {
    // Define a node structure for our linked list
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<RefCell<Node>>>,
    }

    // Create the nodes for our list
    let node3 = Rc::new(RefCell::new(Node {
        value: 3,
        next: None,
    }));

    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(Rc::clone(&node3)),
    }));

    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        next: Some(Rc::clone(&node2)),
    }));

    // Print the original list
    println!("Original list values:");
    let mut current = Some(Rc::clone(&node1));
    while let Some(node) = current {
        println!("{}", node.borrow().value);
        current = node.borrow().next.as_ref().map(Rc::clone);
    }

    // Modify a node in the middle of the list
    node2.borrow_mut().value = 10;

    // Print the modified list
    println!("Modified list values:");
    let mut current = Some(Rc::clone(&node1));
    while let Some(node) = current {
        println!("{}", node.borrow().value);
        current = node.borrow().next.as_ref().map(Rc::clone);
    }
}

#[description("Store Rc<RefCell> values in a collection to maintain a group of shared, mutable values.")]
#[tip("Collections with Rc<RefCell> let you maintain a group of values that can be accessed and modified from multiple places.")]
#[tip("Each element can be individually borrowed and modified without affecting other elements.")]
pub fn collection_of_shared_values() {
    let values = vec![
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
        Rc::new(RefCell::new(3)),
    ];

    // Create references to the values
    let references: Vec<Rc<RefCell<i32>>> = values.iter().map(Rc::clone).collect();

    // Modify values through the original vector
    *values[0].borrow_mut() *= 10;
    *values[2].borrow_mut() *= 10;

    // Print values through references to show they reflect the changes
    println!("Values through references:");
    for r in &references {
        println!("{}", r.borrow());
    }

    // Modify values through references
    *references[1].borrow_mut() *= 10;

    // Print values through original vector to show they reflect the changes
    println!("Values through original vector:");
    for v in &values {
        println!("{}", v.borrow());
    }
}