use csv;
use std::error::Error;

fn read_from_file(filename: &str) -> Result<(), Box<dyn Error>> {} // handle realtime errors
fn main() {
    if let Err(e) = read_from_file("./csv-file/maincsvforrust.csv") {
        println!("{}", e); // println! fineshed
    }
}
