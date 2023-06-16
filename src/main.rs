// Esri Shapefile Specification:
// https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

use std::io::{self, BufReader, Read};
use std::fs::File;
// use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

const TEST_FILE: &str = "/home/tott/layers/numeric_poly_test_3857.shp";

#[derive(Debug)]
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

impl ShapeType {
    fn from_i32(value: i32) -> ShapeType {
        match value {
            0 => ShapeType::NullShape,
            1 => ShapeType::Point,
            3 => ShapeType::PolyLine,
            5 => ShapeType::Polygon,
            8 => ShapeType::MultiPoint,
            11 => ShapeType::PointZ,
            13 => ShapeType::PolyLineZ,
            15 => ShapeType::PolygonZ,
            18 => ShapeType::MultiPointZ,
            21 => ShapeType::PointM,
            23 => ShapeType::PolyLineM,
            25 => ShapeType::PolygonM,
            28 => ShapeType::MultiPointM,
            31 => ShapeType::MultiPatch,
            _ => panic!("Unknown value for ShapeType: {}", value)
        }
    }
}

fn main() -> io::Result<()> {
    let f = File::open(TEST_FILE)?;
    let mut input = BufReader::new(f);

    let mut buf = [0; 4];

    let mut i: u8 = 0;

    // ***
    // Start big Endian header portion

    while i <= 5 {
        input.read_exact(&mut buf)?;
    
        let n = i32::from_be_bytes(buf);
        println!("{:?}", n);

        i += 1;
    }
    input.read_exact(&mut buf)?;
    let file_length = i32::from_be_bytes(buf);
    println!("File length: {}", file_length);
    i += 1;

    // End big Endian header portion
    // ***

    // ***
    // Start little endian header portion

    input.read_exact(&mut buf)?;
    let version = i32::from_le_bytes(buf);
    println!("Version: {}", version);
    i += 1;

    input.read_exact(&mut buf)?;
    let shape_type: ShapeType = ShapeType::from_i32(i32::from_le_bytes(buf));
    println!("Shape type: {:?}", shape_type);
    i += 1;

    println!("\nStart bounding box read:");
    let mut bbox: [f64; 8];
    let mut buf = [0; 8];
    loop {
        input.read_exact(&mut buf)?;
        let n = f64::from_le_bytes(buf);
        println!{"{:?}", n};

        i += 1;
        if i == 17 {
            break;
        }
    }

    Ok(())
}
