use error_chain::error_chain;
use std::path::Path;
use std::fs::File;
use std::fs::metadata;
use std::{io, fs};
use std::io::prelude::*;
use std::path::PathBuf;
use zip_extensions::*;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

async fn download_and_unzip(url:&str, target:&str)-> Result<()>{
    
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
        &PathBuf::from(target),
    );

    let count = fs::read_dir(target).unwrap().count();

    if count == 1 {
        for file in fs::read_dir(target).unwrap() {
            let path = file.as_ref().unwrap().path();
            let path1 = file.as_ref().unwrap().path();
            let path2 = file.as_ref().unwrap().path();

            if fs::metadata(file.as_ref().unwrap().path()).unwrap().is_dir() {
                copy_dir_all(file.as_ref().unwrap().path(), target);
                fs::remove_dir_all(file.as_ref().unwrap().path()).unwrap();
            }
        }
    }
    Ok(())
}



#[tokio::main]
async fn main() -> Result<()> {
    download_and_unzip("https://github.com/minetest/minetest/releases/download/5.6.0/minetest-5.6.0-win64.zip", "versions/5.6.0").await?;
    
    
    Ok(())
}