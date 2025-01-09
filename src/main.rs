use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use zip::{ZipWriter, CompressionMethod, write::FileOptions};
use std::path::Path;
use std::io::BufReader;
use std::fs::OpenOptions;
use zip::read::ZipArchive;
use std::env;

fn zip_file(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_file = File::open(file_name)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let zip_file_name = format!("{}.zip", file_name);
    let zip_file = File::create(&zip_file_name)?;
    let mut zip_writer = ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip_writer.start_file(file_name, options)?;
    zip_writer.write_all(&buffer)?;

    zip_writer.finish()?;
    println!("Zipped {} into {}", file_name, zip_file_name);
    Ok(())
}

fn unzip_file(zip_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(zip_file_name)?;
    let mut zip_archive = ZipArchive::new(zip_file)?;

    let destination_folder = zip_file_name.trim_end_matches(".zip");
    create_dir_all(destination_folder)?;

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i)?;
        let file_path = Path::new(destination_folder).join(file.name());
        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)?;

        std::io::copy(&mut file, &mut output_file)?;
        println!("Unzipped: {}", file.name());
    }

    println!("Unzipped {} successfully.", zip_file_name);
    Ok(())
}

fn main() {
    println!("Choose an option:");
    println!("1. Zip a file");
    println!("2. Unzip a file");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    println!("Enter the file name:");
    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim();

    match choice {
        "1" => {
            if let Err(e) = zip_file(file_name) {
                println!("Error zipping file: {}", e);
            }
        }
        "2" => {
            if let Err(e) = unzip_file(file_name) {
                println!("Error unzipping file: {}", e);
            }
        }
        _ => println!("Invalid option."),
    }
}
