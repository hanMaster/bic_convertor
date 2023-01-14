use bic_creator::app_errors::*;
use bic_creator::{convert, download_zip, unzip};

#[tokio::main]
async fn main() -> Result<()> {
    let zip_file = download_zip().await?;
    let unzipped = unzip(zip_file)?;
    // let unzipped = "20230113_ED807_full.xml".to_string();
    convert(unzipped)?;
    Ok(())
}
