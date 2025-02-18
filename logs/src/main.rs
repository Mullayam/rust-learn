use std::fs;
use std::path::Path;

fn main() {
    let text = match fs::read_to_string(Path::new("logs.txt")) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    match divide(2.0, 0.0) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    let error_logs = extract_log("ERROR", &text);
    for log in &error_logs {
        println!("{}", log);
    }
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn extract_log(keyword: &str, text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| line.starts_with(keyword))
        .collect()
}
