use serde_json::json;
use std::fs::{read_to_string, write};
use crate::notebook;

pub fn add_cell(file_path: &str, new_code: Vec<String>) {
    let content = read_to_string(file_path).expect("Unable to read file");
    let mut notebook: notebook::Notebook =
        serde_json::from_str(&content).expect("Error parsing notebook");

    let new_cell = notebook::Cell {
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
