// Esri Shapefile Specification:
// https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

use std::io::{self, BufReader, Seek, SeekFrom};
use std::fs::File;

use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

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
    let mut reader = BufReader::new(f);

    // Start big Endian header portion
    let file_code = reader.read_i32::<BigEndian>()?;
    reader.seek(SeekFrom::Start(24))?;
    let file_length = reader.read_i32::<BigEndian>()?;

    // Start little endian header portion
    let version = reader.read_i32::<LittleEndian>()?;
    let shape_type = ShapeType::from_i32(reader.read_i32::<LittleEndian>()?);

    let x_min = reader.read_f64::<LittleEndian>()?;
    let y_min = reader.read_f64::<LittleEndian>()?;
    let x_max = reader.read_f64::<LittleEndian>()?;
    let y_max = reader.read_f64::<LittleEndian>()?;
    let z_min = reader.read_f64::<LittleEndian>()?;
    let z_max = reader.read_f64::<LittleEndian>()?;
    let m_min = reader.read_f64::<LittleEndian>()?;
    let m_max = reader.read_f64::<LittleEndian>()?;

    let header = ShpHeader {
        file_code: file_code,
        file_length: file_length,
        version: version,
        shape_type: shape_type,
        bounding_box: BoundingBox {
             x_min: x_min,
             y_min: y_min,
             x_max: x_max,
             y_max: y_max,
             z_min: z_min,
             z_max: z_max,
             m_min: m_min,
             m_max: m_max 
        }
    };

    println!("{:?}", header);

    Ok(())
}
