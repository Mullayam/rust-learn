use std::io::Error;
use std::fs;
use std::path::Path;
fn main() {
    let text: String = fs::read_to_string(Path::new("input.txt")).unwrap();

    let nums: Vec<i32> = text
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let result = divide(nums[0], nums[1]);
    match result {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }

    print!("{}", text);
}
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
