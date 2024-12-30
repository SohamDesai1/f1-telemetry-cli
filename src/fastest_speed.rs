use crate::add_cell::add_cell;

pub fn fastest_speed(file_path: &str) {
    let fastest_speed:Vec<String> = vec![
        "res = race.results\n".to_string(),
        "drivers = res['Abbreviation'].tolist()\n".to_string(),
        "fastest_speed = pd.DataFrame(columns=['Driver', 'Speed'])\n".to_string(),
        "for i in drivers:\n".to_string(),
        "    tele = race.laps.pick_driver(i)\n".to_string(),
        "    speed = tele.SpeedST.max()\n".to_string(),
        "    df = pd.DataFrame({'Driver': [i], 'Speed': [speed]})\n".to_string(),
        "    fastest_speed = pd.concat([fastest_speed, df], ignore_index=True)\n".to_string(),
        "fastest_speed.sort_values(by=\"Speed\",ascending=True)\n".to_string(),
    ];

    add_cell(file_path, fastest_speed);

    let fastest_speed_plot:Vec<String> = vec![
        "fig,ax = plt.subplots(figsize=(18, 6))\n".to_string(),
        "bars = ax.bar(fastest_speed['Driver'], fastest_speed['Speed'])\n".to_string(),
        "ax.set_xlabel('Drivers')\n".to_string(),
        "ax.set_ylabel('Speed')\n".to_string(),
        "ax.set_title('Maximum Speeds of Drivers')\n".to_string(),
        "plt.xticks(rotation=45, ha='right')\n".to_string(),
        "\n".to_string(),
        "y_min = 275\n".to_string(),
        "y_max = np.ceil(fastest_speed['Speed'].max() / 5) * 5 + 5\n".to_string(),
        "y_ticks = np.arange(y_min, y_max, 5)\n".to_string(),
        "ax.set_ylim(y_min, y_max)\n".to_string(),
        "ax.set_yticks(y_ticks)\n".to_string(),
        "\n".to_string(),
        "for bar in bars:\n".to_string(),
        "    height = bar.get_height()\n".to_string(),
        "    ax.annotate('{:.2f}'.format(height),\n".to_string(),
        "                xy=(bar.get_x() + bar.get_width() / 2, height),\n".to_string(),
        "                xytext=(0, 3),\n".to_string(),
        "                textcoords=\"offset points\",\n".to_string(),
        "                ha='center', va='bottom')\n".to_string(),
        "\n".to_string(),
        "plt.show()".to_string()
    ];

    add_cell(file_path, fastest_speed_plot);
}