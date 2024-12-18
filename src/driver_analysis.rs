use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::add_cell::add_cell;

pub fn driver_analysis(file_path: &str) {
    let drivers = vec![
        ("VER", "Max Verstappen".truecolor(54, 113, 198)),
        ("NOR", "Lando Norris".truecolor(255, 128, 0)),
        ("SAI", "Carlos Sainz".truecolor(232, 0, 32)),
        ("LEC", "Charles Leclerc".truecolor(232, 0, 32)),
        ("PIA", "Oscar Piastri".truecolor(255, 128, 0)),
        ("HAM", "Lewis Hamilton".truecolor(39, 244, 210)),
        ("RUS", "George Russell".truecolor(39, 244, 210)),
        ("GAS", "Pierre Gasly".truecolor(0, 147, 204)),
        ("HUL", "Nico Hulkenberg".truecolor(182, 186, 189)),
        ("LAW", "Liam Lawson".truecolor(102, 146, 255)),
        ("ALO", "Fernando Alonso".truecolor(34, 153, 113)),
        ("ALB", "Alexander Albon".truecolor(100, 196, 255)),
        ("BOT", "Valtteri Bottas".truecolor(82, 226, 82)),
        ("STR", "Lance Stroll".truecolor(34, 153, 113)),
        ("MAG", "Kevin Magnussen".truecolor(182, 186, 189)),
        ("PER", "Sergio Perez".truecolor(54, 113, 198)),
        ("TSU", "Yuki Tsunoda".truecolor(102, 146, 255)),
        ("OCO", "Esteban Ocon".truecolor(0, 147, 204)),
        ("ZHO", "Guanyu Zhou".truecolor(82, 226, 82)),
        ("COL", "Franco Colapinto".truecolor(100, 196, 255)),
    ];

    let display_names: Vec<_> = drivers.iter().map(|(_, full_name)| full_name).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a driver you want to analyze:")
        .default(0)
        .items(&display_names)
        .interact()
        .unwrap();

    let (abbreviation, _) = drivers[selection];

    let code1 = vec![
        format!(
            "{} = SpQuali.get_driver(\"{}\")\n",
            abbreviation.to_lowercase(),
            abbreviation
        ),
        abbreviation.to_lowercase(),
    ];
    add_cell(file_path, code1);

    let code2 = vec![
        format!(
            "{}_laps = SpQuali.laps.pick_driver(\"{}\")\n",
            abbreviation.to_lowercase(),
            abbreviation
        ),
        format!(
            "{}_lap_time = {}_laps[\"LapTime\"]",
            abbreviation.to_lowercase(),
            abbreviation.to_lowercase(),
        ),
        format!(
            "{}_sec1 = {}_laps[\"Sector1Time\"]",
            abbreviation.to_lowercase(),
            abbreviation.to_lowercase(),
        ),
        format!(
            "{}_sec2 = {}_laps[\"Sector2Time\"]",
            abbreviation.to_lowercase(),
            abbreviation.to_lowercase(),
        ),
        format!(
            "{}_sec3 = {}_laps[\"Sector3Time\"]",
            abbreviation.to_lowercase(),
            abbreviation.to_lowercase(),
        ),
        format!("{}_laps.loc[:,[\"LapTime\",\"Sector1Time\",\"Sector2Time\",\"Sector3Time\",\"SpeedI1\",\"SpeedI2\",\"SpeedFL\",\"SpeedST\",]]", abbreviation.to_lowercase()),
    ];

    add_cell(file_path, code2);

    let code3 = vec![
        format!(
            "fastest_{} = {}_laps.pick_fastest()\n",
            abbreviation.to_lowercase(),
            abbreviation.to_lowercase(),
        ),
        format!(
            "tele_{} = fastest_{}.get_telemetry().add_distance()\n",
            abbreviation.to_lowercase(),
            abbreviation.to_lowercase(),
        ),
        format!(
            "tele_{}.loc[:, [\"Speed\", \"nGear\", \"RPM\"]]",
            abbreviation.to_lowercase(),
        ),
    ];

    add_cell(file_path, code3);
}
