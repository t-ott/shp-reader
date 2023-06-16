// Esri Shapefile Specification:
// https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

use std::io::{self, BufReader, Read};
use std::fs::File;
// use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

const TEST_FILE: &str = "/home/tott/layers/numeric_poly_test_3857.shp";

enum ShapeType {
    NullShape,
    Point,
    PolyLine,
    Polygon,
    MultiPoint,
    PointZ,
    PolyLineZ,
    PolygonZ,
    MultiPointZ,
    PointM,
    PolyLineM,
    PolygonM,
    MultiPointM,
    MultiPatch
}

fn main() -> io::Result<()> {
    let f = File::open(TEST_FILE)?;
    let mut input = BufReader::new(f);

    let mut i: u8 = 0;

    // Big Endian header portion
    while i <= 6 {
        let mut buf = [0; 4];
        input.read_exact(&mut buf)?;
    
        let n = i32::from_be_bytes(buf);
        println!("{:?}", n);

        i += 1;
    }

    // Little Endian header portion
    loop {
        let mut buf = [0; 4];
        input.read_exact(&mut buf)?;

        let n = i32::from_le_bytes(buf);
        println!{"{:?}", n};

        i += 1;
        if i == 16 {
            break;
        }
    }

    Ok(())
}
