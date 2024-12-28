use std::{
    fs::{read_dir, File},
    io::Write,
    path::{Path, PathBuf},
    process::{exit, Command},
    thread,
    time::Duration,
};

use dialoguer::{theme::ColorfulTheme, Select};
use native_dialog::FileDialog;

pub fn create_venv(file_path: &str) -> Option<PathBuf> {
    println!("Please select the appropriate option:");
    let items = vec!["Select a virtual env", "make a virtual env"];

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
            if notebook_dir.join("venv").exists() {
                println!("Virtual environment already exists at {:?}", notebook_dir);
            } else {
                println!("Creating virtual environment...");

                let output = Command::new("python3")
                    .arg("-m")
                    .arg("venv")
                    .arg(notebook_dir.join("venv"))
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
                    let content = "numpy\npandas\nmatplotlib\nseaborn\nfastf1\ning_theme_matplotlib\nmplcyberpunk\nnbconvert";
                    if let Err(e) = file.write_all(content.as_bytes()) {
                        eprintln!("Failed to write to file: {}", e);
                    } else {
                        println!("requirements.txt file created successfully!",);
                    }
                }
                Err(_) => println!("Error occured creating notebook"),
            }

            let pip = notebook_dir.join("venv").join("bin").join("pip");

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
            python = Some(notebook_dir.join("venv").join("bin").join("python"));
            return python;
        }
        _ => {
            eprintln!("Invalid selection.");
            exit(1);
        }
    }
}
