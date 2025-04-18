use goal_macro::description;

pub fn stack_to_heap() {
    let x_from_heap = Box::new(42);
    dbg!(&x_from_heap);
    dbg!(*x_from_heap);
}

#[description("manipulate and modify a value stored inside a Box")]
pub fn increment_boxed_u8_value() {
    let mut x_from_heap = Box::new(42);

    dbg!(&x_from_heap);

    *x_from_heap += 1;

    dbg!(&x_from_heap);
}

#[description("manipulate and modify a value stored inside a Box<Box>")]
pub fn increment_double_boxed_u8_value(
    mut x_from_heap_from_another_heap: Box<Box<u8>>,
) -> Result<Box<Box<u8>>, String> {
    **x_from_heap_from_another_heap += 1;
    Ok(x_from_heap_from_another_heap)
}

#[description("understand ownership implications when passing a Box")]
pub fn consommer_box() {
    let x_from_heap = Box::new(42);
    println!("let x_from_heap = Box::new(42); // then pass x_from_heap to dbg!() macro");
    dbg!(x_from_heap);
    println!("x_from_heap ownership lost !")
}
