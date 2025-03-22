pub mod styles;
pub mod help;
use clearscreen;
use colored::Colorize;

pub fn clean() {
    clearscreen::clear().unwrap();
}

pub fn print(text: &str) {
    println!("{}\n", text);
}
pub fn error(text: &str) {
    println!("{}", text.on_red().to_string());
}

pub fn info(text: &str) {
    println!("\n{}", text.white().on_bright_blue().bold().to_string());
}

pub fn success(text: &str) {
    println!("{} {}", "✔" .green().bold(), text.green().bold().to_string());
}

pub fn header(title: &str, subtitle: &str) {
    print!("\n{}\n{}\n\n", title.on_purple().bold(), subtitle);
}

pub fn warning(text: &str) {
    println!("{} {}", "⚠" .yellow().bold(), text.yellow().bold().to_string());
}