use goal_log::{tip, description};
use std::cell::RefCell;
use std::rc::Rc;

#[description("Extend a cyclic graph structure using Rc<RefCell> with the ability to dynamically remove connections between nodes. Demonstrate adding and removing neighbors.")]
#[tip("Cyclic references with Rc can cause memory leaks because the reference count never drops to zero.")]
#[tip("Consider using Weak references to break cycles when appropriate in real applications.")]
pub fn cyclic_graph() {

    #[derive(Debug)]
    struct GraphNode {
        id: String,
        neighbors: RefCell<Vec<Rc<GraphNode>>>,
    }

    impl GraphNode {
        fn new(id: &str) -> Rc<GraphNode> {
            Rc::new(GraphNode {
                id: id.to_string(),
                neighbors: RefCell::new(Vec::new()),
            })
        }

        fn add_neighbor(&self, neighbor: &Rc<GraphNode>) {
            self.neighbors.borrow_mut().push(Rc::clone(neighbor));
        }

        fn remove_neighbor(&self, neighbor_id: &str) {
            // Borrow the neighbors list mutably so we can modify it in-place
            let mut neighbors = self.neighbors.borrow_mut();
        
            // Retain only the neighbors whose id does not match the given neighbor_id
            neighbors.retain(|neighbor| {
                // This closure returns true if the neighbor should be kept
                neighbor.id != neighbor_id
            });
        }
        
        fn get_neighbors_ids(&self) -> Vec<String> {
            self.neighbors
                .borrow()
                .iter()
                .map(|node| node.id.clone())
                .collect()
        }
    }

    // Create three nodes
    let node_a = GraphNode::new("A");
    let node_b = GraphNode::new("B");
    let node_c = GraphNode::new("C");

    // Create a cycle: A -> B -> C -> A
    node_a.add_neighbor(&node_b);
    node_b.add_neighbor(&node_c);
    node_c.add_neighbor(&node_a);

    // Print the graph structure
    println!("Node A connects to: {:?}", node_a.get_neighbors_ids());
    println!("Node B connects to: {:?}", node_b.get_neighbors_ids());
    println!("Node C connects to: {:?}", node_c.get_neighbors_ids());

    // Add another connection: A -> C
    node_a.add_neighbor(&node_c);
    println!("After adding C: Node A connects to: {:?}", node_a.get_neighbors_ids());

    // Remove connection from A to B
    node_a.remove_neighbor("B");
    println!("After removing B: Node A connects to: {:?}", node_a.get_neighbors_ids());
}

#[description("Model a tree node structure using Rc<RefCell<T>> where each node owns its children. Mutate a shared subtree and observe the update across references.")]
#[tip("Tree structures using Rc<RefCell<T>> work well if you don’t create parent pointers. To avoid memory leaks with cycles, consider using Weak<T> for back references.")]
pub fn tree_structure_with_mutation() {

    type NodeRef = Rc<RefCell<Node>>;

    struct Node {
        name: String,
        children: Vec<NodeRef>,
    }

    let child = Rc::new(RefCell::new(Node {
        name: String::from("Leaf"),
        children: vec![],
    }));

    let parent1 = Rc::new(RefCell::new(Node {
        name: String::from("Parent1"),
        children: vec![Rc::clone(&child)],
    }));

    let parent2 = Rc::new(RefCell::new(Node {
        name: String::from("Parent2"),
        children: vec![Rc::clone(&child)],
    }));

    child.borrow_mut().name = String::from("Updated Leaf");

    for parent in [&parent1, &parent2] {
        println!(
            "{}'s child: {}",
            parent.borrow().name,
            parent.borrow().children[0].borrow().name
        );
    }
}


#[description("Implement the Observer pattern using Rc<RefCell> to allow subjects to notify registered observers.")]
#[tip("This pattern demonstrates how Rc<RefCell> enables complex data relationships with shared mutable state.")]
#[tip("Each observer can be registered with multiple subjects and each subject can have multiple observers.")]
pub fn observer_pattern() {
    // Observer trait for objects that want to be notified of changes
    trait Observer {
        fn update(&self, message: &str);
    }

    // Subject that will notify observers
    struct Subject {
        observers: RefCell<Vec<Rc<dyn Observer>>>,
        state: RefCell<String>,
    }

    impl Subject {
        fn new() -> Self {
            Subject {
                observers: RefCell::new(Vec::new()),
                state: RefCell::new(String::new()),
            }
        }

        fn attach(&self, observer: Rc<dyn Observer>) {
            self.observers.borrow_mut().push(observer);
        }

        fn set_state(&self, state: &str) {
            *self.state.borrow_mut() = state.to_string();
            self.notify();
        }

        fn notify(&self) {
            let state = self.state.borrow();
            for observer in self.observers.borrow().iter() {
                observer.update(&state);
            }
        }
    }

    // Concrete observer implementation
    struct ConcreteObserver {
        id: String,
        last_message: RefCell<String>,
    }

    impl ConcreteObserver {
        fn new(id: &str) -> Rc<Self> {
            Rc::new(ConcreteObserver {
                id: id.to_string(),
                last_message: RefCell::new(String::new()),
            })
        }

        fn get_last_message(&self) -> String {
            self.last_message.borrow().clone()
        }
    }

    impl Observer for ConcreteObserver {
        fn update(&self, message: &str) {
            *self.last_message.borrow_mut() = message.to_string();
            println!("Observer {} received: {}", self.id, message);
        }
    }

    // Create a subject and observers
    let subject = Subject::new();
    let observer1 = ConcreteObserver::new("Observer1");
    let observer2 = ConcreteObserver::new("Observer2");

    // Register observers with the subject
    subject.attach(observer1.clone());
    subject.attach(observer2.clone());

    // Change subject state - observers will be notified
    subject.set_state("First state change");
    
    // Verify the last message received by each observer
    println!("Observer1 last message: {}", observer1.get_last_message());
    println!("Observer2 last message: {}", observer2.get_last_message());
    
    // Change subject state again
    subject.set_state("Second state change");
}
