use chrono::prelude::Utc;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn append_to_file(s: String, path: &Path) {
    let file_opts = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path);

    let mut file = match file_opts {
        Err(e) => panic!("Couldn't open {}: {}", path.display(), e),
        Ok(file) => file,
    };

    match file.write(s.as_bytes()) {
        Err(e) => panic!("Couldn't write to {}: {}", path.display(), e),
        Ok(_) => (),
    };
}

fn loop_write_timestamp_to_file(path: &Path) {
    loop {
        let now = Utc::now();
        let now_formatted = format!("{:?}\n", now);

        append_to_file(now_formatted, path);
        sleep(Duration::from_secs(5));
    }
}

fn main() {
    let input_path = match env::var("MAINAPP_FILE_PATH") {
        Err(_) => {
            let test_file_path = String::from("test_file_3.txt");
            println!("Warning using test file path: {}", test_file_path);
            test_file_path
        }
        Ok(path) => path,
    };
    let path = Path::new(input_path.as_str());
    loop_write_timestamp_to_file(path);
}
