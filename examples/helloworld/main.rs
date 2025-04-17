use log_macro::log;

#[log("une fonction pour tester le bon fonctionnement du repo")]
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
