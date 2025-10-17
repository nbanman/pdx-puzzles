use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn get_input(year: u8, day: u8) -> io::Result<String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR").to_string();
    let path = manifest_dir + &format!("/../../inputs/advent/20{}/y{}d{:02}.txt", year, year, day);
    // Try to open local file first
    match File::open(&path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if contents.as_bytes().last().unwrap() == &b'\n' {
                contents.pop();
            }
            Ok(contents)
        }
        Err(err) => {
            println!("{:?}\n{}", err, path);
            // If local file doesn't exist, attempt to download
            download_input(year, day, &path)
        }
    }
}

fn download_input(year: u8, day: u8, path: &str) -> Result<String, io::Error> {
    // Ensure the directory exists
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let url = format!("https://adventofcode.com/20{}/day/{}/input", year, day);

    let session = std::env::var("ADVENT_SESSION").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "ADVENT_SESSION environment variable not set",
        )
    })?;
    let session = format!("session={}", session);

    // Propagate network and file errors
    let mut response = ureq::get(&url)
        .header("Cookie", &session)
        .header(
            "User-Agent",
            "github.com/nbanman/pdx-puzzles/tree/main/rust/advent/utilities/get_input.rs",
        )
        .call()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;


    let mut response = response.body_mut().as_reader();

    // Create file and copy contents
    let mut file = File::create(path)?;
    std::io::copy(&mut response, &mut file)?;

    // Read the newly downloaded file
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    Ok(contents)
}
