use async_process::Command;
use std::io;

pub async fn set(max_ma: u32) -> Result<(), io::Error> {
    match Command::new("ectool")
        .arg("chargecurrentlimit")
        .arg(max_ma.to_string())
        .output()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
