use std::{io, num::ParseIntError, string::FromUtf8Error};

use async_process::{Command, Output};

#[derive(Debug)]
pub struct Sustainer {
    /// %
    pub min: u32,
    /// %
    pub max: u32,
}

#[derive(Debug)]
pub enum ChargeControl {
    Normal(Option<Sustainer>),
    Idle,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FromUtf8(FromUtf8Error),
    Format,
    ParseInt(ParseIntError),
}

pub async fn get() -> Result<ChargeControl, Error> {
    match Command::new("ectool").arg("chargecontrol").output().await {
        Ok(output) => {
            // TODO: parse idle mode
            match String::from_utf8(output.stdout) {
                Ok(output) => {
                    let lines = output.lines().collect::<Vec<_>>();

                    let right_side = match lines[0].split_once("=") {
                        Some(parts) => parts.1.trim(),
                        None => return Err(Error::Format),
                    };

                    let mode = match right_side.split_once(" ") {
                        Some(parts) => parts.0,
                        None => return Err(Error::Format),
                    };
                    match mode {
                        "NORMAL" => Ok(ChargeControl::Normal({
                            let right_side = match lines[1].split_once("=") {
                                Some(parts) => parts.1.trim_start(),
                                None => return Err(Error::Format),
                            };
                            let right_side_parts = match right_side.split_once(" ") {
                                Some(parts) => parts,
                                None => return Err(Error::Format),
                            };
                            let is_on = right_side_parts.0;
                            match is_on {
                                "on" => match right_side_parts.1.split_once("~") {
                                    Some(parts) => {
                                        let min_percent = parts.0.trim_end();
                                        let min = match min_percent[1..min_percent.len() - 1]
                                            .parse::<u32>()
                                        {
                                            Ok(min) => min,
                                            Err(e) => return Err(Error::ParseInt(e)),
                                        };
                                        let max_percent = parts.0.trim();
                                        let max = match max_percent[1..max_percent.len() - 1]
                                            .parse::<u32>()
                                        {
                                            Ok(min) => min,
                                            Err(e) => return Err(Error::ParseInt(e)),
                                        };

                                        Some(Sustainer { min, max })
                                    }
                                    None => return Err(Error::Format),
                                },
                                "off" => None,
                                _ => return Err(Error::Format),
                            }
                        })),
                        "IDLE" => return Ok(ChargeControl::Idle),
                        _ => return Err(Error::Format),
                    }
                }
                Err(e) => Err(Error::FromUtf8(e)),
            }
        }
        Err(e) => Err(Error::Io(e)),
    }
}

#[derive(Debug)]
pub enum SetError {
    Io(io::Error),
    Status(Output),
}
pub async fn set(charge_control: ChargeControl) -> Result<(), SetError> {
    let mut command = Command::new("ectool");
    command.arg("chargecontrol");
    match charge_control {
        ChargeControl::Normal(sustainer) => {
            command.arg("normal");
            match sustainer {
                Some(sustainer) => {
                    command.arg(sustainer.min.to_string());
                    command.arg(sustainer.max.to_string());
                }
                None => {}
            };
        }
        ChargeControl::Idle => {
            command.arg("idle");
        }
    };
    match command.output().await {
        Ok(output) => match output.status.success() {
            true => Ok(()),
            false => Err(SetError::Status(output)),
        },
        Err(e) => Err(SetError::Io(e)),
    }
}
