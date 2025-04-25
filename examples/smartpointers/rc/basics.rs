use goal_log::{tip, description};
use std::rc::Rc;

#[description("This exercise demonstrates the basic usage of Rc<T>. Create an Rc that holds a String, 
then clone it and show that both Rc instances point to the same data. Print the value using both clones.")]
#[tip("Rc<T> enables multiple owners of the same data in a single-threaded context.")]
#[tip("Cloning an Rc increases the reference count, and the data is dropped only when the count reaches zero.")]
pub fn basic_rc_usage() {

    let data = Rc::new(String::from("Shared ownership"));
    let shared1 = Rc::clone(&data);
    let shared2 = Rc::clone(&data);

    println!("Original: {}", data);
    println!("Shared1: {}", shared1);
    println!("Shared2: {}", shared2);
}

#[description("Create an Rc, clone it twice, and print the reference count after each operation. 
Then, drop one clone and print the count again. This exercise shows how the reference count changes over time.")]
#[tip("You can use Rc::strong_count to check how many strong references exist. Rc also supports weak references to avoid cyclic references (see Rc + Weak later).")]
#[tip("Rust allows automatic dereferencing through 'deref coercion'. Even if you pass &&&&Rc<T>, Rust will implicitly unwrap layers of references to match what the function expects. For example, Rc::clone needs a &Rc<T>, and Rust will traverse any number of & to get there.")]
pub fn reference_count() {

    let data = Rc::new(String::from("Counting references"));
    println!("Initial count: {}", Rc::strong_count(&data));

    let clone1 = Rc::clone(&&&&&&&&&&&&&&&&&&&data);
    println!("After clone1: {}", Rc::strong_count(&data));

    let _clone2 = Rc::clone(&data);
    println!("After clone2: {}", Rc::strong_count(&data));

    drop(clone1);
    println!("After dropping clone1: {}", Rc::strong_count(&data));
}

#[description("Create a struct that holds an Rc<String>. Instantiate multiple instances of the struct sharing the same Rc. 
Print values from each instance to confirm shared ownership.")]
#[tip("Using Rc in structs is common for managing shared configuration or shared model data in applications, especially in UI or graph structures.")]
pub fn rc_in_structs() {

    struct SharedItem {
        name: Rc<String>,
    }

    let shared_name = Rc::new(String::from("Gadget"));

    println!("Initial count: {}", Rc::strong_count(&shared_name));

    let item1 = SharedItem { name: Rc::clone(&shared_name) };
    let item2 = SharedItem { name: Rc::clone(&shared_name) };

    println!("Item1: {}", item1.name);
    println!("Item2: {}", item2.name);
    println!("After creating two items : {}", Rc::strong_count(&shared_name));

    drop(item2);

    println!("After dropping Item2: {}", Rc::strong_count(&shared_name));

}


#[description("Put several Rc<String> instances into a vector, some of them sharing the same Rc. Demonstrate shared ownership by printing and counting references.")]
#[tip("You can use Rc in collections to model shared data. When using nested data structures (like trees), this becomes even more powerful.")]
pub fn rc_in_collections() {

    let name = Rc::new(String::from("Widget"));
    let vec = vec![
        Rc::clone(&name),
        Rc::new(String::from("Gizmo")),
        Rc::clone(&name),
    ];

    for (i, item) in vec.iter().enumerate() {
        println!("Item {}: {}", i, item);
    }

    println!("Reference count for 'Widget': {}", Rc::strong_count(&name));
}
