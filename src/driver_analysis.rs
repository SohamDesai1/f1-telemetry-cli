use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::add_cell::add_cell;

pub fn driver_analysis(file_path: &str, sesh_var_name: &str) -> Vec<(String, String, String)> {
    let mut drivers = vec![
        ("VER", "Max Verstappen".truecolor(54, 113, 198), 0),
        ("NOR", "Lando Norris".truecolor(255, 128, 0), 0),
        ("SAI", "Carlos Sainz".truecolor(232, 0, 32), 0),
        ("LEC", "Charles Leclerc".truecolor(232, 0, 32), 0),
        ("PIA", "Oscar Piastri".truecolor(255, 128, 0), 0),
        ("HAM", "Lewis Hamilton".truecolor(39, 244, 210), 0),
        ("RUS", "George Russell".truecolor(39, 244, 210), 0),
        ("GAS", "Pierre Gasly".truecolor(0, 147, 204), 0),
        ("HUL", "Nico Hulkenberg".truecolor(182, 186, 189), 0),
        ("LAW", "Liam Lawson".truecolor(102, 146, 255), 0),
        ("ALO", "Fernando Alonso".truecolor(34, 153, 113), 0),
        ("ALB", "Alexander Albon".truecolor(100, 196, 255), 0),
        ("BOT", "Valtteri Bottas".truecolor(82, 226, 82), 0),
        ("STR", "Lance Stroll".truecolor(34, 153, 113), 0),
        ("MAG", "Kevin Magnussen".truecolor(182, 186, 189), 0),
        ("PER", "Sergio Perez".truecolor(54, 113, 198), 0),
        ("TSU", "Yuki Tsunoda".truecolor(102, 146, 255), 0),
        ("OCO", "Esteban Ocon".truecolor(0, 147, 204), 0),
        ("ZHO", "Guanyu Zhou".truecolor(82, 226, 82), 0),
        ("COL", "Franco Colapinto".truecolor(100, 196, 255), 0),
    ];

    let display_names: Vec<_> = drivers.iter().map(|(_, full_name, _)| full_name).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a driver you want to analyze:")
        .default(0)
        .items(&display_names)
        .interact()
        .unwrap();

    if let Some(driver) = drivers.get_mut(selection) {
        driver.2 += 1;
    }
    let (_, full_name, __) = &drivers[selection];
    let (abbreviation, _, _) = drivers[selection];

    if drivers[selection].2 > 1 {
        let code1 = vec![
            format!(
                "{} = {}.get_driver(\"{}\")\n",
                abbreviation.to_lowercase(),
                sesh_var_name,
                abbreviation
            ),
            abbreviation.to_lowercase(),
        ];
        add_cell(file_path, code1);
    }

    let code2 = vec![
        format!(
            "{name}_{sesh}_laps = {sesh}.laps.pick_driver(\"{NAME}\")\n",
            name = abbreviation.to_lowercase(),
            sesh = sesh_var_name,
            NAME = abbreviation
        ),
        format!("fastest_lap_{name}_{sesh} = {name}_{sesh}_laps.pick_fastest().LapTime\n",name = abbreviation.to_lowercase(), sesh = sesh_var_name),
        format!(
            "{name}_{sesh}_lap_time = {name}_{sesh}_laps[\"LapTime\"]\n",
            name = abbreviation.to_lowercase(),
            sesh = sesh_var_name
        ),
        format!(
            "{name}_sec1 = {name}_{sesh}_laps[\"Sector1Time\"]\n",
            name = abbreviation.to_lowercase(),
            sesh = sesh_var_name
        ),
        format!(
            "{name}_sec2 = {name}_{sesh}_laps[\"Sector2Time\"]\n",
            name = abbreviation.to_lowercase(),
            sesh = sesh_var_name
        ),
        format!(
            "{name}_sec3 = {name}_{sesh}_laps[\"Sector3Time\"]\n",
            name = abbreviation.to_lowercase(),
            sesh = sesh_var_name
        ),
        format!(
            "print(\"Fastest Lap:\", convert_to_normal(fastest_lap_{name}_{sesh}), \"which was in Lap number\" ,{name}_{sesh}_laps.pick_fastest().LapNumber)\n",name = abbreviation.to_lowercase(), sesh = sesh_var_name
        ),
        format!("{name}_{sesh}_laps.loc[:,[\"LapTime\",\"Sector1Time\",\"Sector2Time\",\"Sector3Time\",\"SpeedI1\",\"SpeedI2\",\"SpeedFL\",\"SpeedST\"]].head()", name = abbreviation.to_lowercase(),
        sesh = sesh_var_name),
    ];

    add_cell(file_path, code2);

    let code3 = vec![
        format!(
            "fastest_lap_{name}_{sesh} = {name}_{sesh}_laps.pick_fastest()\n",
            name = abbreviation.to_lowercase(),
            sesh = sesh_var_name,
        ),
        format!(
            "tele_{name}_{sesh} = fastest_lap_{name}_{sesh}.get_telemetry().add_distance()\n",
            name = abbreviation.to_lowercase(),
            sesh = sesh_var_name,
        ),
        format!(
            "speed_{name}_{sesh} = tele_{name}_{sesh}.Speed\n",
            sesh = sesh_var_name,
            name = abbreviation.to_lowercase()
        ),
        format!(
            "tele_{}_{}.loc[:, [\"Speed\", \"nGear\", \"RPM\"]]",
            abbreviation.to_lowercase(),
            sesh_var_name,
        ),
    ];

    add_cell(file_path, code3);

    return vec![(
        abbreviation.to_lowercase(),
        abbreviation.to_string(),
        full_name.to_string(),
    )];
}
