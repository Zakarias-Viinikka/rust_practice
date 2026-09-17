use actor_audition::{
    doers::{Doer1, Doer2, Doer3, GetPriorityLevel, MakeAPhoneCall},
    telephone_manager::TelephoneManager,
    things_to_do::Instructions,
};
use rand::RngExt;
use std::time::Duration;
use tokio::{spawn, sync::mpsc};

fn main() {
    let (tx, rx) = mpsc::channel::<Instructions>(32);

    spawn(async move { TelephoneManager::create_telephone_manager(rx).await });

    let doer1 = Doer1::new(tx.clone());
    let doer2 = Doer2::new(tx.clone());
    let doer3 = Doer3::new(tx);

    spawn(infinite_loop(doer1));
    spawn(infinite_loop(doer2));
    spawn(infinite_loop(doer3));
}

async fn infinite_loop(doer: impl GetPriorityLevel + MakeAPhoneCall) {
    loop {
        rnd_sleep().await;
        let phone_result = doer.make_phone_call().await.unwrap_or_default();
        println!("Phone result: {}", phone_result);
    }
}

async fn rnd_sleep() {
    let millis = rand::rng().random_range(0..3000);
    tokio::time::sleep(Duration::from_millis(millis)).await;
}
