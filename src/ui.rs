use console::{Term, style};
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
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
pub fn sub_step(num: usize, total: usize, msg: &str) {
    println!(
        "  {} {} {}",
        style("└─").dim(),
        style(format!("{}/{}:", num, total)).dim(),
        style(msg).dim()
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

#[allow(dead_code)]
pub fn select_option(prompt: &str, options: &[&str]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(style(prompt).bold().to_string())
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn ask(prompt: &str, default: &str) -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(style(prompt).bold().to_string())
        .default(default.into())
        .interact_text()
        .unwrap_or_else(|_| default.to_string())
}

#[allow(dead_code)]
pub fn ask_confirm(prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(style(prompt).bold().to_string())
        .default(true) // Default choice is 'Yes' if they hit Enter
        .wait_for_newline(true)
        .interact()
        .unwrap_or(false)
}

#[allow(dead_code)]
pub fn select_multiple(prompt: &str, options: &[&str]) -> Vec<usize> {
    MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(style(prompt).bold().to_string())
        .items(options)
        .interact()
        .unwrap_or_default()
}

pub fn ask_number(prompt: &str, default: usize) -> usize {
    Input::<usize>::with_theme(&ColorfulTheme::default())
        .with_prompt(style(prompt).bold().to_string())
        .default(default)
        .validate_with(|input: &usize| -> Result<(), &str> {
            if *input > 0 {
                Ok(())
            } else {
                Err("Please enter a number greater than 0")
            }
        })
        .interact_text()
        .unwrap_or(default)
}
