use goal_macro::description;
use std::f64::consts::PI;
use std::fmt::Debug;

#[description("store different trait implementations using vect<Box<dyn Trait>>")]
pub fn show_perimeters(figures: Vec<Box<dyn Figure>>) {
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
