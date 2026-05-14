use colored::*;

// Enable ANSI colors on Windows
pub fn init() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();
}

pub fn print_welcome() {
    init();

    let blue = Color::TrueColor { r: 60, g: 120, b: 216 };
    let gray = Color::TrueColor { r: 120, g: 120, b: 120 };
    let red = Color::TrueColor { r: 220, g: 40, b: 40 };

    let top    = format!("  {}{}{}",  "╔".color(blue), "══════════════════════════════════════════════╗".color(blue), "");
    let sep    = format!("  {}{}{}",  "╠".color(blue), "══════════════════════════════════════════════╣".color(blue), "");
    let bottom = format!("  {}{}{}",  "╚".color(blue), "══════════════════════════════════════════════╝".color(blue), "");
    let side_l = "║".color(blue);
    let side_r = "║".color(blue);

    println!();
    println!("{}", top);
    println!("  {}  {}{}                                    {}", side_l, "KAIRO".bold().white(), "DB".color(blue).bold(), side_r);
    println!("  {}  {}                          {}", side_l, "human-readable databases".color(gray), side_r);
    println!("{}", sep);
    println!("  {}                                                {}", side_l, side_r);
    println!("  {}  {}                                  {}", side_l, "COMMANDS".color(gray).bold(), side_r);
    println!("  {}                                                {}", side_l, side_r);
    println!("  {}   {} {}          {}", side_l, "init".white().bold(), "set up a new project".color(gray), side_r);
    println!("  {}   {} {}    {}", side_l, "create".white().bold(), "apply a .kairo schema".color(gray), side_r);
    println!("  {}   {} {}     {}", side_l, "query".white().bold(), "run a database query".color(gray), side_r);
    println!("  {}   {} {}      {}", side_l, "read".white().bold(), "inspect a database file".color(gray), side_r);
    println!("  {}   {} {}    {}", side_l, "export".white().bold(), "convert db to .kairo".color(gray), side_r);
    println!("  {}   {} {}    {}", side_l, "tables".white().bold(), "list all tables".color(gray), side_r);
    println!("  {}   {} {}    {}", side_l, "status".white().bold(), "show project info".color(gray), side_r);
    println!("  {}                                                {}", side_l, side_r);
    println!("{}", sep);
    println!("  {}  {}                  {}", side_l, "kairo.infiniware.bid".color(blue), side_r);
    println!("  {}  {} {}                       {}", side_l, "made by".color(gray), format!("{}{}","I".color(red).bold(), "NFINIWARE".white().bold()), side_r);
    println!("{}", bottom);
    println!();
}

pub fn print_success(msg: &str) {
    init();
    let blue = Color::TrueColor { r: 60, g: 120, b: 216 };
    println!("  {} {}", "OK".color(blue).bold(), msg);
}

pub fn print_dim(msg: &str) {
    init();
    let gray = Color::TrueColor { r: 120, g: 120, b: 120 };
    println!("  {}", msg.color(gray));
}

pub fn print_header(msg: &str) {
    init();
    let blue = Color::TrueColor { r: 60, g: 120, b: 216 };
    println!();
    println!("  {}", msg.color(blue).bold());
    println!("  {}", "─".repeat(msg.len()).color(blue));
}

pub fn print_row(label: &str, value: &str) {
    init();
    let gray = Color::TrueColor { r: 120, g: 120, b: 120 };
    println!("  {}  {}", label.white().bold(), value.color(gray));
}

#[allow(dead_code)]
pub fn print_error(msg: &str) {
    init();
    let red = Color::TrueColor { r: 220, g: 40, b: 40 };
    eprintln!("  {} {}", "ERROR".color(red).bold(), msg);
}
