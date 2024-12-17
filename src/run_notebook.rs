use std::{path::PathBuf, process::{exit, Command}};

pub fn run_notebook(file_path: &str,python_dir:Option<PathBuf>) {
    if let Some(python_path) = python_dir {
        let output = Command::new(python_path.to_str().unwrap())
            .arg("-m")
            .arg("nbconvert")
            .arg("--to")
            .arg("notebook")
            .arg("--execute")
            .arg(file_path)
            .arg("--output")
            .arg(file_path)
            .output()
            .expect("Failed to execute Python command");

        if !output.status.success() {
            eprintln!(
                "Error executing notebook: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        } else {
            println!("Notebook executed successfully");
        }
    } else {
        eprintln!("No Python kernel selected.");
        exit(1);
    }
}