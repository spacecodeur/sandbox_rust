use goal_log::tip;

#[tip("Using Box allows you to store values on the heap, which is useful when dealing with large data or recursive types.")]
#[tip("Even if a Box is wrapped in many references (e.g., &&&&Box), it can still be dereferenced to access the value.")]
pub fn stack_to_heap() {
    let x_from_heap = Box::new(42);
    
    dbg!(&x_from_heap);
    dbg!(&&&&&x_from_heap);
    dbg!(*x_from_heap);
}

#[tip("Box<T> allows mutable access to the contained value by dereferencing with *.")]
#[tip("Box must be declared as mutable to change its inner value.")]
pub fn increment_boxed_u8_value() {
    let mut x_from_heap = Box::new(42);

    dbg!(&x_from_heap);

    *x_from_heap += 1;

    dbg!(&x_from_heap);
}

#[tip("Chain dereferences (**) allows to access deeply boxed values.")]
#[tip("Box<Box<T>> is rare in practice, but useful for understanding nested heap allocations.")]
pub fn increment_double_boxed_u8_value() {
    let mut x_from_heap_from_another_heap = Box::new(Box::new(23));

    dbg!(&x_from_heap_from_another_heap);

    **x_from_heap_from_another_heap += 1;

    dbg!(&x_from_heap_from_another_heap);
}

#[tip("Passing a Box to a function or macro without referencing it moves its ownership.")]
#[tip("After ownership is moved, the original variable can no longer be used.")]
pub fn consommer_box() {
    let x_from_heap = Box::new(42);
    
    println!("let x_from_heap = Box::new(42); // then pass x_from_heap to dbg!() macro");
    dbg!(x_from_heap);
    println!("x_from_heap ownership lost !")
}
