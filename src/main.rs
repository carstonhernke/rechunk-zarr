use std::env;
use std::fs;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};

// create a struct to handle the .zarray metadata file
#[derive(Deserialize, Serialize)]
struct ZArray {
    chunks: [i32;1],
    compressor: Option<String>,
    dtype: Option<String>,
    fill_value: i32,
    filters: Option<String>,
    order: Option<String>,
    shape: [i32;1],
    zarr_format: i32
}

fn main() {
    // get the filename from the first argument
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // get all the paths within the specified filename
    let paths = fs::read_dir(file_path).unwrap();

    // we want to get the number of 'extra' chunks to rechunk into the 0 chunk
    let num_chunks:i32 = (paths.count() as i32) - 1;

    {
        // open file 0 (assume this will always exist)
        let mut chunk0 = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}{}", file_path, "/0"))
        .unwrap();
    
        // because of the zarr chunk naming convention (continuous integers), we can iterate through the indexes
        // for each extra chunk, append the byte contents to the 0th chunk and delete the file
        // assumption: all values are 32-bit signed integers
        for i in 1i32..num_chunks{
            let mut f = File::open(format!("{}{}{}",&file_path, "/", i)).expect("error opening file");
            let mut buffer = [0; 4];
            f.read(&mut buffer).expect("buffer overflow");

            chunk0.write(&buffer).expect("unable to write");

            // remove file because we don't need it anymore
            fs::remove_file(format!("{}{}{}",&file_path, "/", i)).unwrap();
        }
        }
    
    // finally, we need to modify the .zarray metadata file with the new chunk information
    let zarray_string: String = fs::read_to_string(format!("{}{}",&file_path, "/.zarray")).expect("Unable to read file");
    let mut zarray: ZArray = serde_json::from_str(&zarray_string).expect("unable to parse JSON");
    
    // set the chunk attribute to the number of files we processed
    zarray.chunks = [num_chunks; 1];

    // serialize and write the new .zarray file to disk
    let new_zarray: String = serde_json::to_string_pretty(&zarray).unwrap();
    fs::write(format!("{}{}",&file_path, "/.zarray"),new_zarray).expect("unable to serialize");
}