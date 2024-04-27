use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::{fs::read_to_string, io, process::Command};

pub async fn set_rtc_wake(time: SystemTime) -> Result<(), ()> {
    let a = Command::new("rtcwake")
        .arg(format!(
            "-t {}",
            time.duration_since(UNIX_EPOCH).unwrap().as_secs()
        ))
        .spawn()
        .unwrap()
        .wait_with_output()
        .await
        .unwrap();
    match a.status.success() {
        true => Ok(()),
        false => Err(()),
    }
}

pub async fn get_rtc_wake_time() -> Result<Option<SystemTime>, io::Error> {
    let number = read_to_string("/sys/class/rtc/rtc0/power/runtime_suspended_time")
        .await?
        .trim_end()
        .parse::<u64>()
        .unwrap();
    Ok(match number == 0 {
        true => Some(SystemTime::UNIX_EPOCH + Duration::from_secs(number)),
        false => None,
    })
}
