// Esri Shapefile Specification:
// https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

use std::fs::File;
use std::io::{self, BufReader, Seek, SeekFrom};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

const TEST_FILE: &str = "/home/tott/layers/numeric_pt_test_3857.shp";

#[derive(Debug, Default)]
enum ShapeType {
    #[default]
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
    MultiPatch,
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
            _ => panic!("Unknown value for ShapeType: {}", value),
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
    m_max: f64,
}

struct BoundingBoxSimple {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
}

#[derive(Debug, Default)]
struct ShpHeader {
    file_code: i32,
    file_length: i32,
    version: i32,
    shape_type: ShapeType,
    bounding_box: BoundingBox,
}

#[derive(Default)]
struct Point {
    x: f64,
    y: f64,
}

struct PolyLine {
    bounding_box: BoundingBoxSimple,
    num_parts: i32,
    num_points: i32,
    parts: Vec<i32>,
    points: Vec<Point>,
}

struct Polygon {
    bounding_box: BoundingBoxSimple,
    num_parts: i32,
    num_points: i32,
    parts: Vec<i32>,
    points: Vec<Point>,
}

enum ShpRecordGeom {
    Point(Point),
    PolyLine(PolyLine),
    Polygon(Polygon),
}

#[derive(Default)]
struct ShpRecordContent {
    shape_type: ShapeType,
    shape_record_geom: Option<ShpRecordGeom>,
}

#[derive(Default)]
struct ShpRecord {
    record_number: i32,
    content_length: i32,
    record_content: ShpRecordContent,
}

fn main() -> io::Result<()> {
    let f = File::open(TEST_FILE)?;
    let reader = BufReader::new(f);

    let header = read_header(reader).expect("Failure reading .shp header");
    println!("{:?}", header);

    Ok(())
}

fn read_header(mut reader: BufReader<File>) -> Result<ShpHeader, std::io::Error> {
    // Start big endian header portion
    let file_code = reader.read_i32::<BigEndian>()?;
    // Skip over empty bytes
    reader.seek(SeekFrom::Start(24))?;
    let file_length = reader.read_i32::<BigEndian>()?;

    // Start little endian header portion
    let version = reader.read_i32::<LittleEndian>()?;
    let shape_type = ShapeType::from_i32(reader.read_i32::<LittleEndian>()?);
    // Read bbox coordinates (Double)
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
            m_max: m_max,
        },
    };

    return Ok(header);
}

fn read_record(mut reader: BufReader<File>) -> Result<ShpRecord, std::io::Error> {
    return Ok(ShpRecord::default());
}
