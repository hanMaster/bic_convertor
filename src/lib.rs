pub mod app_errors;
pub mod json_creator;

use std::fs::File;
use std::io::{copy, Cursor, Read};
use chrono::{Datelike, Duration, Utc, Weekday};
use xmltojson::to_json;
pub use app_errors::*;

pub async fn download_zip() -> Result<String> {
    let mut date = Utc::now();

    // get file for friday during weekend
    if date.weekday() == Weekday::Sat {
        date -= Duration::days(1);
    } else if date.weekday() == Weekday::Sun {
        date -= Duration::days(2);
    }

    let file_name = format!(
        "{}{:02}{:02}ED01OSBR.zip",
        date.year(),
        date.month(),
        date.day()
    );
    let target = format!(
        "https://www.cbr.ru/vfs/mcirabis/BIKNew/{}",
        file_name
    );

    println!("Downloading file from: {}", target);

    let response = reqwest::get(target).await?;
    let payload = response.bytes().await?;
    println!("File size: {}", payload.len());
    let start = String::from_utf8_lossy(&payload[..17]).to_string();
    if payload.len() < 10000 && start.contains("<!DOCTYPE html>") {
        return Err(Error::from("Failed to download file"));
    }
    let mut content = Cursor::new(payload);
    let mut dest = File::create(file_name.clone())?;
    copy(&mut content, &mut dest)?;
    Ok(file_name)
}

pub fn unzip(file_name: String) -> Result<String> {
    let file = File::open(&file_name)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut file = archive.by_index(0)?;

    let out_path = file.enclosed_name().ok_or("Failed to get zipped file name")?;
    let filename = out_path.to_str().ok_or("Failed to get zipped file name")?.to_string();
    let mut outfile = File::create(out_path)?;
    copy(&mut file, &mut outfile)?;
    Ok(filename)
}

pub fn convert(file_name: String) -> Result<()> {
    let output_path = "data.json";
    let file = File::open(file_name).expect("File not found");
    let mut rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::WINDOWS_1251))
        .build(file);

    let mut content = String::new();
    rdr.read_to_string(&mut content).unwrap();
    let json = to_json(&content).expect("Failed to convert to json");

    // Save the JSON structure into the other file.
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&json).unwrap(),
    )
        .expect("failed to write result file");
    Ok(())
}