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

#[derive(Debug)]
struct BoundingBoxSimple {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
}

#[derive(Debug)]
struct Header {
    file_code: i32,
    file_length: i32,
    version: i32,
    shape_type: ShapeType,
    bounding_box: BoundingBox,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct PolyLine {
    bounding_box: BoundingBoxSimple,
    num_parts: i32,
    num_points: i32,
    parts: Vec<i32>,
    points: Vec<Point>,
}

#[derive(Debug)]
struct Polygon {
    bounding_box: BoundingBoxSimple,
    num_parts: i32,
    num_points: i32,
    parts: Vec<i32>,
    points: Vec<Point>,
}

#[derive(Debug)]
enum RecordGeom {
    Point(Point),
    PolyLine(PolyLine),
    Polygon(Polygon),
}

#[derive(Debug, Default)]
struct RecordContent {
    shape_type: ShapeType,
    record_geom: Option<RecordGeom>,
}

#[derive(Debug, Default)]
struct Record {
    record_number: i32,
    content_length: i32,
    record_content: RecordContent,
}

fn main() -> io::Result<()> {
    let f = File::open(TEST_FILE)?;
    let mut reader = BufReader::new(f);

    let header = read_header(&mut reader).expect("Failed to read .shp header");
    println!("Header:\n{:?}\n", header);

    // TODO: loop specified number of times based on value in header.file_length?
    // Or maybe just while buffer can be filled, then break
    for _ in 0..3 {
        let record = read_record(&mut reader).expect("Failed to read .shp record");
        println!("Record:\n{:?}\n", record)
    }

    Ok(())
}

fn read_header(reader: &mut BufReader<File>) -> Result<Header, std::io::Error> {
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

    let header = Header {
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

    Ok(header)
}

fn read_record(reader: &mut BufReader<File>) -> Result<Record, std::io::Error> {
    let record_number = reader.read_i32::<BigEndian>()?;
    let content_length = reader.read_i32::<BigEndian>()?;
    let record_content = read_record_content(reader)?;

    Ok(Record{record_number, content_length, record_content})
}

fn read_record_content(reader: &mut BufReader<File>) -> Result<RecordContent, std::io::Error> {
    let shape_type = ShapeType::from_i32(reader.read_i32::<LittleEndian>()?);
    let record_geom: Option<RecordGeom> = match shape_type {
        ShapeType::NullShape => None,
        ShapeType::Point => Some(RecordGeom::Point(
            read_point_geom(reader).expect("Failure reading Point record geom")
        )),
        _ => panic!("Haven't written that yet!")
    };

    Ok(RecordContent{shape_type, record_geom})
}

fn read_point_geom(reader: &mut BufReader<File>) -> Result<Point, std::io::Error> {
    let x = reader.read_f64::<LittleEndian>()?;
    let y = reader.read_f64::<LittleEndian>()?;

    Ok(Point{x, y})
}