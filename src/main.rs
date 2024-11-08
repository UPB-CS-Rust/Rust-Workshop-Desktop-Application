use lazy_static::lazy_static;
use regex::Regex;
use slint::include_modules;

include_modules!();

lazy_static! {
    static ref VALID_EXPRESSION: Regex = Regex::new(r"(\+|-|\*|\/)[0-9]+").unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_add_to_text_area(move |current_text, new_input| {
        let ui = ui_handle.unwrap();
        match new_input.as_str() {
            "C" => ui.set_textarea("".into()), 
            "=" => {
                let result = evaluate(&current_text);
                ui.set_textarea(result.into());
            } 
            _ => ui.set_textarea(format!("{}{}", current_text, new_input).into()), 
        }
    });

    ui.run()
}

fn evaluate(input: &str) -> String {
    if VALID_EXPRESSION.is_match(input) {
        if let Some(result) = compute(input) {
            return result.to_string();
        }
    }
    "Invalid Expression".to_string()
}

fn compute(input: &str) -> Option<f64> {
    let symbols = ["+", "-", "*", "/"];
    for symbol in symbols {
        let parts: Vec<&str> = input.split(symbol).collect();
        if parts.len() == 2 {
            let n1: f64 = parts[0].parse().unwrap_or(0.0);
            let n2: f64 = parts[1].parse().unwrap_or(0.0);
            return Some(match symbol {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "*" => n1 * n2,
                "/" => n1 / n2,
                _ => 0.0,
            });
        }
    }
    None
}
