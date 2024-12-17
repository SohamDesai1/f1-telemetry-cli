pub mod add_cell;
pub mod create_env;
pub mod notebook;
pub mod run_notebook;

use crate::add_cell::add_cell;
use crate::run_notebook::run_notebook;
use notebook::select_notebook;

fn main() {
    if let Some(file_path) = select_notebook() {
        if let Some(file_path_str) = file_path.to_str() {
            let python_dir = create_env::create_venv(file_path_str);
            add_cell::add_cell(
                file_path_str,
                vec![
                    "import numpy as np\n".to_string(),
                    "import pandas as pd\n".to_string(),
                    "import matplotlib.pyplot as plt\n".to_string(),
                    "from matplotlib.collections import LineCollection\n".to_string(),
                    "import fastf1 as f1\n".to_string(),
                    "from ing_theme_matplotlib import mpl_style\n".to_string(),
                    "import mplcyberpunk\n".to_string(),
                    "from fastf1 import plotting".to_string(),
                ],
            );
            add_cell(
                file_path_str,
                vec![
                    "SpQuali = f1.get_session(2024, \"Qatar\", \"Sprint Qualifying\")\n"
                        .to_string(),
                    "SpQuali.load()".to_string(),
                ],
            );
            add_cell(file_path_str, vec!["SpQuali.session_info".to_string()]);
            add_cell(
                file_path_str,
                vec!["SpQuali.results.loc[\n".to_string(),
    "    :, [\"Abbreviation\", \"TeamName\", \"GridPosition\", \"Position\", \"Time\", \"Status\"]\n".to_string(),
    "]".to_string()],
            );
            add_cell(file_path_str, vec!["SpQuali.laps".to_string()]);

            run_notebook(file_path_str, python_dir);
        } else {
            println!("Failed to convert the file path to a valid string.");
        }
    } else {
        println!("No notebook selected or an error occurred.");
    }
}
