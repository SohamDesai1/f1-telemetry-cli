use dialoguer::{theme::ColorfulTheme, Input, Select};
use native_dialog::{Error, FileDialog};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::Write, path::PathBuf, process::Command};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "code")]
    Code {
        source: Vec<String>,
        metadata: Value,
        outputs: Option<Vec<Value>>,
        execution_count: Option<i32>,
    },

    #[serde(rename = "markdown")]
    Markdown {
        source: Vec<String>,
        metadata: Value,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notebook {
    pub cells: Vec<Cell>,
    pub metadata: Value,
    pub nbformat: i32,
    pub nbformat_minor: i32,
}

pub fn select_notebook() -> Option<PathBuf> {
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
            let file_path: Result<Option<PathBuf>, Error>;
            if wsl::is_wsl() {
                println!("WSL detected");
                let res = Command::new("kdialog")
                    .arg("--getexistingdirectory")
                    .output()
                    .unwrap();
                file_path = Ok(Some(PathBuf::from(
                    String::from_utf8_lossy(&res.stdout).trim(),
                )));
            } else {
                file_path = FileDialog::new()
                    .add_filter("Jupyter Notebook", &["ipynb"])
                    .show_open_single_file();
            }
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
            let output_dir: PathBuf;
            if wsl::is_wsl() {
                println!("WSL detected");
                let res = Command::new("kdialog")
                    .arg("--getexistingdirectory")
                    .output()
                    .unwrap();
                output_dir = PathBuf::from(String::from_utf8_lossy(&res.stdout).trim());
            } else {
                output_dir = FileDialog::new()
                    .show_open_single_dir()
                    .expect("Failed to open file dialog")
                    .expect("No directory selected");
            }
            // Step 2: Project name prompt
            let project_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the file name")
                .interact_text()
                .unwrap();

            let project_path = output_dir.join(&project_name);
            let file_path = project_path.with_extension("ipynb");

            let file = File::create(&file_path);
            match file {
                Ok(mut file) => {
                    let content = r#"{
                    "cells": [],
                    "metadata": {
                        "kernelspec": {
                            "display_name": ".venv",
                            "language": "python",
                            "name": "python3"
                        },
                        "language_info": {
                            "codemirror_mode": {
                                "name": "ipython",
                                "version": 3
                            },
                            "file_extension": ".py",
                            "mimetype": "text/x-python",
                            "name": "python",
                            "nbconvert_exporter": "python",
                            "pygments_lexer": "ipython3",
                            "version": "3.12.3"
                        }
                    },
                    "nbformat": 4,
                    "nbformat_minor": 2
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
