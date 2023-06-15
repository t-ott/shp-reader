// Esri Shapefile Specification:
// https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

use std::io::{self, Cursor, Read};
use std::fs::File;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

static TEST_FILE: &str = "/home/tott/layers/numeric_poly_test_3857.shp";

fn main() -> io::Result<()> {
    let mut f = File::open(TEST_FILE)?;
    let mut buf = [0; 1];
    println!("buffer before: {:?}", buf);

    f.read_i32_into::<BigEndian>(&mut buf)?;

    println!("buffer after: {:?}", buf);
    Ok(())
}
