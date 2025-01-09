use std::fs::File;
use std::io::{Read, Write};
use zip::write::FileOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // File to be zipped
    let file_to_zip = "example.txt";
    // Output zip file name
    let zip_file_name = "example.zip";

    // Open the file to be zipped
    let mut input_file = File::open(file_to_zip)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Create the zip file
    let zip_file = File::create(zip_file_name)?;
    let mut zip_writer = zip::ZipWriter::new(zip_file);

    // Set options for the file in the zip
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Write the file into the zip archive
    zip_writer.start_file(file_to_zip, options)?;
    zip_writer.write_all(&buffer)?;

    // Finish the zip archive
    zip_writer.finish()?;
    println!("Zipped {} into {}", file_to_zip, zip_file_name);

    Ok(())
}
