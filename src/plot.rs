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
        "fig, ax = plt.subplots(2, 1, figsize=(7, 7), height_ratios=[2, 1])\n".to_string(),
        "\n".to_string(),
    ];

    for (driver_var, driver_abbr, driver_name) in &drivers {
        plot.push(format!(
                "ax[0].plot(\n    tele_{var}.Distance, tele_{var}.Speed, label=\"{name}\", color=get_driver_color(\"{abbr}\")\n)\n",
                abbr = driver_abbr,
                var = driver_var,
                name = driver_name
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
        "\n".to_string(),
    ]);

    for (driver_var, driver_abbr, driver_name) in &drivers {
        plot.push(format!(
                "ax[1].plot(\n    tele_{var}.Distance, tele_{var}.Throttle, label=\"{name}\", color=get_driver_color(\"{abbr}\")\n)\n",
                abbr = driver_abbr,
                var = driver_var,
                name = driver_name
            ));
    }

    plot.extend(vec![
        "\n".to_string(),
        "ax[1].set_title(\"Throttle vs Distance\", fontsize=14)\n".to_string(),
        "ax[1].set_xlabel(\"Distance\", fontsize=10)\n".to_string(),
        "ax[1].set_ylabel(\"Throttle\", fontsize=10)\n".to_string(),
        "ax[1].legend(fontsize=8)\n".to_string(),
        "ax[1].grid(True)\n".to_string(),
        "ax[1].tick_params(axis=\"both\", which=\"major\", labelsize=8)\n".to_string(),
        "\n".to_string(),
        "plt.tight_layout()\n".to_string(),
        "plt.show()\n".to_string(),
    ]);

    let merged_cell = functions.into_iter().chain(plot.into_iter()).collect();
    add_cell(file_path, merged_cell);
}
