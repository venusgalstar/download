use error_chain::error_chain;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use zip_extensions::*;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

async fn download_and_unzip(string url, string target)-> Result<()>{
    
    //let target = "https://github.com/minetest/minetest/releases/download/5.6.0/minetest-5.6.0-win64.zip";
    let response = reqwest::get(url).await?;

    let path = Path::new("./download1.zip");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content =  response.bytes().await?;
    file.write_all(content.as_ref())?;

    let mut zip_file1 = File::open("./download1.zip")?;


    let mut zip = zip::ZipArchive::new(zip_file1).unwrap();

    zip_extensions::read::zip_extract(
        &PathBuf::from("./download1.zip"),
        &PathBuf::from("./target"),
    );
    Ok(());

}
#[tokio::main]
async fn main() -> Result<()> {
    
    Ok(())
}