use crate::error::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use spinners::{Spinner, Spinners};
use terminal_size::{terminal_size, Width};
use textwrap::fill;

pub struct PromptDisplay {
    pub content: String,
    pub model: String,
}

impl PromptDisplay {
    pub fn new(content: String, model: String) -> Self {
        Self { content, model }
    }

    pub fn display(&self) {
        // Get terminal width or use a default
        let width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80
        };

        // Leave some margin and account for borders
        let available_width = width.saturating_sub(4);

        // Wrap text to terminal width
        let wrapped_text = fill(&self.content, available_width.saturating_sub(4));
        let lines: Vec<&str> = wrapped_text.lines().collect();

        // Print header
        let header = "\nImproved Prompt".blue().bold();
        println!("{}", header);

        // Top border with proper width
        let border_line = "─".repeat(available_width);
        let top_border = format!("\n┌{}┐", border_line);
        println!("{}", top_border.blue());

        // Content with borders
        for line in lines {
            let padding = " ".repeat(available_width.saturating_sub(line.len()));
            let line_content = format!("│ {}{} │", line, padding);
            println!("{}", line_content.blue());
        }

        // Bottom border
        let bottom_border = format!("└{}┘", border_line);
        println!("{}", bottom_border.blue());

        // Show model info
        let model_info = format!("\nModel used: {}", self.model);
        println!("{}", model_info.dimmed());
    }
}

pub struct LoadingSpinner {
    spinner: Spinner,
}

impl LoadingSpinner {
    pub fn new(message: &str) -> Self {
        Self {
            spinner: Spinner::new(Spinners::Dots12, message.into()),
        }
    }

    pub fn stop_with_message(&mut self, message: &str) {
        self.spinner.stop_with_message(message.to_string());
    }

    pub fn stop_with_symbol(&mut self, message: &str) {
        let symbol_message = format!("✓ {}", message).green();
        self.spinner.stop_with_symbol(&symbol_message.to_string());
    }
}

pub fn prompt_for_action(choices: &[&str]) -> Result<usize> {
    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(choices)
        .default(0)
        .interact()?)
}

pub fn get_input(prompt: &str) -> Result<String> {
    let theme = ColorfulTheme::default();
    Ok(Input::<String>::with_theme(&theme)
        .with_prompt(prompt)
        .interact()?)
}
