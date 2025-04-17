use log_macro::log;

pub fn basics() {
    println!("coucou !");
}

#[log("plop")]
fn simple_usage() -> Result<(), String> {
    let x = Box::new(42);
    println!("x = {}", x);

    Ok(())
}
