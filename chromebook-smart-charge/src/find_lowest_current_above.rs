use std::time::Duration;

use crate::ectool::{battery::battery, charge_current_limit};

#[derive(Debug)]
pub struct ExperimentalLimitedCurrent {
    pub charge_current_limit: u32,
    pub experimental_current: u32,
}

/// Wait this much to check battery
const DELAY_TIME: Duration = Duration::from_secs(1);
const CHECKS: u32 = 8;
async fn try_current(current: u32) -> i32 {
    charge_current_limit::set(current).await.unwrap();
    let mut min_current = current as i32;
    for _ in 0..CHECKS {
        tokio::time::sleep(DELAY_TIME).await;
        min_current = min_current.min(battery().await.unwrap().present_current);
    }
    min_current
}

mod find_current_above {
    use crate::find_lowest_current_above::try_current;

    use super::ExperimentalLimitedCurrent;

    #[derive(Debug)]
    pub struct Output {
        pub current_above: u32,
        /// If the first attempt succeeds, this will be equal to the input
        pub last_failed_attempt: ExperimentalLimitedCurrent,
    }

    /// Returns a current equal to or greater than input current
    pub async fn find_current_above(min_current: u32) -> Output {
        const JUMP: u32 = 50;
        let mut attempt_number: u32 = 0;
        let mut highest_failed_attempt = ExperimentalLimitedCurrent {
            charge_current_limit: min_current,
            experimental_current: 0,
        };
        loop {
            let attempt = min_current + JUMP * ((2 as u32).pow(attempt_number) - 1);
            let current = try_current(attempt).await;
            let current = current.max(0) as u32;
            println!("attempt: {} current: {}", attempt, current);
            if current >= min_current {
                return Output {
                    last_failed_attempt: highest_failed_attempt,
                    current_above: current,
                };
            } else {
                highest_failed_attempt = ExperimentalLimitedCurrent {
                    charge_current_limit: attempt,
                    experimental_current: highest_failed_attempt.experimental_current.max(current),
                };
                attempt_number += 1;
            }
        }
    }
}

async fn find_lowest_current_between(
    mut min: u32,
    mut max: u32,
    max_range: u32,
) -> ExperimentalLimitedCurrent {
    loop {
        let between = (max + min) / 2;
        let current = try_current(between).await;
        let current = current.max(0) as u32;
        println!(
            "min: {} max: {} between: {} current:{}",
            min, max, between, current
        );
        if current < min {
            min = between;
            max = max;
        } else if current == min {
            return ExperimentalLimitedCurrent {
                charge_current_limit: between,
                experimental_current: current,
            };
        } else if current < between {
            min = min;
            max = current;
        } else {
            min = between;
            max = current;
        }
    }
}

pub async fn find_lowest_current_above(min_current: u32) -> u32 {
    /// Setting charge current limit to below this can result with the battery actually draining instead of charging. For now, we can just keep it as it is, unless for some reason it's better to programmatically find this in run time.
    const MIN_CURRENT: u32 = 150;
    let min_current = min_current.max(MIN_CURRENT);
    // If the min_current cannot be used, jump higher by this much and see if that works.
    let current_above = find_current_above::find_current_above(min_current).await;
    println!("{:#?}", current_above);
    const MAX_RANGE: u32 = 10;
    let lowest_current =
        if current_above.current_above - current_above.last_failed_attempt > MAX_RANGE {
            find_lowest_current_between(
                current_above.last_failed_attempt,
                current_above.current_above,
                MAX_RANGE,
            )
            .await
        } else {
            current_above.current_above
        };
    lowest_current
}
