pub use declarative_macros::*;
pub use procedural_macros::*;

use colored::*;

pub fn log(fn_name: &str, message: &str) {
    println!(
        "\n[ {} : {} ] => {}",
        "GOAL".bold(),
        fn_name.bold().green(),
        message.bold().blue()
    );
}

pub fn log_detailed(message: &str) {
    println!("{} : {}", "description".bold(), message.blue());
}

pub fn log_tip(message: &str) {
    println!("{} {}", "[tip]".bold().green(), message.green());
}
