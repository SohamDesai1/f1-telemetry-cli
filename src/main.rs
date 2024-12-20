pub mod add_cell;
pub mod create_env;
pub mod driver_analysis;
pub mod notebook;
pub mod plot;
pub mod run_notebook;

use add_cell::{add_cell, add_markdown};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use driver_analysis::driver_analysis;
use notebook::select_notebook;
use plot::generate_plot;
use run_notebook::run_notebook;

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

            let time_fn = vec![
                "def convert_to_normal(time):\n".to_string(),
                "    if pd.isna(time):\n".to_string(),
                "        return time\n".to_string(),
                "\n".to_string(),
                "    time_str = str(time)\n".to_string(),
                "    parts = time_str.split()\n".to_string(),
                "\n".to_string(),
                "    days = int(parts[0])\n".to_string(),
                "    minutes, seconds_with_microseconds = parts[2].split(\":\")[1:]\n".to_string(),
                "    seconds, microseconds = seconds_with_microseconds.split(\".\")\n".to_string(),
                "\n".to_string(),
                "    minutes = int(minutes)\n".to_string(),
                "    seconds = int(seconds)\n".to_string(),
                "    microseconds = int(microseconds)\n".to_string(),
                "\n".to_string(),
                "    total_microseconds = (\n".to_string(),
                "        days * 86400000000 + minutes * 60000000 + seconds * 1000000 + microseconds\n".to_string(),
                "    )\n".to_string(),
                "\n".to_string(),
                "    minutes, remaining_microseconds = divmod(total_microseconds, 60000000)\n".to_string(),
                "    seconds, microseconds = divmod(remaining_microseconds, 1000000)\n".to_string(),
                "\n".to_string(),
                "    normal_time = f\"{minutes:02d}:{seconds:02d}.{microseconds:06d}\"\n".to_string(),
                "    return normal_time".to_string()
            ];
            add_cell(file_path_str, time_fn);

            let sessions = vec![
                ("Sprint Qualifying", "Sprint Qualifying", "SpQuali"),
                ("Sprint Race", "Sprint", "sprint"),
                ("Qualifying", "Qualifying", "quali"),
                ("Race", "R", "race"),
            ];

            loop {
                let mut sesh_names: Vec<_> = sessions.iter().map(|(sesh, _, _)| sesh).collect();
                sesh_names.push(&"Cancel");
                let select_sesh = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a session you want to analyze:")
                    .default(0)
                    .items(&sesh_names)
                    .interact()
                    .unwrap();

                if select_sesh == sesh_names.len() - 1 {
                    println!("Exiting session analysis.");
                    break;
                }
                let (sesh_name, _, _) = sessions[select_sesh];
                let (_, sesh, _) = sessions[select_sesh];
                let (_, _, sesh_var_name) = sessions[select_sesh];

                add_markdown(file_path_str, vec![format!("*{}*", sesh_name)]);
                add_cell(
                    file_path_str,
                    vec![
                        format!(
                            "{} = f1.get_session(2024, \"Qatar\", \"{}\")\n",
                            sesh_var_name, sesh
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

                let options = vec!["Yes", "No"];
                let mut drivers_analysis: Vec<(String, String, String)> = Vec::new();

                loop {
                    let do_analysis = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Analyze a driver?")
                        .default(0)
                        .items(&options)
                        .interact()
                        .unwrap();

                    match do_analysis {
                        0 => {
                            let drivers = driver_analysis(file_path_str, sesh_var_name);
                            for (abr, var, name) in drivers {
                                drivers_analysis.push((abr, var, name));
                            }
                            run_notebook(file_path_str, python_dir.clone());
                            println!("Driver Analysis complete.");
                        }
                        1 => {
                            println!("Exit Driver Analysis.");
                            break;
                        }
                        _ => (),
                    }
                }

                println!("Chart analysis for {} session.", sesh_var_name);
                let plot_data: Vec<(&str, &str, &str)> = drivers_analysis
                    .iter()
                    .map(|(a, b, c)| (a.as_str(), b.as_str(), c.as_str()))
                    .collect();
                generate_plot(file_path_str, "SpQuali", plot_data);
                run_notebook(file_path_str, python_dir.clone());
            }
        } else {
            println!("Failed to convert the file path to a valid string.");
        }
    } else {
        println!("No notebook selected or an error occurred.");
    }
}
