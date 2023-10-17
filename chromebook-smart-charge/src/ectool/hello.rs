use async_process::Command;
use std::{io, string::FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    Utf8(FromUtf8Error),
    Io(io::Error),
}

pub async fn hello() -> Result<String, Error> {
    match Command::new("ectool").arg("hello").output().await {
        Result::Ok(output) => match String::from_utf8(output.stdout) {
            Result::Ok(s) => Ok(s),
            Result::Err(e) => Err(Error::Utf8(e)),
        },
        Result::Err(e) => Result::Err(Error::Io(e)),
    }
}
