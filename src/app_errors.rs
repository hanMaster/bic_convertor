use error_chain::error_chain;

error_chain! {
     foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Zip(zip::result::ZipError);
        Json(serde_json::Error);
     }
}