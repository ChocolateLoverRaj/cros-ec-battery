use ectool::{
    battery::battery,
    charge_control::{self, ChargeControl, Sustainer},
    charge_current_limit,
    hello::hello,
};
use futures::{self, executor::block_on};

mod ectool;

async fn async_main() {
    let result = hello().await;
    match result {
        Ok(s) => println!("{s}"),
        Err(e) => panic!("{:#?}", e),
    }

    let battery_info = battery().await;
    match battery_info {
        Ok(battery_info) => {
            println!("{:#?}", battery_info)
        }
        Err(e) => panic!("{:#?}", e),
    }

    let charge_control = charge_control::get().await;
    match charge_control {
        Ok(charge_control) => {
            println!("{:#?}", charge_control)
        }
        Err(e) => panic!("{:#?}", e),
    }

    // charge_control::set(ChargeControl::Normal(Some(Sustainer { min: 80, max: 80 })))
    //     .await
    //     .unwrap();

    charge_current_limit::set(250).await.unwrap();
}

fn main() {
    block_on(async_main());
}
