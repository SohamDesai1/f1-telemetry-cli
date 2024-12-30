use crate::add_cell::add_cell;

pub fn drivers_positions(file_path: &str){
    let get_dr:Vec<String> = vec![format!("drivers = race.laps.Driver.unique()\ndrivers_ab = drivers.tolist()\nnlaps = int(race.laps.LapNumber.unique().max())\n\nteam_colors = {{}}\n\nfor driver in drivers_ab:\n    driver_object = race.get_driver(driver)\n    hex_color = driver_object.TeamColor\n    team_colors[driver] = hex_to_rgb(hex_color)\n\nplt.figure(figsize=(10, 6))\nfor driver in drivers_ab:\n    positions = race_laps[race_laps['Driver'] == driver]['Position'].values  \n    color = team_colors.get(driver, (0.5, 0.5, 0.5))\n    plt.plot(range(1, len(positions) + 1), positions, label=driver, color=color)\n    \n    plt.text(\n        1 - 0.2,\n        positions[0],\n        driver,\n        color=color,\n        fontsize=10,\n        verticalalignment='center',\n        horizontalalignment='right'\n    )\n    \n    plt.text(\n        len(positions) + 0.2,\n        positions[-1],\n        driver,\n        color=color,\n        fontsize=10,\n        verticalalignment='center',\n        horizontalalignment='left'\n    )\n\nplt.title(\"Driver Positions Over Laps\", fontsize=14)\nplt.grid(True, linestyle=':', alpha=0.5)\nplt.xlim(0.5, float(nlaps) + 1.5)\nplt.xlabel(\"Lap Number\")\nplt.ylim(0, 21)\nplt.gca().invert_yaxis()\nplt.gca().set_yticks([])\nplt.xticks(range(1, nlaps + 1))\nplt.tight_layout()\nplt.show()")];

    add_cell(file_path, get_dr);


}