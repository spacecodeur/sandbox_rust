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
    
    goals!{
        interplay::rc_refcell::basics::basic_shared_mutability =>    "Create a value that can be mutated across shared Rc references",
        interplay::rc_refcell::basics::vector_of_shared_counters =>  "Create a vector of shared mutable counters using Rc<RefCell<i32>>",
        interplay::rc_refcell::basics::shared_owner_struct => "Combine Rc and RefCell inside a struct for shared state across instances",
        interplay::rc_refcell::basics::linked_list_mutation => "Build a simple singly linked list with shared mutable nodes",
        interplay::rc_refcell::basics::linked_list_example =>        "Build an another simple linked list with shared, mutable nodes using Rc<RefCell>",
        interplay::rc_refcell::basics::collection_of_shared_values =>"Store Rc<RefCell> values in a collection for shared mutability"
    }
    
    goals!{
        interplay::rc_refcell::advanceds::tree_structure_with_mutation => "Create a tree-like structure where nodes share mutable children",
        interplay::rc_refcell::advanceds::cyclic_graph =>       "Model a cyclic graph structure with Rc<RefCell>",
        interplay::rc_refcell::advanceds::observer_pattern =>   "Implement the Observer pattern using Rc<RefCell> for shared state"
    }
}
