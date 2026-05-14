use colored::*;

pub fn print_welcome() {
    println!();
    println!("  {}", "kairo".bold());
    println!("  human-readable databases");
    println!();
    println!("  usage:");
    println!("    kairo init           set up a new project");
    println!("    kairo create <name>  apply a .kairo schema");
    println!("    kairo query <sql>    run a query");
    println!("    kairo read <file>    inspect a database file");
    println!("    kairo export <file>  convert a database to .kairo format");
    println!("    kairo tables         list all tables");
    println!("    kairo status         show project info");
    println!();
    println!("  {}", "https://kairo.infiniware.bid".dimmed());
    println!();
}

pub fn print_success(msg: &str) {
    println!("{} {}", "ok".green().bold(), msg);
}

pub fn print_dim(msg: &str) {
    println!("{}", msg.dimmed());
}

#[allow(dead_code)]
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), msg);
}
