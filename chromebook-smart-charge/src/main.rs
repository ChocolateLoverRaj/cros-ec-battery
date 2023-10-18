use std::time::Duration;

use chrono::Utc;
use ectool::battery::battery;
mod ectool;
mod find_lowest_current_above;
use find_lowest_current_above::find_lowest_current_above;
use ms_converter::HOUR;
use toml::Table;

const CONFIG_FILE: &str = "charge_settings.toml";

#[derive(Debug)]
struct Config {
    pub target_energy: i64,
    pub target_time: i64,
}

async fn parse_config() -> Config {
    let config = tokio::fs::read_to_string(CONFIG_FILE)
        .await
        .unwrap()
        .parse::<Table>()
        .unwrap();
    let target_energy = config["target_energy"].as_integer().unwrap();
    let target_time = config["target_time"].as_integer().unwrap().to_owned();
    Config {
        target_energy,
        target_time,
    }
}

#[tokio::main]
async fn main() {
    let config = parse_config().await;
    let now = Utc::now().naive_utc().timestamp_millis();
    let time_until_target = (config.target_time - now).try_into().unwrap();
    let battery_info = battery().await.unwrap();
    let target_energy_mah =
        (config.target_energy as f64) / (100 as f64) * (battery_info.last_full_charge as f64);
    let difference_energy = target_energy_mah - (battery_info.remaining_capacity as f64);
    let perfect_current = target_energy_mah / ((time_until_target as f64) / HOUR);

    let lowest_current_above = find_lowest_current_above(10).await;

    println!(
        "{:#?} {}  {target_energy_mah} {difference_energy} {perfect_current} {lowest_current_above}",
        config,
        // time_until_target,
        humantime::format_duration(Duration::from_millis(time_until_target))
    );

    // let result = hello().await.unwrap();
    // println!("{result}");

    // let battery_info = battery().await.unwrap();
    // println!("{:#?}", battery_info);

    // let charge_control = charge_control::get().await.unwrap();
    // println!("{:#?}", charge_control);

    // charge_control::set(ChargeControl::Normal(Some(Sustainer { min: 80, max: 80 })))
    //     .await
    //     .unwrap();

    // charge_current_limit::set(250).await.unwrap();
}
