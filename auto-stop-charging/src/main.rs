use battery::units::{power::watt, ratio::ratio, Power, Ratio};
use ectool::charge_control::ChargeControl;
use std::{thread::sleep, time::Duration};

const THRESHOLD: f32 = 0.8;

#[tokio::main]
async fn main() -> Result<(), battery::Error> {
    let manager = battery::Manager::new()?;

    let mut battery = manager.batteries()?.next().unwrap()?;
    loop {
        // println!("{:#?}", battery);
        if battery.energy() / battery.energy_full() > Ratio::new::<ratio>(THRESHOLD) {
            match ectool::charge_control::set(ChargeControl::Idle).await {
                Ok(_) => {
                    println!("Stopped charging");
                }
                Err(e) => {
                    println!("Error stopping charging: {:#?}", e);
                }
            };
        }

        sleep(Duration::from_secs(5));
        battery.refresh()?;
    }
}
