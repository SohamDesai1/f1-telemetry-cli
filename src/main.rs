pub mod add_cell;
pub mod create_env;
pub mod driver_analysis;
pub mod notebook;
pub mod run_notebook;

use crate::add_cell::add_cell;
use crate::run_notebook::run_notebook;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use driver_analysis::driver_analysis;
use notebook::select_notebook;

fn main() {
    if let Some(file_path) = select_notebook() {
        if let Some(file_path_str) = file_path.to_str() {
            let python_dir = create_env::create_venv(file_path_str);
            add_cell(
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

            let sessions = vec![
                ("Sprint Qualifying", "Sprint Qualifying", "SpQuali"),
                ("Sprint Race", "Sprint", "sprint"),
                ("Qualifying", "Qualifying", "quali"),
                ("Race", "R", "race"),
            ];
            let sesh: Vec<_> = sessions.iter().map(|(sesh, _, _)| sesh).collect();
            let select_sesh = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a driver you want to analyze:")
                .default(0)
                .items(&sesh)
                .interact()
                .unwrap();

            let (_, _, sesh_var_name) = sessions[select_sesh];
            add_cell(
                file_path_str,
                vec![
                    format!(
                        "{} = f1.get_session(2024, \"Qatar\", \"{}\")\n",
                        sesh_var_name, select_sesh
                    ),
                    format!("{}.load()", sesh_var_name),
                ],
            );
            add_cell(
                file_path_str,
                vec![format!("{}.session_info", sesh_var_name)],
            );
            add_cell(
                file_path_str,
                vec![format!("{}.results.loc[\n",sesh_var_name),
    "    :, [\"Abbreviation\", \"TeamName\", \"GridPosition\", \"Position\", \"Time\", \"Status\"]\n".to_string(),
    "]".to_string()],
            );
            add_cell(file_path_str, vec![format!("{}.laps", sesh_var_name)]);

            driver_analysis(file_path_str);
            
            run_notebook(file_path_str, python_dir);
        } else {
            println!("Failed to convert the file path to a valid string.");
        }
    } else {
        println!("No notebook selected or an error occurred.");
    }
}
