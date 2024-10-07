use std::time::Duration;

use anyhow::anyhow;
use clap::Parser;
use crosec::{
    commands::{
        charge_control::{set_charge_control, ChargeControl},
        charge_current_limit::set_charge_current_limit,
    },
    CROS_EC_PATH,
};
use starship_battery::{
    units::{self, time::hour, Ratio, Time},
    Battery,
};
use tokio::{fs::File, time::sleep};
use try_again::{retry_async, Delay, Retry, TokioSleep};
use zbus::Connection;
use zbus_systemd::login1::ManagerProxy;

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// The percent to charge to (0.8 would be 80%)
    #[arg(long, value_parser = charge_to_parser)]
    charge_to: f32,
    /// In C, where 0.5C means it will take 2 hours to charge. If none is specified, the charge speed will not be limited.
    #[arg(long, value_parser = max_charge_speed_parser)]
    max_charge_speed: Option<f32>,
}

fn charge_to_parser(s: &str) -> Result<f32, String> {
    let charge_to = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
    if (0.0..=1.0).contains(&charge_to) {
        Ok(charge_to)
    } else {
        Err("Charge to value should be between 0 and 1".into())
    }
}

fn max_charge_speed_parser(s: &str) -> Result<f32, String> {
    let charge_to = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
    if charge_to > 0.0 {
        Ok(charge_to)
    } else {
        Err("Charge speed must be greater than 0C".into())
    }
}

trait ReachedRatio {
    fn reached_ratio(&self, ratio: Ratio) -> bool;
}

impl ReachedRatio for Battery {
    fn reached_ratio(&self, ratio: Ratio) -> bool {
        self.energy() / self.energy_full() >= ratio
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli {
        charge_to,
        max_charge_speed,
    } = Cli::parse();

    let connection = Connection::system().await?;
    let manager = ManagerProxy::new(&connection).await?;
    let fd = {
        // Sometimes it doesn't work if we attempt to get an inhibitor lock immediately after resume from suspend. So we retry after a short delay.
        retry_async(
            Retry {
                max_tries: 100,
                delay: Some(Delay::Static {
                    delay: Duration::from_millis(500),
                }),
            },
            TokioSleep {},
            || {
                manager.inhibit(
                    "sleep".into(),
                    "Charging Service".into(),
                    "Charge up to a certain energy".into(),
                    "delay".into(),
                )
            },
        )
        .await?
    };
    let manager = starship_battery::Manager::new()?;
    let mut battery = manager
        .batteries()?
        .next()
        .ok_or(anyhow!("No batteries detected"))??;
    println!("Battery info: {battery:#?}");
    let charge_to_ratio = Ratio::new::<units::ratio::ratio>(charge_to);
    if !battery.reached_ratio(charge_to_ratio) {
        println!("Starting charging");
        let mut file = File::open(CROS_EC_PATH).await?;
        set_charge_control(&mut file, ChargeControl::Normal(None))?;
        if let Some(max_charge_speed) = max_charge_speed {
            let max_power =
                battery.energy_full_design() / Time::new::<hour>(1.0 / max_charge_speed);
            let max_current = max_power / battery.voltage();
            println!("Setting max charging current: {max_current:?}. Max charge power should be {max_power:?}.");
            set_charge_current_limit(&mut file, max_current)?;
        }
        loop {
            battery.refresh()?;
            sleep(Duration::from_secs(5)).await;
            if battery.reached_ratio(charge_to_ratio) {
                break;
            }
        }
        println!("Done charging. Stopping...");
        set_charge_control(&mut file, ChargeControl::Idle)?;
    } else {
        println!("Already charged. No need to charge.")
    }
    drop(fd);
    Ok(())
}
