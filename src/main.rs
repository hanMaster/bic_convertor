use bic_creator::{convert, download_zip, unzip};
use bic_creator::app_errors::*;
use bic_creator::json_creator::gen_json;

#[tokio::main]
async fn main() -> Result<()> {
    // let zip_file = download_zip().await?;
    // println!("Zip downloaded: {}", zip_file);
    // let unzipped = unzip(zip_file)?;
    // println!("Unzipped: {}", unzipped);
    // convert(unzipped)?;
    gen_json();
    Ok(())
}
