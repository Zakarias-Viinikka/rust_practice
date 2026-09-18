use actor_audition::{
    cli_gui::{Stats, run_gui},
    doers::{Doer1, Doer2, Doer3, GetPriorityLevel, MakeAPhoneCall},
    queue_manager::Queues,
    settings::Settings,
    telephone_manager::TelephoneManager,
    things_to_do::Instructions,
};
use rand::RngExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::{spawn, sync::mpsc};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<Instructions>(32);
    let (wake_tx, wake_rx) = mpsc::channel::<()>(1);

    let stats = Stats::new();
    let settings = Settings::new();
    let queues = Queues::create_queue_manager(wake_rx, stats.clone(), settings.clone());

    TelephoneManager::create_telephone_manager(rx, queues.clone(), wake_tx);

    let doer1 = Arc::new(Doer1::new(tx.clone()));
    let doer2 = Arc::new(Doer2::new(tx.clone()));
    let doer3 = Arc::new(Doer3::new(tx));

    spawn(infinite_loop(doer1));
    spawn(infinite_loop(doer2));
    spawn(infinite_loop(doer3));

    run_gui(queues, stats, settings).await
}

async fn infinite_loop<D>(doer: Arc<D>)
where
    D: GetPriorityLevel + MakeAPhoneCall + Send + Sync + 'static,
{
    loop {
        rnd_sleep().await;
        let doer = Arc::clone(&doer);
        spawn(async move {
            let _ = doer.make_phone_call().await;
        });
    }
}

async fn rnd_sleep() {
    let millis = rand::rng().random_range(0..3000);
    tokio::time::sleep(Duration::from_millis(millis)).await;
}
