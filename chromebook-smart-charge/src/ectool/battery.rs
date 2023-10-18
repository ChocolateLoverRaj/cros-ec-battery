use async_process::Command;
use std::{io, str::FromStr, string::FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    Utf8(FromUtf8Error),
    Io(io::Error),
    ParseInt,
    Format,
}

#[derive(Debug)]
pub struct BatteryOutputFlags {
    pub ac_present: bool,
    pub batt_present: bool,
    pub charging: bool,
    pub discharging: bool,
}

#[derive(Debug)]
pub struct BatteryOutput {
    pub oem_name: String,
    pub model_number: String,
    pub chemistry: String,
    pub serial_number: String,
    /// (mAh)
    pub design_capacity: u32,
    /// (mAh)
    pub last_full_charge: u32,
    /// (mV)
    pub design_output_voltage: u32,
    pub cycle_count: u32,
    /// (mV)
    pub present_voltage: u32,
    /// (mA)
    pub present_current: i32,
    /// (mAh)
    pub remaining_capacity: u32,
    /// (mV)
    pub desired_voltage: u32,
    /// (mA)
    pub desired_current: u32,
    pub flags: BatteryOutputFlags,
}

pub async fn battery() -> Result<BatteryOutput, Error> {
    match Command::new("ectool").arg("battery").output().await {
        Result::Ok(output) => match String::from_utf8(output.stdout) {
            Result::Ok(output) => {
                let mut output = output.lines();
                const FIRST_COLUMN_SIZE: usize = "  OEM name:               ".len();
                fn get_value_column(line: &str) -> String {
                    String::from(&line[FIRST_COLUMN_SIZE..])
                }

                fn parse_number<F: FromStr>(line: &str) -> Result<F, Error> {
                    let value_column = &get_value_column(line);
                    let number = value_column.split_once(" ");
                    let number = match number {
                        Some(number) => number.0,
                        None => value_column,
                    };
                    let number = number.parse::<F>();
                    match number {
                        Ok(number) => Ok(number),
                        Err(_) => return Err(Error::ParseInt),
                    }
                }

                match output.next() {
                    Some(_) => {}
                    None => return Err(Error::Format),
                }
                let oem_name;
                match output.next() {
                    Some(line) => oem_name = get_value_column(line),
                    None => return Err(Error::Format),
                }
                let model_number;
                match output.next() {
                    Some(line) => model_number = get_value_column(line),
                    None => return Err(Error::Format),
                }
                let chemistry;
                match output.next() {
                    Some(line) => chemistry = get_value_column(line),
                    None => return Err(Error::Format),
                }
                let serial_number;
                match output.next() {
                    Some(line) => serial_number = get_value_column(line),
                    None => return Err(Error::Format),
                }
                let design_capacity;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => design_capacity = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let last_full_charge;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => last_full_charge = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let design_output_voltage;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => design_output_voltage = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let cycle_count;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => cycle_count = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let present_voltage;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => present_voltage = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let present_current;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => present_current = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let remaining_capacity;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => remaining_capacity = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let desired_voltage;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => desired_voltage = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let desired_current;
                match output.next() {
                    Some(line) => match parse_number(line) {
                        Ok(number) => desired_current = number,
                        Err(e) => return Err(e),
                    },
                    None => return Err(Error::Format),
                }
                let flags = match output.next() {
                    Some(line) => {
                        let value = &get_value_column(line)["0x06".len()..];
                        let flags_iterator = value.split_whitespace();
                        let mut flags = BatteryOutputFlags {
                            ac_present: false,
                            batt_present: false,
                            charging: false,
                            discharging: false,
                        };
                        for flag in flags_iterator {
                            match flag {
                                "AC_PRESENT" => flags.ac_present = true,
                                "BATT_PRESENT" => flags.batt_present = true,
                                "CHARGING" => flags.charging = true,
                                "DISCHARGING" => flags.discharging = true,
                                _ => {}
                            }
                        }
                        flags
                    }
                    None => return Err(Error::Format),
                };

                Ok(BatteryOutput {
                    oem_name,
                    model_number,
                    chemistry,
                    serial_number,
                    design_capacity,
                    last_full_charge,
                    design_output_voltage,
                    cycle_count,
                    present_voltage,
                    present_current,
                    remaining_capacity,
                    desired_voltage,
                    desired_current,
                    flags,
                })
            }
            Result::Err(e) => Err(Error::Utf8(e)),
        },
        Result::Err(e) => Result::Err(Error::Io(e)),
    }
}
