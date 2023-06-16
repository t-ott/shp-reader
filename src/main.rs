// Esri Shapefile Specification:
// https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

use std::io::{self, BufReader, Read};
use std::fs::File;

const TEST_FILE: &str = "/home/tott/layers/numeric_poly_test_3857.shp";

#[derive(Debug, Default)]
enum ShapeType {
    #[default] NullShape,
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

#[derive(Debug, Default)]
struct BoundingBox {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    m_min: f64,
    m_max: f64
}

#[derive(Debug, Default)]
struct ShpHeader {
    file_code: i32,
    file_length: i32,
    version: i32,
    shape_type: ShapeType,
    bounding_box: BoundingBox
}

fn main() -> io::Result<()> {
    let f = File::open(TEST_FILE)?;
    let mut input = BufReader::new(f);

    let mut header: ShpHeader = ShpHeader::default();

    // ***
    // Start big Endian header portion
    // reading Integer (i32)
    
    let mut buf = [0; 4];
    for i in 0..7 {
        input.read_exact(&mut buf)?;
        let n = i32::from_be_bytes(buf);

        match i {
            0 => header.file_code = n,
            6 => header.file_length = n,
            _ => continue
        }
    }
    println!("{:?}", header);

    // End big Endian header portion
    // ***

    // ***
    // Start little endian header portion
    for i in 0..2 {
        input.read_exact(&mut buf)?;
        let n = i32::from_le_bytes(buf);

        match i {
            0 => header.version = n,
            1 => header.shape_type = ShapeType::from_i32(n),
            _ => break
        }
    }
    println!("{:?}", header);

    // switch to Double (f64)
    let mut buf = [0; 8];
    let mut bbox = BoundingBox::default();

    for i in 0..8 {
        input.read_exact(&mut buf)?;
        let n = f64::from_le_bytes(buf);

        match i {
            0 => bbox.x_min = n,
            1 => bbox.y_min = n,
            2 => bbox.x_max = n,
            3 => bbox.y_max = n,
            4 => bbox.z_min = n,
            5 => bbox.z_max = n,
            6 => bbox.m_min = n,
            7 => bbox.m_max = n,
            _ => panic!("Went too far parsing bounding box")
        }
    }
    header.bounding_box = bbox;

    println!("{:?}", header);

    Ok(())
}
