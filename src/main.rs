// Esri Shapefile Specification:
// https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

use std::io::{self, Read};
use std::fs::File;

static TEST_FILE: &str = "/home/tott/layers/numeric_poly_test_3857.shp";

fn main() -> io::Result<()> {
    let mut f = File::open(TEST_FILE)?;
    let mut buffer = [0; 92];
    println!("buffer: {:?}", buffer);

    f.read(&mut buffer)?;

    println!("buffer: {:?}", buffer);
    Ok(())
}
