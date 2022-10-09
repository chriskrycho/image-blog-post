use std::io::BufReader;
use std::{borrow::Borrow, fs::File};

use exif::{self, Exif, Tag};

fn main() -> Result<(), exif::Error> {
    let path = std::path::PathBuf::from("/Users/chris/Desktop/10-08-inversion.jpg");
    let fd = File::open(&path)?;
    let mut reader = BufReader::new(&fd);
    let ForTable {
        make,
        model,
        lens,
        exposure,
        f_number,
        iso,
    } = exif::Reader::new()
        .read_from_container(&mut reader)?
        .borrow()
        .into();

    let table = textwrap::dedent(&format!(
        r#"
        <table>
        <tr><th scope="row">Camera</th><td>{make} {model}</td></tr>
        <tr><th scope="row">Lens</th><td>{lens}</td></tr>
        <tr><th scope="row">Settings</th><td>{f_number}, {exposure}, {iso}</td></tr>
        </table>
        "#,
    ));

    println!("{table}");
    Ok(())
}

struct ForTable {
    make: String,
    model: String,
    lens: String,
    exposure: String,
    f_number: String,
    iso: String,
}

impl From<&Exif> for ForTable {
    fn from(exif: &Exif) -> Self {
        let make = get_field(exif, Tag::Make)
            .replace("\"", "")
            .replace("SONY", "Sony");

        let model = match get_field(exif, Tag::Model).replace("\"", "").as_str() {
            "ILCE-7RM4" => "α7R IV",
            "ILCE-7M4" => "α7 IV",
            "ILCE-7C" => "α7C",
            s => s,
        }
        .into();

        let lens = get_field(exif, Tag::LensModel).replace("\"", "");
        let exposure = get_field(exif, Tag::ExposureTime);
        let f_number = get_field(exif, Tag::FNumber).replace("f", "𝑓");
        let iso = String::from("ISO ") + &get_field(exif, Tag::PhotographicSensitivity);

        ForTable {
            make,
            model,
            lens,
            exposure,
            f_number,
            iso,
        }
    }
}

fn get_field(exif: &Exif, tag: Tag) -> String {
    exif.get_field(tag, exif::In::PRIMARY)
        .expect(&format!("Missing field {tag}"))
        .display_value()
        .with_unit(exif)
        .to_string()
}
