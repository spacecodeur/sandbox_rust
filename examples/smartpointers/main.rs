mod r#box;
mod rc;
mod refcell;
mod interplay;
use goal_log::goals;

fn main() {
    goals! {
        r#box::basics::stack_to_heap =>                    "understand the basic use of Box to allocate a value on the heap",
        r#box::basics::increment_boxed_u8_value =>         "manipulate and modify a value stored inside a Box",
        r#box::basics::consommer_box =>                    "understand ownership implications when passing a Box",
        r#box::basics::increment_double_boxed_u8_value =>  "manipulate and modify a value stored inside a Box<Box>",
    }

    goals! {
        r#box::advanceds::box_trait_objects => "use Box to store different types implementing the same trait in a collection"
    }

    goals! {
        rc::basics::basic_rc_usage =>       "create and clone an Rc to share ownership of data",
        rc::basics::reference_count =>      "check how Rc reference counting works as you clone and drop values",
        rc::basics::rc_in_structs =>        "store Rc values in a struct and share them across instances",
        rc::basics::rc_in_collections =>    "Store multiple Rc values in a vector and track shared data"
    }

    goals!{
        rc::advanceds::dag_with_rc => "Model a Directed Acyclic Graph (DAG) using Rc to demonstrate shared ownership"
    }

    goals!{
        refcell::basics::basic_refcell_usage =>                         "Understand the basic usage of RefCell for interior mutability",
        refcell::basics::refcell_exclusive_borrowing_runtime_check =>   "Learn the runtime checks of RefCell for borrow rules",
        refcell::basics::refcell_inside_struct =>                       "Use RefCell inside a struct to allow internal mutability on fields",
        refcell::basics::refcell_error_handling =>                      "Handle RefCell borrow errors gracefully instead of panicking"
    }

    goals!{
        refcell::advanceds::refcell_with_pattern => "Apply RefCell in a common mutation pattern: caching a computed value"
    }
}
