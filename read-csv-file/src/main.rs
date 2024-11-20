use std::error::Error;

fn main() {
    if let Err(e) = read_from_file("foo.csv") {
        eprintln!("Application error: {}", e);
    }
}
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(path);
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
