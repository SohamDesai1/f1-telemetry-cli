use crate::add_cell::add_cell;

pub fn generate_plot(file_path: &str, sesh_var_name: &str, drivers: Vec<(&str, &str, &str)>) {
    let functions = vec![
        "def hex_to_rgb(hex_color):\n".to_string(),
        "    hex_color = hex_color.lstrip(\"#\")\n".to_string(),
        "    r, g, b = [int(hex_color[i : i + 2], 16) for i in (0, 2, 4)]\n".to_string(),
        "    return (r / 255, g / 255, b / 255)\n".to_string(),
        "\n".to_string(),
        "\n".to_string(),
        "def get_driver_color(driver_abbreviation):\n".to_string(),
        format!(
            "    driver = {}.get_driver(driver_abbreviation)\n",
            sesh_var_name
        ),
        "    team_color = driver.TeamColor\n".to_string(),
        "    return hex_to_rgb(team_color)\n".to_string(),
        "\n".to_string(),
        "\n".to_string(),
    ];

    let mut plot = vec![
        "plt.style.use(\"cyberpunk\")\n".to_string(),
        "\n".to_string(),
        "fig, ax = plt.subplots(4, 1, figsize=(7, 7), gridspec_kw={\"height_ratios\": [4, 1, 1, 1]})\n".to_string(),
        "\n".to_string(),
    ];

    for (driver_var, driver_abbr, driver_name) in &drivers {
        plot.push(format!(
                "ax[0].plot(\n    tele_{var}_{sesh}.Distance, tele_{var}_{sesh}.Speed, label=\"{name}\", color=get_driver_color(\"{abbr}\")\n)\n",
                abbr = driver_abbr,
                var = driver_var,
                name = driver_name,
                sesh = sesh_var_name
            ));
    }

    plot.extend(vec![
        "\n".to_string(),
        "ax[0].set_title(\"Speed vs Distance\", fontsize=16)\n".to_string(),
        "ax[0].set_xlabel(\"Distance\", fontsize=12)\n".to_string(),
        "ax[0].set_ylabel(\"Speed\", fontsize=12)\n".to_string(),
        "ax[0].legend(fontsize=10)\n".to_string(),
        "ax[0].grid(True)\n".to_string(),
        "ax[0].tick_params(axis=\"both\", which=\"major\", labelsize=10)\n".to_string(),
    ]);

    for (driver_var, driver_abbr, driver_name) in &drivers {
        plot.push(format!(
            "ax[1].plot(\n    tele_{var}_{sesh}.Distance, tele_{var}_{sesh}.Throttle, label=\"{name}\", color=get_driver_color(\"{abbr}\")\n)\n",
            abbr = driver_abbr,
            var = driver_var,
            name = driver_name,
            sesh = sesh_var_name
        ));
    }

    let mut sectors: Vec<String> = vec![];
    for (index, (driver_var, _, driver_name)) in drivers.iter().enumerate() {
        sectors.push(format!("driver{i}_sectors = pd.DataFrame(\n    {{\n         \"Driver\": [\"{name}\"] * len({var}_{sesh}_sec1),\n        \"Sector1Time\": {var}_{sesh}_sec1,\n        \"Sector2Time\": {var}_{sesh}_sec2,\n        \"Sector3Time\": {var}_{sesh}_sec3,\n        \"Lap Time\": {var}_{sesh}_lap_time,\n    }}\n)\n\n\n",i = index +1,name = driver_name,var = driver_var,sesh =sesh_var_name),);
    }

    let concat_drivers: String = (0..drivers.len())
    .map(|i| format!("driver{i}_sectors", i = i + 1))
    .collect::<Vec<String>>()
    .join(", ");

    sectors.push(format!("all_drivers_sectors = pd.concat(\n    [{}],\n    ignore_index=True,\n)\nall_drivers_sectors.sort_values(\n    by=[\"Sector1Time\", \"Sector2Time\", \"Sector3Time\"], ascending=True\n)\nall_drivers_sectors.dropna()",concat_drivers));

    add_cell(file_path, sectors);

    plot.extend(vec![
        "\n".to_string(),
        "ax[1].set_title(\"Throttle vs Distance\", fontsize=14)\n".to_string(),
        "ax[1].set_xlabel(\"Distance\", fontsize=10)\n".to_string(),
        "ax[1].set_ylabel(\"Throttle\", fontsize=10)\n".to_string(),
        "ax[1].legend(fontsize=8)\n".to_string(),
        "ax[1].grid(True)\n".to_string(),
        "ax[1].tick_params(axis=\"both\", which=\"major\", labelsize=8)\n".to_string(),
    ]);

    for (driver_var, driver_abbr, driver_name) in &drivers {
        plot.push(format!(
            "ax[2].plot(\n    tele_{var}_{sesh}.Distance, tele_{var}_{sesh}.nGear, label=\"{name}\", color=get_driver_color(\"{abbr}\")\n)\n",
            abbr = driver_abbr,
            var = driver_var,
            name = driver_name,
            sesh = sesh_var_name
        ));
    }

    plot.extend(vec![
        "\n".to_string(),
        "ax[2].set_title(\"nGear vs Distance\", fontsize=14)\n".to_string(),
        "ax[2].set_xlabel(\"Distance\", fontsize=10)\n".to_string(),
        "ax[2].set_ylabel(\"Gear\", fontsize=10)\n".to_string(),
        "ax[2].legend(fontsize=8)\n".to_string(),
        "ax[2].grid(True)\n".to_string(),
        "ax[2].tick_params(axis=\"both\", which=\"major\", labelsize=8)\n".to_string(),
    ]);

    for (driver_var, driver_abbr, driver_name) in &drivers {
        plot.push(format!(
            "ax[3].plot(\n    tele_{var}_{sesh}.Distance, tele_{var}_{sesh}.Brake, label=\"{name}\", color=get_driver_color(\"{abbr}\")\n)\n",
            abbr = driver_abbr,
            var = driver_var,
            name = driver_name,
            sesh = sesh_var_name
        ));
    }

    plot.extend(vec![
        "\n".to_string(),
        "ax[3].set_title(\"nGear vs Distance\", fontsize=14)\n".to_string(),
        "ax[3].set_xlabel(\"Distance\", fontsize=10)\n".to_string(),
        "ax[3].set_ylabel(\"Brake\", fontsize=10)\n".to_string(),
        "ax[3].legend(fontsize=8)\n".to_string(),
        "ax[3].grid(True)\n".to_string(),
        "ax[3].tick_params(axis=\"both\", which=\"major\", labelsize=8)\n".to_string(),
        "\n".to_string(),
        "plt.tight_layout()\n".to_string(),
        "plt.show()\n".to_string(),
    ]);

    let merged_cell = functions.into_iter().chain(plot.into_iter()).collect();
    add_cell(file_path, merged_cell);

    let mut sectors: Vec<String> = vec![];
    let mut sectors_graph: Vec<String> = vec![];

    for (index, (driver_var, _, driver_name)) in drivers.iter().enumerate() {
        sectors.push(format!("driver{i}_sectors = pd.DataFrame(\n    {{\n         \"Driver\": [\"{name}\"] * len({var}_{sesh}_sec1),\n        \"Sector1Time\": {var}_{sesh}_sec1,\n        \"Sector2Time\": {var}_{sesh}_sec2,\n        \"Sector3Time\": {var}_{sesh}_sec3,\n        \"Lap Time\": {var}_{sesh}_lap_time,\n    }}\n)\n\n\n",i = index +1,name = driver_name,var = driver_var,sesh =sesh_var_name),);
    }

    let concat_drivers: String = (0..drivers.len())
        .map(|i| format!("driver{i}_sectors", i = i + 1))
        .collect::<Vec<String>>()
        .join(", ");

    sectors.push(format!("all_drivers_sectors = pd.concat(\n    [{}],\n    ignore_index=True,\n)\nall_drivers_sectors.sort_values(\n    by=[\"Sector1Time\", \"Sector2Time\", \"Sector3Time\"], ascending=True\n)\nall_drivers_sectors.dropna()",concat_drivers));

    add_cell(file_path, sectors);

    sectors_graph.push(format!("from datetime import timedelta\n\nmelted_data = all_drivers_sectors.melt(\n    id_vars=[\"Driver\"],\n    value_vars=[\"Sector1Time\", \"Sector2Time\", \"Sector3Time\"],\n    var_name=\"Sector\",\n    value_name=\"Time\",\n)\n\nplt.figure(figsize=(12, 6))\nbar_plot = sns.barplot(\n    data=melted_data, x=\"Sector\", y=\"Time\", hue=\"Driver\", palette=\"viridis\"\n)\n\nfor p in bar_plot.patches:\n    height = p.get_height()\n    bar_plot.annotate(\n        f\"{{timedelta(seconds=height)}}\"\n        (p.get_x() + p.get_width() / 2.0, height),\n        ha=\"center\",\n        va=\"bottom\",\n        fontsize=10,\n        color=\"black\",\n        xytext=(0, 5),\n        textcoords=\"offset points\",\n    )\n\nplt.title(\"Sector Times Comparison\")\nplt.ylabel(\"Time (seconds)\")\nplt.xlabel(\"Sector\")\nplt.gca().set_yticks([])\nplt.gca().invert_yaxis()\nplt.show()"));

    add_cell(file_path, sectors_graph);
}
