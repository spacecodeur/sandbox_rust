mod r#box;
use r#box::*;
use goal_macro::goal;

// use goal_macro::goal;

fn main() {
    // basics

    #[goal("understand the basic use of Box to allocate a value on the heap")]
    basics::stack_to_heap();

    basics::increment_boxed_u8_value();
    basics::consommer_box();

    let mut x_from_heap_from_another_heap = Box::new(Box::new(23));
    dbg!(&x_from_heap_from_another_heap);
    x_from_heap_from_another_heap =
        basics::increment_double_boxed_u8_value(x_from_heap_from_another_heap).unwrap();
    dbg!(&x_from_heap_from_another_heap);

    // advanced

    let circle1 = advanced::Circle { radius: 10 };
    let square1 = advanced::Square {
        width: 5,
        heigth: 7,
    };

    let figures: Vec<Box<dyn advanced::Figure>> = vec![Box::new(circle1), Box::new(square1)];
    advanced::show_perimeters(figures);
}
