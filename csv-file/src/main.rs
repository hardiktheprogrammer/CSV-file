use csv;
use std::error::Error;
fn main() {
    if let Err(e) = read_from_file("./csv-file/maincsvforrust.csv") {}
}
