use tokio::fs::read_to_string;

pub async fn is_plugged_in() -> bool {
    read_to_string("/sys/class/power_supply/AC/online")
        .await
        .unwrap()
        .trim_end()
        .parse::<u8>()
        .unwrap()
        == 1
}
