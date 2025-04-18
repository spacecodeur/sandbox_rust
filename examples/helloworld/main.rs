// use goal_macro::goal;

// #[goal("Do you like helloworld ? here a helloworld !")]
fn main() -> Result<(), String> {
    let message = get_hello_world();
    println!("{}", message);
    Ok(())
}

fn get_hello_world() -> String {
    "hello world !".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hello_world_test() {
        assert_eq!(get_hello_world(), "hello world !".to_string());
    }
}
