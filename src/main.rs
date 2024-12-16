use dialoguer::{theme::ColorfulTheme, Select};
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::{exit, Command},
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

fn add_cell(file_path: &str, new_code: &str) {
    // Read the notebook file
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    let mut notebook: Notebook = serde_json::from_str(&content).expect("Error parsing notebook");

    // Create a new code cell
    let new_cell = Cell {
        cell_type: "code".to_string(),
        source: vec![new_code.to_string()],
        metadata: json!({}),
        outputs: Some(vec![]),
        execution_count: None,
    };

    // Add the new cell to the notebook
    notebook.cells.push(new_cell);

    // Write the updated notebook back to the file
    let updated_content =
        serde_json::to_string_pretty(&notebook).expect("Error serializing notebook");
    fs::write(file_path, updated_content).expect("Unable to write file");
}

fn run_notebook(file_path: &str) {
    println!("Please select the appropriate option:");
    let items = vec!["Select a python kernel", "make a virtual env"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option :")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    let python: PathBuf;
    match selection {
        0 => {
            python = FileDialog::new().show_open_single_file().unwrap().unwrap();
        }
        _ => todo!(),
    }

    let output = Command::new(python.to_str().unwrap())
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
}

fn main() {
    let file_path = FileDialog::new()
        .set_location(".")
        .add_filter(".ipynb", &["ipynb"])
        .show_open_single_file()
        .unwrap()
        .unwrap();

    add_cell(file_path.to_str().unwrap(), "print('This is demo')");
    run_notebook(file_path.to_str().unwrap());
}
