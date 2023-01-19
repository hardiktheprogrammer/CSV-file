use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
} // handle realtime errors
fn main() {
    if let Err(e) = read_from_file("./csv-file/maincsvforrust.csv") {
        eprintln!("{}", e); // println! fineshed
    }
}
