// pub mod systemd_integration;

use std::time::Duration;

use anyhow::anyhow;
use chrono::{DateTime, Local, Timelike, Utc};
use tokio::{process::Command, time::sleep_until};
use zbus::Connection;
use zbus_systemd::login1::ManagerProxy;

// 9:05pm (electricity is more expensive from 4pm to 9pm for me)
const HOUR: u32 = 21;
const MINUTE: u32 = 5;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let output = Command::new("rtcwake")
        .arg("--date")
        .arg(format!("{HOUR}:{MINUTE}"))
        .arg("-m")
        .arg("no")
        .output()
        .await?;
    if output.status.success() {
        println!("Succcessfully set rtcwake: {output:#?}");
        // Wait until the charge time
        let start_charging_time = Local::now()
            .with_hour(HOUR)
            .ok_or(anyhow!("Hour"))?
            .with_minute(MINUTE)
            .ok_or(anyhow!("Minute"))?;
        sleep_until(start_charging_time.into());
        Ok(())
    } else {
        Err(anyhow!("Faield to set rtcwake: {output:#?}"))
    }
    // let connection = Connection::system().await?;
    // let manager = ManagerProxy::new(&connection).await?;
    // let get_fd = || async {
    //     manager
    //         .inhibit(
    //             "sleep".into(),
    //             "Scheduled Charger".into(),
    //             "Charge up to a certain energy".into(),
    //             "delay".into(),
    //         )
    //         .await
    // };
    // let fd = get_fd().await?;
    // tokio::time::sleep(Duration::from_secs(60)).await;
    // drop(fd);
    // Ok(())
}
