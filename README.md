# f1-telemetry-cli: F1 Telemetry Analysis Tool

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

`f1-telemetry-cli` is a command-line tool designed for analyzing Formula 1 racing telemetry data. It simplifies the process of creating and executing Jupyter Notebooks to explore various aspects of F1 race data, such as driver performance, track comparisons, pit stop strategies, and more.

## Features

-   **Interactive Session Selection:** Choose the year, Grand Prix, and specific session (Sprint Qualifying, Sprint Race, Qualifying, Race) you want to analyze through an interactive CLI interface.
-   **Automated Notebook Creation:** Automatically generates a Jupyter Notebook or modifies an existing one, pre-populating it with necessary Python code and data loading instructions.
-   **Virtual Environment Management:**
    -   Helps you create a virtual environment (`venv`) or use an existing one.
    -   Installs all required Python packages (numpy, pandas, matplotlib, seaborn, fastf1, ing_theme_matplotlib, mplcyberpunk, nbconvert) in the `requirements.txt` automatically.
-   **Driver Analysis:** Easily choose a driver and get analysis on them.
-   **Driver Position and Pit Stop Analysis:** Analyze driver positions during the race and view pit stop data.
-   **Fastest Speed Analysis:** Get data on the fastest speeds achieved during the race session.
-   **Track Comparison:** Enables you to compare driver performance on different parts of the track.
-   **Plot Generation:** Generates common telemetry plots.
-   **Seamless Notebook Execution:** It automates the process of running the generated Jupyter Notebook through the use of `nbconvert`.
- **Customizable:** Ability to select which driver, session and GP to analyse.

## Dependencies

-   **Rust:** This project is built using Rust. Make sure you have Rust installed.
-   **Python:** Python is required for the telemetry analysis, and a virtual environment is recommended.
- **Python Packages:** The following packages are automatically installed in the virtual environment.
    -   `numpy`
    -   `pandas`
    -   `matplotlib`
    -   `seaborn`
    -   `fastf1`
    -   `ing_theme_matplotlib`
    -   `mplcyberpunk`
    -   `nbconvert`

## Installation

1.  **Install Rust:** If you don't have Rust installed, follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2.  **Clone the Repository:**

    ```bash
    git clone [https://github.com/SohamDesai1/f1-telemetry-cli.git](https://github.com/SohamDesai1/f1-telemetry-cli.git)
    cd f1-telemetry-cli
    ```

3.  **Build the Project:**

    ```bash
    cargo build --release
    ```

4.  **Run the Executable:**

    ```bash
    ./target/release/f1-telemetry-cli
    ```

## Usage

1.  **Select a Jupyter Notebook:** Upon execution, the tool will prompt you to select an existing Jupyter Notebook or start with a new one.
2.  **Virtual Environment Setup:** It will then guide you through setting up a virtual environment (or selecting an existing one) and installing the required Python packages.
3.  **Interactive Prompts:** Follow the prompts to select:
    -   The year of the race.
    -   The Grand Prix.
    -   The session (Sprint Qualifying, Sprint Race, Qualifying, or Race).
    - Optionally, the drivers to analyse.
4.  **Notebook Population:** The tool will populate the notebook with Python code to:
    -   Import necessary libraries.
    -   Load the selected session data.
    -   Display session information and results.
    -   Add plots.
    -   Include driver and other analysis, if selected.
5.  **Notebook Execution:** After generating the notebook content, the tool will run it and display the results. You can also open the Jupyter Notebook manually to explore or modify it.
6. Repeat the process for different sessions.

## Example Workflow

1. Run the executable
2. Select a jupyter notebook
3. Create a new virtual environment
4. Wait for all of the dependencies to be installed.
5. Choose the year, session and GP to analyse
6. Add analysis for the session, if desired.
7. Watch the notebook run and print the analysis.

## Contributing

Contributions are welcome! If you'd like to contribute, please follow these steps:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Make your changes and commit them.
4.  Push your branch to your forked repository.
5.  Submit a pull request.

## License

This project is licensed under the Apache-2.0 License. See the `LICENSE` file for details.

## Contact

For any questions or support, please contact [Soham Desai](mailto:sohamcodesstuff@gmail.com).
