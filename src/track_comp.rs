use colored::Colorize;
use dialoguer::MultiSelect;
use regex::Regex;

use crate::add_cell::{add_cell, add_markdown};

pub fn track_comparison(file_path: &str) {
    add_markdown(file_path, vec!["*Pace Comparison*".to_string()]);
    add_cell(file_path, vec!["plotting.setup_mpl()".to_string()]);

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
    loop {
        let selections = MultiSelect::new()
            .with_prompt("Select 2 drivers to compare: (Space to select, Enter to confirm)")
            .items(&display_names)
            .interact()
            .unwrap();

        if selections.len() == 2 {
            println!("Pace comparison between {} and {}", display_names[selections[0]], display_names[selections[1]]);
            let selected_drivers: Vec<_> =
                selections.iter().map(|&index| &drivers[index]).collect();

            let (abbreviation1, full_name1) = selected_drivers[0];
            let (abbreviation2, full_name2) = selected_drivers[1];

            let telemetry_cell = vec![format!("laps_{name1} = race_laps.pick_driver(\"{abbr1}\")\nlaps_{name2} = race_laps.pick_driver(\"{abbr2}\")\n\nfastest_{name1} = laps_{name1}.pick_fastest().get_telemetry().add_distance()\nfastest_{name2} = laps_{name2}.pick_fastest().get_telemetry().add_distance()\n\nfastest_{name1}[\"Driver\"] = \"{abbr1}\"\nfastest_{name2}[\"Driver\"] = \"{abbr2}\"\ntelemetry = pd.concat([fastest_{name1},fastest_{name2}])", name1 = abbreviation1.to_lowercase(), abbr1 = abbreviation1,name2 = abbreviation2.to_lowercase(), abbr2 = abbreviation2)];
            add_cell(file_path, telemetry_cell);

            let minisecs_cell = vec![
                "num_minisectors = 25\n".to_string(),
                "total_distance = total_distance = max(telemetry['Distance'])\n".to_string(),
                "minisector_length = total_distance / num_minisectors".to_string(),
                "\n".to_string(),
                "minisectors = [0]\n".to_string(),
                "for i in range(0, (num_minisectors - 1)):\n".to_string(),
                "    minisectors.append(minisector_length * (i + 1))".to_string(),
                "\n".to_string(),
                "telemetry['Minisector'] = telemetry['Distance'].apply(\n".to_string(),
                "    lambda dist: (\n".to_string(),
                "        int((dist // minisector_length) + 1)\n".to_string(),
                "    )\n".to_string(),
                ")".to_string(),
            ];
            add_cell(file_path, minisecs_cell);

            let avgsp_cell = vec!["average_speed = telemetry.groupby(['Minisector', 'Driver'])['Speed'].mean().reset_index()\n".to_string(),
    "average_speed".to_string()];
            add_cell(file_path, avgsp_cell);

            let fastestdr_cell = vec!["fastest_driver = average_speed.loc[average_speed.groupby(['Minisector'])['Speed'].idxmax()]\n".to_string(),
    "\n".to_string(),
    "fastest_driver = fastest_driver[['Minisector', 'Driver']].rename(columns={'Driver': 'Fastest_driver'})\n".to_string()];
            add_cell(file_path, fastestdr_cell);

            let telemetry_cell2 = vec![
                "telemetry = telemetry.merge(fastest_driver, on=['Minisector'])\n".to_string(),
                "\n".to_string(),
                "telemetry = telemetry.sort_values(by=['Distance'])\n".to_string(),
                "\n".to_string(),
                format!(
            "telemetry.loc[telemetry['Fastest_driver'] == '{}', 'Fastest_driver_int'] = 1\n",
            abbreviation1
        ),
                format!(
                    "telemetry.loc[telemetry['Fastest_driver'] == '{}', 'Fastest_driver_int'] = 2",
                    abbreviation2
                ),
            ];
            add_cell(file_path, telemetry_cell2);

            let plot_cell = vec![
                "x = np.array(telemetry['X'].values)\n".to_string(),
                "y = np.array(telemetry['Y'].values)\n".to_string(),
                "\n".to_string(),
                "points = np.array([x, y]).T.reshape(-1, 1, 2)\n".to_string(),
                "segments = np.concatenate([points[:-1], points[1:]], axis=1)\n".to_string(),
                "fastest_driver_array = telemetry['Fastest_driver_int'].to_numpy().astype(float)\n"
                    .to_string(),
                "\n".to_string(),
                "cmap = plt.get_cmap('winter', 2)  # Create a colormap with 2 colors\n".to_string(),
                "lc_comp = LineCollection(segments, norm=plt.Normalize(1, cmap.N+1), cmap=cmap)\n"
                    .to_string(),
                "lc_comp.set_array(fastest_driver_array)\n".to_string(),
                "lc_comp.set_linewidth(5)\n".to_string(),
                "\n".to_string(),
                "plt.rcParams['figure.figsize'] = [18, 10]\n".to_string(),
                "plt.gca().add_collection(lc_comp)\n".to_string(),
                "plt.axis('equal')\n".to_string(),
                "plt.tick_params(labelleft=False, left=False, labelbottom=False, bottom=False)\n"
                    .to_string(),
                "\n".to_string(),
                "cbar = plt.colorbar(mappable=lc_comp, boundaries=[1, 2, 3])\n".to_string(),
                "cbar.set_ticks([1.5, 2.5])\n".to_string(),
                format!(
                    "cbar.set_ticklabels(['{}', '{}'])\n",
                    abbreviation1, abbreviation2
                ),
                format!("plt.title(\"{} vs {} Pace\")\n", remove_ansi_codes(full_name1), remove_ansi_codes(full_name2)),
                "plt.show()\n".to_string(),
            ];
            add_cell(file_path, plot_cell);
            break;
        } else {
            println!("You must select exactly 2 drivers. Please try again.");
        }
    }
}

fn remove_ansi_codes(input: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(input, "").to_string()
}