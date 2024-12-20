use crate::notebook;
use serde_json::json;
use std::fs::{read_to_string, write};

pub fn add_cell(file_path: &str, new_code: Vec<String>) {
    let content = read_to_string(file_path).expect("Unable to read file");
    let mut notebook: notebook::Notebook =
        serde_json::from_str(&content).expect("Error parsing notebook");

    let new_cell = notebook::Cell::Code {
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

pub fn add_markdown(file_path: &str, new_code: Vec<String>) {
    let content = read_to_string(file_path).expect("Unable to read file");
    let mut notebook: notebook::Notebook =
        serde_json::from_str(&content).expect("Error parsing notebook");

    let new_cell = notebook::Cell::Markdown {
        source: new_code,
        metadata: json!({}),
    };

    notebook.cells.push(new_cell);

    let updated_content =
        serde_json::to_string_pretty(&notebook).expect("Error serializing notebook");
    write(file_path, updated_content).expect("Unable to write file");
}
