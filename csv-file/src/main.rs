use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?; // read from stdin and convert to string

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
} // handle realtime errors
fn main() {
    if let Err(e) = read_from_file("/csv-file/Book1.csv") {
        // passed the file path to the error handler
        eprintln!("{}", e); // println! fineshed
    }
}
