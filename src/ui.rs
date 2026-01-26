use console::{style, Term};
use indicatif::ProgressStyle;

pub static LOGO: &str = r#"
    ___         _ 
   /   | _   __(_) ____ ____  ________
  / /| || | / / / / ___/ __ \/ ___/ _ \
 / ___ || |/ / / / /__/ /_/ / /  /  __/
/_/  |_||___/_/_/\___/\____/_/   \___/
"#;

pub fn print_logo() {
    // Clear screen for a fresh start
    let term = Term::stdout();
    let _ = term.clear_screen();

    println!("{}", style(LOGO).cyan().bold());
    println!(
        "  {} {}",
        style("System Version:").dim(),
        env!("CARGO_PKG_VERSION")
    );
    println!("  {}\n", style("─".repeat(50)).dim());
}

pub fn step(num: usize, total: usize, msg: &str) {
    println!(
        "{} {}",
        style(format!("[{}/{}]", num, total)).bold().dim(),
        style(msg).bold()
    );
}

pub fn main_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{spinner:.cyan} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-")
}

pub fn info_line(key: &str, value: &str) {
    println!(
        "  {} {}",
        style(format!("{}:", key)).dim(),
        style(value).cyan()
    );
}

pub fn spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.cyan} {msg}")
        .unwrap()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", " "])
}
