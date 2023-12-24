// build.rs

use std::process::Command;
use std::env;

fn main() {

    if let Some(action) = env::args().nth(1) {
        match action.as_str() {
            "compile" => {
                // Run the compile script
                run_script("scripts/compile.sh");
            }
            "install" => {
                // Run the install script
                run_script("scripts/install.sh");
            }
            _ => {
                eprintln!("Error: Unknown build action specified");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Specify the build action using a command-line argument (e.g., 'cargo build --compile')");
        std::process::exit(1);
    }
}

fn run_script(script_path: &str) {
    let status = Command::new("bash")
        .arg(script_path)
        .status()
        .expect("Failed to execute the script");

    if !status.success() {
        eprintln!("Error: The script exited with a non-zero status code");
        std::process::exit(1);
    }
}
