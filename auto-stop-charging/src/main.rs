use battery::units::{power::watt, ratio::ratio, Power, Ratio};
use chrono::{Timelike, Utc};
use ectool::charge_control::ChargeControl;
use is_plugged_in::is_plugged_in;
use rtc::{get_rtc_wake_time, set_rtc_wake};
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

mod is_plugged_in;
mod rtc;
use sysctrlcmd::SystemCommandsImpl;

const THRESHOLD: f32 = 0.8;

const CHARGE_TIME: &str = "9pm";
const CHARGE_CURRENT_LIMIT: u32 = 2000;

const CHECK_INTERVAL: Duration = Duration::from_secs(5);
/// Must be greater than `CHECK_INTERVAL`
const MIN_RTC_DELAY: Duration = Duration::from_secs(60);

#[tokio::main]
async fn main() -> Result<(), battery::Error> {
    let short_rtc_delay = Duration::from_secs(10);
    let manager = battery::Manager::new()?;

    let mut battery = manager.batteries()?.next().unwrap()?;
    let mut rtc_wake_set = false;
    loop {
        let woke_from_rtc = rtc_wake_set && get_rtc_wake_time().await.unwrap().is_none();
        if woke_from_rtc {
            rtc_wake_set = false;

            // Set to idle and then go back to sleep
            match ectool::charge_control::set(ChargeControl::Idle).await {
                Ok(_) => {
                    println!("Stopped charging");
                }
                Err(e) => {
                    println!("Error stopping charging: {:#?}", e);
                }
            };
            // Go back to sleep
            match SystemCommandsImpl::suspend() {
                Ok(()) => {
                    println!("Suspended again");
                }
                Err(e) => {
                    println!("Error suspending: {:#?}", e);
                }
            }
        }

        println!("{:#?}", is_plugged_in().await);
        let should_stop_charging =
            battery.energy() / battery.energy_full() > Ratio::new::<ratio>(THRESHOLD);
        if is_plugged_in().await {
            if should_stop_charging {
                match ectool::charge_control::set(ChargeControl::Idle).await {
                    Ok(_) => {
                        println!("Stopped charging");
                    }
                    Err(e) => {
                        println!("Error stopping charging: {:#?}", e);
                    }
                };
            } else {
                match ectool::charge_control::set(ChargeControl::Normal(None)).await {
                    Ok(_) => {
                        println!("Started charging");
                    }
                    Err(e) => {
                        println!("Error starting charging: {:#?}", e);
                    }
                };
                match ectool::charge_current_limit::set(CHARGE_CURRENT_LIMIT).await {
                    Ok(_) => {
                        println!("Set charge current limit to {}mA", CHARGE_CURRENT_LIMIT);
                    }
                    Err(e) => {
                        println!("Error setting charge current limit: {:#?}", e);
                    }
                };

                let _ = set_rtc_wake(SystemTime::now() + short_rtc_delay).await;
                rtc_wake_set = true;
                println!("Set rtc wake");

                // Utc::now()
                //     .with_hour(21)
                //     .unwrap()
                //     .with_minute(0)
                //     .unwrap()
                //     .with_second(0)
                //     .unwrap()
                //     .with_nanosecond(0)
                //     .unwrap()
                //     .timestamp_millis();
            }
        }
        // The energy rate check is to avoid repeatedly setting charge control to idle
        // if battery.energy() / battery.energy_full() > Ratio::new::<ratio>(THRESHOLD)
        // {
        //     match ectool::charge_control::set(ChargeControl::Idle).await {
        //         Ok(_) => {
        //             println!("Stopped charging");
        //         }
        //         Err(e) => {
        //             println!("Error stopping charging: {:#?}", e);
        //         }
        //     };
        // } else {
        //     let charge_time =
        // }

        sleep(CHECK_INTERVAL);
        battery.refresh()?;
    }
}
