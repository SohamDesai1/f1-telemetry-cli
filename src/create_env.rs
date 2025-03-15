use std::{
    fs::{read_dir, File},
    io::{stdout, Write},
    path::{Path, PathBuf},
    process::{exit, Command},
    thread,
    time::Duration,
};

use dialoguer::{theme::ColorfulTheme, Select};
use native_dialog::FileDialog;

pub fn create_venv(file_path: &str) -> Option<PathBuf> {
    println!("Please select the appropriate option:");
    let items = vec!["Select a virtual env (select the folder)", "Create a virtual env"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option:")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    match selection {
        0 => {
            let python_dir = FileDialog::new().show_open_single_dir().unwrap();
            if let Some(dir) = python_dir {
                let bin_folder = if cfg!(target_os = "windows") {
                    "Scripts"
                } else {
                    "bin"
                };

                if dir.join(bin_folder).is_dir() {
                    for entry in read_dir(dir.join(bin_folder)).expect("Failed to read bin folder")
                    {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            let python_exe = if cfg!(target_os = "windows") {
                                "python.exe"
                            } else {
                                "python3"
                            };
                            if path.file_name().unwrap_or_default() == python_exe {
                                println!("Found Python executable: {}", path.display());
                                return Some(path);
                            }
                        }
                    }
                }
                println!("No Python executable found in the {} folder.", bin_folder);
            } else {
                println!("No directory selected.");
            }
            None
        }
        1 => {
            let notebook_dir = Path::new(file_path)
                .parent()
                .expect("Invalid file path provided.");

            let is_windows = cfg!(target_os = "windows");
            let venv_path = notebook_dir.join("venv");

            if venv_path.exists() {
                println!("Virtual environment already exists at {:?}", notebook_dir);
            } else {
                println!("Creating virtual environment...");

                let python_cmd = if is_windows { "python" } else { "python3" };

                let output = Command::new(python_cmd)
                    .arg("-m")
                    .arg("venv")
                    .arg(&venv_path)
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

            let requirements_path = notebook_dir.join("requirements.txt");

            if let Ok(mut file) = File::create(&requirements_path) {
                let content = "numpy\npandas\nmatplotlib\nseaborn\nfastf1\ning_theme_matplotlib\nmplcyberpunk\nnbconvert";
                if file.write_all(content.as_bytes()).is_err() {
                    eprintln!("Failed to write to requirements.txt");
                } else {
                    println!("requirements.txt file created successfully!");
                }
            } else {
                println!("Error occurred while creating requirements.txt");
            }

            let pip_path = if is_windows {
                venv_path.join("Scripts").join("pip.exe")
            } else {
                venv_path.join("bin").join("pip")
            };

            let install_thread = thread::spawn(move || {
                let install_output = Command::new(&pip_path)
                    .arg("install")
                    .arg("-r")
                    .arg(requirements_path)
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

            print!("Installing packages");
            stdout().flush().unwrap();

            loop {
                print!(".");
                stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(500));

                if install_thread.is_finished() {
                    break;
                }
            }

            install_thread.join().unwrap();

            let python_path = if is_windows {
                venv_path.join("Scripts").join("python.exe")
            } else {
                venv_path.join("bin").join("python")
            };

            Some(python_path)
        }
        _ => {
            eprintln!("Invalid selection.");
            exit(1);
        }
    }
}
