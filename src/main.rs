use dialoguer::{theme::ColorfulTheme, Input, Select};
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{read_dir, read_to_string, write, File},
    io::Write,
    path::{Path, PathBuf},
    process::{exit, Command},
    thread,
    time::Duration,
};

#[derive(Debug, Serialize, Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
    metadata: Value,
    nbformat: i32,
    nbformat_minor: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cell {
    cell_type: String,
    source: Vec<String>,
    metadata: Value,
    outputs: Option<Vec<Value>>,
    execution_count: Option<i32>,
}
fn select_notebook() -> Option<PathBuf> {
    let items = vec![
        "Select a existing notebook",
        "create a new jupyter notebook",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option :")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();
    match selection {
        0 => {
            let file_path = FileDialog::new()
                .set_location(".")
                .add_filter("Jupyter Notebook", &["ipynb"])
                .show_open_single_file();

            match file_path {
                Ok(Some(path)) => {
                    // File selected successfully
                    println!("Selected file: {:?}", path);
                    Some(path)
                }
                Ok(None) => {
                    // User canceled the dialog
                    println!("File selection canceled.");
                    None
                }
                Err(e) => {
                    // An error occurred
                    println!("An error occurred while opening the file dialog: {}", e);
                    None
                }
            }
        }
        1 => {
            let output_dir = FileDialog::new()
                .show_open_single_dir()
                .expect("Failed to open file dialog")
                .expect("No directory selected");

            // Step 2: Project name prompt
            let project_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the project name")
                .interact_text()
                .unwrap();

            let project_path = output_dir.join(&project_name);
            let file_path = project_path.with_extension("ipynb");

            let file = File::create(&file_path);
            match file {
                Ok(mut file) => {
                    let content = r#"{
                    "cells": [],
                    "metadata": {},
                    "nbformat": 4,
                    "nbformat_minor": 5
                }"#;

                    if let Err(e) = file.write_all(content.as_bytes()) {
                        eprintln!("Failed to write to file: {}", e);
                        return None;
                    } else {
                        println!("File created successfully at: {}", file_path.display());
                        return Some(file_path);
                    }
                }
                Err(_) => {
                    println!("Error occured creating notebook");
                    None
                }
            }
        }
        _ => todo!(),
    }
}

fn add_cell(file_path: &str, new_code: Vec<String>) {
    let content = read_to_string(file_path).expect("Unable to read file");
    let mut notebook: Notebook = serde_json::from_str(&content).expect("Error parsing notebook");

    let new_cell = Cell {
        cell_type: "code".to_string(),
        source: new_code,
        metadata: json!({}),
        outputs: Some(vec![]),
        execution_count: None,
    };

    notebook.cells.push(new_cell);

    let updated_content =
        serde_json::to_string_pretty(&notebook).expect("Error serializing notebook");
    write(file_path, updated_content).expect("Unable to write file");
}

fn create_venv(file_path: &str) -> Option<PathBuf> {
    println!("Please select the appropriate option:");
    let items = vec!["Select a python kernel", "make a virtual env"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option :")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    let python: Option<PathBuf>;
    match selection {
        0 => {
            let python_dir = Some(FileDialog::new().show_open_single_dir().unwrap().unwrap());
            if let Some(dir) = python_dir {
                if dir.join("bin").is_dir() {
                    for entry in read_dir(dir.join("bin")).expect("Failed to read bin folder") {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.file_name().unwrap_or_default() == "python" {
                                println!("Found python executable: {}", path.display());
                                return Some(path);
                            }
                        }
                    }
                }
                println!("No python executable found in the bin folder.");
            } else {
                println!("No directory selected.");
            }

            None
        }
        1 => {
            let notebook_dir = Path::new(file_path).parent().unwrap();

            if notebook_dir.join(".venv").exists() {
                println!("Virtual environment already exists at {:?}", notebook_dir);
            } else {
                println!("Creating virtual environment...");

                let output = Command::new("python3")
                    .arg("-m")
                    .arg("venv")
                    .arg(notebook_dir.join(".venv"))
                    .output()
                    .expect("Failed to create virtual environment");

                if !output.status.success() {
                    eprintln!(
                        "Error creating virtual environment: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    exit(1);
                } else {
                    println!(
                        "Virtual environment created successfully at {}",
                        notebook_dir.display()
                    );
                }
            }
            let requirements_path = notebook_dir.join("requirements");
            let file_path = requirements_path.with_extension("txt");

            let file = File::create(&file_path);
            match file {
                Ok(mut file) => {
                    let content = "numpy\npandas\nmatplotlib\nseaborn\nfastf1\ning_theme_matplotlib\nmplcyberpunk";
                    if let Err(e) = file.write_all(content.as_bytes()) {
                        eprintln!("Failed to write to file: {}", e);
                    } else {
                        println!("requirements.txt file created successfully!",);
                    }
                }
                Err(_) => println!("Error occured creating notebook"),
            }

            let pip = notebook_dir.join(".venv").join("bin").join("pip");

            let install_thread = thread::spawn(move || {
                let install_output = Command::new(pip)
                    .arg("install")
                    .arg("-r")
                    .arg("requirements.txt")
                    .output()
                    .expect("Failed to install packages in the virtual environment");

                if !install_output.status.success() {
                    eprintln!(
                        "Error installing packages: {}",
                        String::from_utf8_lossy(&install_output.stderr)
                    );
                    exit(1);
                } else {
                    println!("Packages installed successfully.");
                }
            });

            let mut dots = 0;
            print!("Installing packages");
            loop {
                print!(".");
                thread::sleep(Duration::from_millis(500));
                dots += 1;
                if dots > 3 {
                    dots = 0;
                    print!("\rInstalling packages");
                }
                if install_thread.is_finished() {
                    break;
                }
                std::io::stdout().flush().unwrap();
            }
            python = Some(notebook_dir.join(".venv").join("bin").join("python"));
            return python;
        }
        _ => {
            eprintln!("Invalid selection.");
            exit(1);
        }
    }
}

fn run_notebook(file_path: &str) {
    let python = create_venv(file_path);
    if let Some(python_path) = python {
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

fn main() {
    if let Some(file_path) = select_notebook() {
        if let Some(file_path_str) = file_path.to_str() {
            add_cell(file_path_str, vec!["import fastf1 as f1".to_string()]);
            add_cell(
                file_path_str,
                vec![
                    "SpQuali = f1.get_session(2024, \"US\", \"Sprint Qualifying\")".to_string(),
                    "SpQuali.load()".to_string(),
                ],
            );
            add_cell(file_path_str, vec!["SpQuali.session_info".to_string()]);
            run_notebook(file_path_str);
        } else {
            println!("Failed to convert the file path to a valid string.");
        }
    } else {
        println!("No notebook selected or an error occurred.");
    }
}
