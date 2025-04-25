use goal_log::{tip, description};
use std::rc::Rc;

/* Structure generated here 
 *
 *          ┌────────┐      ┌───────┐                                                             
 * root────►│"root"│ │─────►│ │ │...│                                                             
 *      RC  └────────┘ VEC  └┬─┬────┘                                                             
 *                           │ │        ┌───────────┐      ┌─────┐                
 *                           │ └───────►│"ModuleA"│ │─────►│ │...│    shared_dependency                            
 *                           │     RC   └───────────┘ VEC  └┬────┘    │                            
 *                           │                              │         │ RC                           
 *                           │                              │ RC      │                            
 *                           │                              │         ▼
 *                           │                              └───────►┌────────────────────┐      ┌┐
 *                           │                              ┌───────►│"SharedDependency"│ ┼─────►││
 *                           │                              │        └────────────────────┘ VEC  └┘
 *                           │                              │ RC
 *                           │                              │                                 
 *                           │          ┌───────────┐      ┌─────┐                                 
 *                           └─────────►│"ModuleB"│ │─────►│ │...│                                 
 *                                 RC   └───────────┘ VEC  └─────┘                                 
 *                                                       
 */

#[description("Build a simple directed acyclic graph (DAG) using Rc for shared nodes. Demonstrate shared ownership by multiple parents and traverse the graph.")]
#[tip("Rc is useful in graph structures like DAGs, where multiple nodes might point to a shared child without creating cycles.")]
#[tip("This pattern is useful for modeling dependency graphs, where one task can be a prerequisite for several others.")]
pub fn dag_with_rc() {

    #[derive(Debug)]
    struct GraphNode {
        name: String,
        children: Vec<Rc<GraphNode>>,
    }

    fn print_graph(node: &Rc<GraphNode>, indent: usize) {
        println!("{}- {}", "  ".repeat(indent), node.name);
        for child in &node.children {
            print_graph(child, indent + 1);
        }
    }

    // Shared leaf node (like a library used by multiple programs)
    let shared_dependency = Rc::new(GraphNode {
        name: "SharedDependency".to_string(),
        children: vec![],
    });

    let module_a = Rc::new(GraphNode {
        name: "ModuleA".to_string(),
        children: vec![Rc::clone(&shared_dependency)],
    });

    let module_b = Rc::new(GraphNode {
        name: "ModuleB".to_string(),
        children: vec![Rc::clone(&shared_dependency)],
    });

    let root = Rc::new(GraphNode {
        name: "MainApp".to_string(),
        children: vec![module_a, module_b],
    });

    println!("Graph structure:");
    print_graph(&root, 0);

    println!(
        "SharedDependency strong count (used in both modules): {}",
        Rc::strong_count(&shared_dependency)
    );
}
