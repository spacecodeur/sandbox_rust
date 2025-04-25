use goal_log::{description, tip};
use std::f64::consts::PI;
use std::fmt::Debug;

#[description("This exercise demonstrates how to use Box<dyn Trait> to store heterogeneous types that implement a common trait. 
You will define a trait called Figure and implement it for multiple structs (Circle and Square). 
Then, you will store these structs as boxed trait objects in a vector and iterate through them to compute their perimeter.")]
#[tip("Box<dyn Trait> is often used to enable polymorphism when you want to store multiple different types that implement the same trait. 
It is especially useful when trait objects are required but size cannot be known at compile time.")]
#[tip("Using trait objects with Box comes at the cost of dynamic dispatch, which means method calls are resolved at runtime rather 
than compile time. However, it allows for great flexibility in structuring your code.")]
pub fn box_trait_objects() {
    let circle1 = Circle { radius: 10 };
    let square1 = Square {
        width: 5,
        heigth: 7,
    };
    let figures: Vec<Box<dyn Figure>> = vec![Box::new(circle1), Box::new(square1)];

    dbg!(&figures);
    for figure in figures.iter() {
        dbg!(figure.perimeter());
    }
}

pub trait Figure: Debug {
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
pub struct Square {
    pub width: u8,
    pub heigth: u8,
}

#[derive(Debug)]
pub struct Circle {
    pub radius: u8,
}

impl Figure for Circle {
    fn perimeter(&self) -> f64 {
        self.radius as f64 * 2.0 * PI
    }
}

impl Figure for Square {
    fn perimeter(&self) -> f64 {
        self.width as f64 + self.heigth as f64
    }
}
