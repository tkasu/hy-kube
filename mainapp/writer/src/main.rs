extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::path::Path;
use writer::loop_write_timestamp_to_file;

fn main() {
    dotenv().ok();

    let polling_freq_s: u64 = env::var("MAINAPP_WRITER_UPDATE_FREQ")
        .unwrap_or("5".to_string())
        .parse()
        .unwrap();
    let input_path = match env::var("MAINAPP_FILE_PATH") {
        Err(_) => {
            let test_file_path = String::from("test_file_3.txt");
            println!("Warning using test file path: {}", test_file_path);
            test_file_path
        }
        Ok(path) => path,
    };
    let path = Path::new(input_path.as_str());
    loop_write_timestamp_to_file(path, polling_freq_s);
}
