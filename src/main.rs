use bic_creator::{convert, download_zip, unzip};
use bic_creator::app_errors::*;

#[tokio::main]
async fn main() -> Result<()> {
    // let zip_file = download_zip().await?;
    // println!("Zip downloaded: {}", zip_file);
    // let unzipped = unzip(zip_file)?;
    // println!("Unzipped: {}", unzipped);
    let unzipped = "20230113_ED807_full.xml".to_string();
    convert(unzipped)?;
    Ok(())
}
