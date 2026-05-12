use colored::*;

pub fn print_welcome() {
    println!("{}", "KairoDB".bold());
    println!();
    println!("Human-readable databases.");
    println!("Minimal. Fast. Local-first.");
    println!();
    println!("{}", "https://kairo.infiniware.bid".blue());
    println!();
    println!("{}", "MADE BY INFINIWARE".dimmed());
}

pub fn print_success(msg: &str) {
    println!("{}", msg.green());
}

#[allow(dead_code)]
pub fn print_error(msg: &str) {
    eprintln!("{}: {}", "error".red().bold(), msg);
}
