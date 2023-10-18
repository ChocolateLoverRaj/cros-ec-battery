use std::time::Duration;

use crate::ectool::{battery::battery, charge_current_limit};

/// Wait this much to check battery
const DELAY_TIME: Duration = Duration::from_millis(2000);
async fn try_current(current: u32) -> i32 {
    charge_current_limit::set(current).await.unwrap();
    tokio::time::sleep(DELAY_TIME).await;
    let current = battery().await.unwrap().present_current;
    current
}

mod find_current_above {
    use crate::find_lowest_current_above::try_current;

    #[derive(Debug)]
    pub struct Output {
        pub current_above: u32,
        /// If the first attempt succeeds, this will be equal to the input
        pub last_failed_attempt: u32,
    }

    /// Returns a current equal to or greater than input current
    pub async fn find_current_above(min_current: u32) -> Output {
        /// If the min_current cannot be used, jump higher by this much and see if that works.
        const JUMP: u32 = 50;
        let mut attempt_number: u32 = 0;
        let mut last_failed_attempt = min_current;
        loop {
            let attempt = min_current + JUMP * ((2 as u32).pow(attempt_number) - 1);
            let current = try_current(attempt).await;
            let current = current.max(0) as u32;
            println!("attempt: {} current: {}", attempt, current);
            if current >= min_current {
                return Output {
                    last_failed_attempt,
                    current_above: current,
                };
            } else {
                last_failed_attempt = attempt;
                attempt_number += 1;
            }
        }
    }
}

async fn find_lowest_current_between(mut min: u32, mut max: u32, max_range: u32) -> u32 {
    loop {
        let between = (max + min) / 2;
        let current = try_current(between).await;
        let current = current.max(0) as u32;
        println!(
            "min: {} max: {} between: {} current:{}",
            min, max, between, current
        );
        if ((between as i32) - (current as i32)).abs() as u32 <= max_range {
            return current;
        } else if current < min {
            if max - between <= max_range {
                return max;
            } else {
                min = between;
                max = max;
            }
        } else if current == min {
            return current;
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
